use failure::Error;

mod elements;

#[cfg(test)]
#[macro_use]
mod test_crate;

mod scanner; use crate::scanner::Scanner;

pub fn scan(code : &str) -> Result<Scanner,Error> {
    let scanner = Scanner::init(code).scan()?;
    Ok(scanner)
}