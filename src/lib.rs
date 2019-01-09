use failure::Error;

#[macro_use] mod grammar;
mod token;
mod tokentype;
mod codeslice;
mod chunk;
mod scanner; use crate::scanner::Scanner;
mod tree; use crate::tree::Tree;

pub fn parse(code : &str) -> Result<(),Error> {
    let scanner = Scanner::new(code).scan()?;
    let _tree = Tree::from_scanner(scanner)?.create_tree()?;
    Ok(())
}