use ariadne::{sources, Color, Label, Report, ReportKind};
use chumsky::{input::Input, Parser};
use clap::Parser as CLParser;
use color_eyre::eyre;
use rcc::{codegen::Codegen, lexer::lexer, parser::parser};
use std::{fs, path::PathBuf};

#[derive(CLParser)]
#[command(author, version, about)]
struct Args {
    /// Source File
    file: PathBuf,

    #[arg(short, long)]
    output: PathBuf,

    #[arg(long, default_value_t = false)]
    print_ast: bool,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let src = fs::read_to_string(&args.file).expect("unable to read input file");

    let (tokens, lex_errs) = lexer().parse(&src).into_output_errors();

    let parse_errs = match &tokens {
        Some(tokens) => {
            let (ast, parse_errs) = parser()
                .map_with(|ast, e| (ast, e.span()))
                .parse(tokens.as_slice().spanned((src.len()..src.len()).into()))
                .into_output_errors();

            if let Some((stmts, _)) = ast {
                fs::write(args.output, stmts.code_gen(&mut 0))
                    .expect("failed to write to output file");
            };

            parse_errs
        }
        None => Vec::new(),
    };

    let file_name = args.file.to_string_lossy().to_string();
    lex_errs
        .into_iter()
        .map(|e| e.map_token(|t| t.to_string()))
        .chain(
            parse_errs
                .into_iter()
                .map(|e| e.map_token(|t| t.to_string())),
        )
        .for_each(|e| {
            Report::build(ReportKind::Error, file_name.clone(), e.span().start)
                .with_message(e.to_string())
                .with_label(
                    Label::new((file_name.clone(), e.span().into_range()))
                        .with_message(e.reason().to_string())
                        .with_color(Color::Red),
                )
                .with_labels(e.contexts().map(|(label, span)| {
                    Label::new((file_name.clone(), span.into_range()))
                        .with_message(format!("while parsing this {}", label))
                        .with_color(Color::Yellow)
                }))
                .finish()
                .print(sources([(file_name.clone(), src.clone())]))
                .expect("unable to print errors")
        });

    Ok(())
}
