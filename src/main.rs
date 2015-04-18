#![feature(plugin)]
#![feature(box_syntax)]
#![feature(io)]

#![feature(plugin)]
#![plugin(phf_macros)]
extern crate phf;

#[cfg(not(test))]
use parser::Parser;

mod lexer;
mod error;
mod parser;
mod ast;

#[cfg(not(test))]
fn main() {
    if let Some(file) = std::env::args().nth(1) {
        match Parser::from_file(&file[..]) {
            Ok(mut parser) => {
                match parser.parse() {
                    Ok(prog) => println!("prog = {:?}", prog),
                    Err(err) => println!("{}", err),
                }
            }

            Err(_) => println!("could not open file {}!", file)
        }

    } else {
        println!("no file given");
    }
}
