mod enums; pub use enums::value::Value;
mod structs;
mod parser; pub use parser::Parser;

#[macro_use] extern crate failure;
use failure::Error;

pub fn parse(code : &str) -> Result<Value,Error> {
    let mut parser = Parser::new(code);
    parser.eval()
}