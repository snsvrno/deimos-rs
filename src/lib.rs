mod enums;
mod structs;
mod parser; use parser::Parser;

#[macro_use] extern crate failure;
use failure::Error;

pub fn parse(code : &str) -> Result<i32,Error> {
    let mut parser = Parser::new(code);
    parser.build_tree()?;
    parser.eval()
}