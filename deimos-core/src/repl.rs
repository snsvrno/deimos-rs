use crate::elements::{ Statement, Token, TokenType };
use crate::scanner::Scanner;
use crate::parser::Parser;
use crate::elements::Scope;

use failure::{Error,format_err};

pub struct Repl {
    code : String,
    statements : Vec<Statement>,
    scope : Scope,
}

impl Repl {
    pub fn new() -> Repl {
        Repl {
            code : String::new(),
            statements : Vec::new(),
            scope : Scope::new(),
        }
    }

    pub fn add(&mut self, code : &str) -> Result<Statement,Error> {
        // adds the code 
        self.code.push_str(code);

        let scanner = Scanner::init(code).scan()?;
        let parser = Parser::from_scanner(scanner)?;

        let mut result = Statement::Empty;

        let (_code,chunks) = parser.disassemble();
        for chunk in chunks.iter() { 
            result = chunk.eval(&mut self.scope)?;
        }

        Ok(result)
    }

    pub fn check_for_complete_statement(text : &str) -> Result<bool,Error> {
        //! makes s token stream out of a text string, and then checks to see
        //! if it is a complete statement or not.
        //! 
        //! designed to allow multi line input in a repl like envirnoment
        
        let mut scanner = Scanner::init(text).scan()?;
        let mut depth = 0;
        
        loop {
            match scanner.token() {
                None => break,
                Some(token) => {
                    match token.get_type() {
                        TokenType::Function |
                        TokenType::Do => depth += 1,

                        TokenType::End => depth -= 1,
                        _ => (),
                    }
                }
            }
        }

        if depth == 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
