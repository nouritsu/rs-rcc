use chumsky::{
    input::{Input, Stream},
    Parser,
};
use clap::Parser as CLParser;
use logos::Logos;
use rs_rcc::{codegen::Codegen, parser::parser, Token};
use std::{fs, path::PathBuf};

#[derive(CLParser)]
#[command(author, version, about)]
struct Args {
    /// Source File
    file: PathBuf,

    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    let args = Args::parse();

    match fs::read_to_string(&args.file) {
        Ok(src) => {
            // Lexer
            let tokens = Token::lexer(&src);

            let token_iter = tokens.spanned().map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(_) => panic!("lexer Error"),
            });

            let token_stream = Stream::from_iter(token_iter).spanned((src.len()..src.len()).into());

            // Parser
            match parser().parse(token_stream).into_result() {
                Ok(ast) => fs::write(
                    args.output,
                    ast.into_iter()
                        .map(|stmt| stmt.code_gen())
                        .fold(String::new(), |s, x| s + &x),
                )
                .expect("failed to write to output file"),
                Err(err) => {
                    err.into_iter().for_each(|err| println!("{:?}", err));
                }
            }
        }

        Err(err) => {
            panic!("{}", err);
        }
    }
}
