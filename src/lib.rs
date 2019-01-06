#[macro_use] extern crate failure;
use failure::Error;

mod grammar;
mod token;
mod tokentype;
mod codeslice;
mod scanner; use scanner::Scanner;
mod tree; use tree::Tree;

pub fn parse(code : &str) -> Result<(),Error> {
    let scanner = Scanner::new(code).scan()?;
    let tree = Tree::from_scanner(scanner)?.create_tree()?;
    Ok(())
}