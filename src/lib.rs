#[macro_use] extern crate failure;
use failure::Error;

mod grammar;
mod token;
mod tokentype;
mod codeslice;
mod scanner; use scanner::Scanner;
mod tree; use tree::Tree;

pub fn parse(code : &str) {
    match Scanner::new(code).scan() {
        Err(error) => { println!("ERROR : {}",error); assert!(false); },
        Ok(scanner) => {
            match Tree::from_scanner(scanner) {
                Err(error)  => { println!("ERROR : {}",error); assert!(false); },
                Ok(tree) => {

                }
            }
        }
    }
}