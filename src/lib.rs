use failure::Error;

#[macro_use] pub mod grammar;
pub mod token;
pub mod tokentype;
mod codeslice;
pub mod chunk;
mod scanner; use crate::scanner::Scanner;
pub mod tree; use crate::tree::Tree;

pub fn parse(code : &str) -> Result<Tree,Error> {
    let scanner = Scanner::new(code).scan()?;
    let tree = Tree::from_scanner(scanner)?.create_tree()?;
    Ok(tree)
}