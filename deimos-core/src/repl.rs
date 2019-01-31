use crate::elements::Statement;
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
}
