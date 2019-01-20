#[cfg(test)]
#[macro_use] mod test_crate;

use failure::Error;

mod elements;
mod scanner; use crate::scanner::Scanner;
mod parser;

pub fn scan(code : &str) -> Result<Scanner,Error> {
    let scanner = Scanner::init(code).scan()?;
    Ok(scanner)
}