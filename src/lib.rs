use failure::Error;

mod elements;

#[cfg(test)]
#[macro_use] 
mod test_macros;

mod scanner; use crate::scanner::Scanner;
mod parser; use crate::parser::Parser;

pub fn scan(code : &str) -> Result<Scanner,Error> {
    let scanner = Scanner::init(code).scan()?;
    Ok(scanner)
}

mod tests {

    #[test]
    fn create_scanner() {
        let code = "5+5";

        match crate::scan(&code) {
            Err(error) => panic!("{}",error),
            Ok(scanner) => {
                assert_eq!(scanner.code(),code);
            }
        }
    }
}