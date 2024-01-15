use clap::{error::Result, Parser as CLParser};
use logos::Logos;
use rs_rcc::Token;
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
            let res = Token::lexer(&src).collect::<Result<Vec<Token>, ()>>();
            println!("{:?}", res);
        }

        Err(err) => {
            panic!("{}", err);
        }
    }
}
