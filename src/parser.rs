use failure::{ Error, format_err };

use crate::scanner::Scanner;
use crate::elements::Chunk;
use crate::elements::Statement;
use crate::elements::TokenType;

pub struct Parser<'a> {
    raw_code : &'a str,
    chunks : Vec<Chunk>,
}

impl<'a> Parser<'a> {
    pub fn from_scanner(scanner : Scanner <'a>) -> Result<Parser,Error> {
        /// creates a completed parser from a scanner, this means all the tokens
        /// are grouped and organized into chunks and ready to execute, or process
        
        // prepares the scanner parts
        let (raw_code, tokens) = scanner.disassemble();
        let mut raw_statements = Statement::tokens_to_statements(tokens);

        // working parts
        let mut chunks : Vec<Chunk> = Vec::new();
        let mut working_statements : Vec<Statement> = Vec::new();
        let mut statement : Vec<Statement> = Vec::new();

        loop {

            if raw_statements.len() <= 0 { 
                if statement.len() > 0 {
                    working_statements.push(Parser::collapse_statement(statement)); // crunch the last one
                    chunks.push(Chunk::new(working_statements));
                }
                break; 
            }

            let token = raw_statements.remove(0);
            match token.as_token_type() {
                TokenType::EOL => {
                    working_statements.push(Parser::collapse_statement(statement));
                    statement = Vec::new();
                },
                _ => {
                    statement.push(token)
                },
            }
        }

        Ok(Parser {
            raw_code : raw_code,
            chunks : chunks,
        })
    }

    fn collapse_statement(mut statement : Vec<Statement>) -> Statement {
        
        loop {
            if statement.len() <= 1 { break; }

            let current = statement.remove(0);
            
            if current.is_unop() {
                if let Some(expr) = Parser::peek_expr(&mut statement) {
                    statement.insert(0,current.into_unary(expr));
                }
            }
            
        }
        statement.remove(0)
    }

    fn peek_expr(statement : &mut Vec<Statement>) -> Option<Statement> {
        if statement.len() <= 0 { return None; }

        match statement[0].is_expr() {
            true => Some(statement.remove(0)),
            false => None,
        }
    }


}

mod tests {

    #[test]
    fn simpe() {
        use crate::scanner::Scanner;
        use crate::parser::Parser;

        let scanner = Scanner::init("-5").scan().unwrap();
        let parser = Parser::from_scanner(scanner).unwrap();

        assert_eq!(1,parser.chunks.len());
        assert_eq!(chunk!(unary!("-","5")),parser.chunks[0]);
    }
}