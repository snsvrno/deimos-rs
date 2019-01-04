#[macro_use] extern crate failure;
use failure::Error;

mod token;
mod tokentype;
mod codeslice;
mod scanner; use scanner::Scanner;

pub fn parse(code : &str) {
    match Scanner::new(code).scan() {
        Err(error) => { println!("ERROR : {}",error); assert!(false); },
        Ok(scanner) => {

        }
    }
}