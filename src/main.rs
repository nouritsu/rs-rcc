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
}

fn main() {
    let args = Args::parse();

    match fs::read_to_string(&args.file) {
        Ok(src) => {
            let tokens = Token::lexer(&src).spanned().map(|(tok, span)| match tok {
                Ok(tok) => (tok, span.into()),
                Err(_) => panic!("Lexer Error"),
            });

            let token_stream = Stream::from_iter(tokens).spanned((src.len()..src.len()).into());

            match parser().parse(token_stream).into_result() {
                Ok(ast) => ast
                    .into_iter()
                    .map(|stmt| stmt.code_gen())
                    .for_each(|s| println!("{s}")),
                Err(err) => err.into_iter().for_each(|err| println!("{:?}", err)),
            }
        }

        Err(err) => {
            panic!("{}", err);
        }
    }
}
