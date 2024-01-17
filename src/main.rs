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

    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    match fs::read_to_string(&args.file) {
        Ok(src) => {
            // Lexer
            let tokens = Token::lexer(&src);

            if args.debug {
                println!(
                    "Lexed:\n{:?}\n",
                    tokens.clone().collect::<Result<Vec<Token>, ()>>()
                );
            }

            let token_iter = tokens.spanned().map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(_) => panic!("Lexer Error"),
            });

            let token_stream = Stream::from_iter(token_iter).spanned((src.len()..src.len()).into());

            // Parser
            match parser().parse(token_stream).into_result() {
                Ok(ast) => {
                    if args.debug {
                        println!("Parsed:\n{:?}\n", ast);
                        println!("Assembly:");
                    }

                    ast.into_iter()
                        .map(|stmt| stmt.code_gen())
                        .for_each(|s| println!("{s}"));
                }
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
