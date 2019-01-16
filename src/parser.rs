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
        
        let (raw_code, tokens) = scanner.disassemble();
        // converts the vec<Token> to a vec<Statement> for processessing
        let mut raw_statements = Statement::tokens_to_statements(tokens);

        let mut chunks : Vec<Chunk> = Vec::new();
        // the group of statements that will become the 'chunks'
        let mut working_statements : Vec<Statement> = Vec::new();
        // the current statement, starts off as a vec<Statement> of all the tokens and is gradually
        // compressed down to a single statement
        let mut statement : Vec<Statement> = Vec::new();

        loop {

            if raw_statements.len() <= 0 { 
                if statement.len() > 0 {
                    // crunch the last one, if still exists
                    working_statements.push(Parser::collapse_statement(statement));
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
        /// takes a list of Statements that can be collapsed down to a new statement.
        /// primarily used for taking a list of Tokens and making a single statement
        /// from them
        ///
        ///     vec<"5","+","3","*","3" => binary("+","5",binary("*","3","3"))

        let mut pos = 0;

        loop {
            // already a single statement, stop the loop
            if statement.len() <= 1 || statement.len() <= pos { break; }

            // checks if current statement is an unary operator, so it can then
            // check if we can make a unary grouping
            if statement[pos].is_unop() {
                println!("Found u {}",statement[pos]);
                if Parser::peek_expr_after(pos,&statement) && !Parser::peek_expr_before(pos,&statement) {
                    let expr = statement.remove(pos+1);
                    let op = statement.remove(pos);

                    statement.insert(pos,op.into_unary(expr));

                    pos = 0;
                    continue;
                }
            }

            // checks if current statement is a binary operator
            if statement[pos].is_binop() {
                println!("Found b {}",statement[pos]);
                if Parser::peek_expr_before(pos,&statement) && Parser::peek_expr_after(pos,&statement) {
                    let expr2 = statement.remove(pos+1);
                    let op = statement.remove(pos);
                    let expr1 = statement.remove(pos-1);
                        
                    statement.insert(pos-1,op.into_binary(expr1,expr2));

                    pos = 0;
                    continue;
                }
            }
            

            pos += 1;
        }

        // removes the first element of the list of statements,
        // will panic tread if there is nothing in the statement
        statement.remove(0)
    }

    fn peek_expr_before(pos : usize,statement : &Vec<Statement>) -> bool {
        if statement.len() < pos || pos == 0 { return false; }
        statement[pos-1].is_expr()
    }

    fn peek_expr_after(pos : usize,statement : &Vec<Statement>) -> bool {
        if statement.len() < (pos+1) { return false; }
        statement[pos+1].is_expr()
    }

}

mod tests {

    #[test]
    fn simple_unary() {
        use crate::scanner::Scanner;
        use crate::parser::Parser;

        let scanner = Scanner::init("-5").scan().unwrap();
        let parser = Parser::from_scanner(scanner).unwrap();

        assert_eq!(1,parser.chunks.len());
        assert_eq!(chunk!(unary!("-","5")), parser.chunks[0]);
    }

    #[test]
    fn simple_binary() {
        let parser = setup_simple!("5+4");

        assert_eq!(1,parser.chunks.len());
        assert_eq!(chunk!(binary!("+","5","4")), parser.chunks[0]);
    
        let parser2 = setup_simple!("5+4-3");

        assert_eq!(1,parser.chunks.len());
        assert_eq!(chunk!(binary!("-",s binary!("+","5","4"),"3")), parser2.chunks[0]);
    }
}
