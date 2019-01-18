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
        //! creates a completed parser from a scanner, this means all the tokens
        //! are grouped and organized into chunks and ready to execute, or process
        
        let (raw_code, tokens) = scanner.disassemble();
        // converts the vec<Token> to a vec<Statement> for processessing
        let mut raw_statements = Statement::tokens_to_statements(tokens);
        // creates the empty chunk object needed for the returning struct.
        let mut chunks : Vec<Chunk> = Vec::new();

        // TODO : need to actually process the chunks, make different chunks
        let processed_statements = Parser::process_statements(raw_statements)?;
        chunks.push(Chunk::new(processed_statements));

        Ok(Parser {
            raw_code : raw_code,
            chunks : chunks,
        })
    }

    fn process_statements(mut raw_statements : Vec<Statement>) -> Result<Vec<Statement>,Error> {
        //! the meat and potatos of the parsing, used to split lines and form the statement tokens
        //! into real statements.

        // the current statement, starts off as a vec<Statement> of all the tokens and is gradually
        // compressed down to a single statement
        let mut statement : Vec<Statement> = Vec::new();
        // the group of statements that will become the returned value
        let mut working_statements : Vec<Statement> = Vec::new();
        
        loop {
            // when `raw_statements` is completely consumed.
            if raw_statements.len() <= 0 {
                if statement.len() > 0 {
                    working_statements.push(Parser::collapse_statement(statement)?);
                }

                return Ok(working_statements);
            }

            let token = raw_statements.remove(0);
            match token.as_token_type() {
                TokenType::EOL => {
                    working_statements.push(Parser::collapse_statement(statement)?);
                    statement = Vec::new();
                },
                TokenType::Do => {
                    let do_statement = Parser::collapse_block_statement(&mut raw_statements,TokenType::Do)?;
                    working_statements.push(do_statement);
                }
                _ => {
                    statement.push(token)
                },

            }

        }
    }

    fn collapse_block_statement(raw_statements : &mut Vec<Statement>, starter : TokenType) -> Result<Statement,Error> {
        //! takes block statements, like do-end and if-then-end, and processes them. works by
        //! finding the correct beginning and end of these blocks and then processing the insides.
        //! this should allow everything to be processed correctly (even with multiple nestings).
        
        // TODO : implement the other kinds of loops here, only works for Do-end and is hard coded
        // for it.
        
        let mut loop_looker = 1;
        let mut working_statement : Vec<Statement> = Vec::new();

        loop {
            if raw_statements.len() <= 0 { return Err(format_err!("Can't find the end of the statement!")); }
                        
            let loop_token = raw_statements.remove(0);
            Statement::counting_loops(&loop_token, &mut loop_looker);

            match loop_looker == 0 {
                true => {
                    let processed_statements = Parser::process_statements(working_statement)?;
                    let mut insides : Vec<Box<Statement>> = Vec::new();
                    for p in processed_statements {
                        insides.push(Box::new(p));
                    }
                    return Ok(Statement::DoEnd(insides));
                },
                false => {
                    working_statement.push(loop_token);
                }
            }

        }
    }

    fn collapse_statement(mut statement : Vec<Statement>) -> Result<Statement,Error> {
        //! takes a list of Statements that can be collapsed down to a new statement.
        //! primarily used for taking a list of Tokens and making a single statement
        //! from them.
        //!
        //! only used for single line statements, anything that is multlined should
        //! be handled above this one.
        //!
        //! ```test
        //!
        //!     vec<"5","+","3","*","3" => binary("+","5",binary("*","3","3"))
        //!
        //! ```

        let mut pos = 0;

        loop {
            // already a single statement, stop the loop
            if statement.len() <= 1 || statement.len() <= pos { break; }

            // checks if current statement is an unary operator, so it can then
            // check if we can make a unary grouping
            if statement[pos].is_unop() {
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
        Ok(statement.remove(0))
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
    fn unary_simple() {
        assert_eq!(setup_simple!("-5").chunks[0],
            chunk!(unary!("-","5")));
    }

    #[test]
    fn binary_simple() {
        assert_eq!(setup_simple!("5+4").chunks[0],
            chunk!(binary!("+","5","4")));

        assert_eq!(setup_simple!("5+4-3").chunks[0],
            chunk!(binary!("-",s binary!("+","5","4"),"3")));

        assert_eq!(setup_simple!("5+4*3").chunks[0],
            chunk!(binary!("+","5",s binary!("*","4","3"))));

        assert_eq!(setup_simple!("50 == 4 and 3 <= 10 or true").chunks[0],
            chunk!(binary!("or",
                s binary!("and",
                    s binary!("==","50","4"),
                    s binary!("<=","3","10")
                ),
                "true" ))
            );

    }

    #[test]
    fn loops_simple() {
        assert_eq!(setup_simple!("do 5+4 end").chunks[0],
            chunk!(do_end!(
                binary!("+","5","4")
            )));
    }

}
