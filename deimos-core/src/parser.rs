use failure::{ Error, format_err };

use crate::scanner::Scanner;
use crate::elements::Chunk;
use crate::elements::Statement;
use crate::elements::TokenType;

enum InternalError {
    Syntax(usize,usize,usize,usize),
    SyntaxMsg(String,usize,usize,usize,usize),
    General(String),
}

impl InternalError {
    fn render(self, raw_code : &str) -> String {
        match self {
            InternalError::General(string) => format!("{}",string),
            InternalError::Syntax(start,end,line,col) => format!("Syntax error parsing code:\n      {}\n      ^ line: {} col: {}",
                &raw_code[start .. end],
                line,
                col
            ),
            InternalError::SyntaxMsg(prefix,start,end,line,col) => format!("{}:\n      {}\n      ^ line: {} col: {}",
                prefix,
                &raw_code[start .. end],
                line,
                col
            ),
        }
    }
}

enum Option3<T> {
    Some(T),
    None,
    Skip,
}

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
        let raw_statements = Statement::tokens_to_statements(tokens);
        // creates the empty chunk object needed for the returning struct.
        let mut chunks : Vec<Chunk> = Vec::new();

        // TODO : need to actually process the chunks, make different chunks
        match Parser::process_statements(raw_statements) {
            // error handling
            Err(error) => Err(format_err!("{}",error.render(&raw_code))),
            Ok(processed_statements) => {
                chunks.push(Chunk::new(processed_statements));

                Ok(Parser {
                    raw_code : raw_code,
                    chunks : chunks,
                })
            },
        }
    }

    fn process_statements(mut raw_statements : Vec<Statement>) -> Result<Vec<Statement>,InternalError> {
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

            let resulting_statement : Option3<Statement> = match token.as_token_type() {
                TokenType::EOF |
                TokenType::EOL => {
                    if statement.len() > 0 {
                        let stat = Parser::collapse_statement(statement)?;
                        statement = Vec::new();
                        Option3::Some(stat)
                    } else {
                        Option3::Skip
                    }
                },
                TokenType::Do => {
                    let do_statement = Parser::collapse_block_statement(&mut raw_statements,TokenType::Do)?;
                    Option3::Some(do_statement)
                },
                TokenType::While => {
                    let while_statement = Parser::collapse_block_statement(&mut raw_statements, TokenType::While)?;
                    Option3::Some(while_statement)
                },
                TokenType::Equal => {
                    match Parser::contains_token(&statement, TokenType::Local) {
                        true => Option3::None, // TODO : implement local assignment
                        false => {
                            match Parser::consume_until_token(&mut raw_statements, TokenType::EOL, false) {
                                Err((start,end,line,col)) => return Err(InternalError::SyntaxMsg("Failed to find EOL".to_string(),
                                    start,end,line,col
                                )),
                                Ok(exprs) => {
                                    // splits the vars by ',' and the collapses each piece.
                                    let var_list = {
                                        let mut list : Vec<Statement> = Vec::new();
                                        let splits_list = Parser::split_by_token(statement, TokenType::Comma);
                                        for split in splits_list {
                                            let stat : Statement = Parser::collapse_statement(split)?;
                                            list.push(stat);
                                        }
                                        list
                                    };
                                    // splits the expressions by ',' and the collapses each piece.
                                    let expr_list = {
                                        let mut list : Vec<Statement> = Vec::new();
                                        let splits_list = Parser::split_by_token(exprs, TokenType::Comma);
                                        for split in splits_list {
                                            let stat : Statement = Parser::collapse_statement(split)?;
                                            list.push(stat);
                                        }
                                        list
                                    };

                                    let assignment = Statement::create_assignment(var_list,expr_list);
                                    statement = Vec::new();
                                    Option3::Some(assignment)
                                },
                            }
                        } 
                    }
                }
                _ => Option3::None,
            };

            match resulting_statement {
                Option3::Skip => (),
                Option3::None => { statement.push(token); },
                Option3::Some(stat) => {
                    working_statements.push(stat);
                },
            }

        }
    }

    fn collapse_block_statement(mut raw_statements : &mut Vec<Statement>, starter : TokenType) -> Result<Statement,InternalError> {
        //! takes block statements, like do-end and if-then-end, and processes them. works by
        //! finding the correct beginning and end of these blocks and then processing the insides.
        //! this should allow everything to be processed correctly (even with multiple nestings).
        
        let mut loop_looker = 1;
        let mut working_statement : Vec<Statement> = Vec::new();

        // looks for any prestatments to the actual block, so like while .. do .. end:
        //      
        //            |<------>| this stuff here          
        //      while value <= 3 do
        //          ...
        //      end
        //
        let pre_expr : Option<Statement> = match starter {
            TokenType::While => {
                match Parser::consume_until_token(&mut raw_statements, TokenType::Do, false) {
                    Err((start,end,line,col)) => return Err(InternalError::SyntaxMsg("Couldn't find the `do` in the `while .. do .. end`".to_string(),
                        start,end,line,col
                    )),
                    Ok(pre_tokens) => {
                        let expr = Parser::collapse_statement(pre_tokens)?;
                        Some(expr)
                    },
                }
            },
            _ => None,
        };

        loop {
            if raw_statements.len() <= 0 { return Err(InternalError::General("Can't find the end of the statement!".to_string())); }
                        
            let loop_token = raw_statements.remove(0);
            Statement::counting_loops(&loop_token, &mut loop_looker);

            match loop_looker == 0 {
                true => {
                    let processed_statements = Parser::process_statements(working_statement)?;
                    let mut insides : Vec<Box<Statement>> = Vec::new();
                    for p in processed_statements {
                        insides.push(Box::new(p));
                    }

                    return match starter {
                        TokenType::Do => Ok(Statement::DoEnd(insides)),
                        TokenType::While => { 
                            match pre_expr {
                                None => Err(InternalError::General("while .. do .. end has no expression?!?, Impossible!".to_string())),
                                Some(expr) => {
                                    Ok(Statement::WhileDoEnd(Box::new(expr),insides)) 
                                }
                            }
                        },
                        _ => Err(InternalError::General("FALT".to_string())),
                    };
                },
                false => {
                    working_statement.push(loop_token);
                }
            }

        }
    }

    fn collapse_statement(mut statement : Vec<Statement>) -> Result<Statement,InternalError> {
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
        match statement.len() {
            0 => Err(InternalError::General("Statement is empty?".to_string())),
            1 => Ok(statement.remove(0)),
            _ => { 
                let (line,col) = statement[0].get_code_display_info();
                Err(InternalError::Syntax(
                    statement[0].get_code_start(),
                    statement[statement.len()-1].get_code_end(),
                    line,col
                )) 
            },
        }
    }

    fn peek_expr_before(pos : usize,statement : &Vec<Statement>) -> bool {
        if statement.len() < pos || pos == 0 { return false; }
        statement[pos-1].is_expr()
    }

    fn peek_expr_after(pos : usize,statement : &Vec<Statement>) -> bool {
        if statement.len() < (pos+1) { return false; }
        statement[pos+1].is_expr()
    }

    fn consume_until_token(buffer : &mut Vec<Statement>, desired_token : TokenType, include : bool) -> Result<Vec<Statement>,(usize,usize,usize,usize)> {
        let mut tokens : Vec<Statement> = Vec::new();
        loop {
            if buffer.len() <= 0 { 
                let (line,col) = tokens[0].get_code_display_info();
                return Err((
                    tokens[0].get_code_start(),
                    tokens[tokens.len()-1].get_code_end(),
                    line,col
                )); 
            }

            let token = buffer.remove(0);
            
            // checks if the token is the desired token, but if the desired token is EOL then it will also match on EOF token).
            if token.as_token_type() == &desired_token || (&desired_token == &TokenType::EOL && token.as_token_type() == &TokenType::EOF) {
                if include { tokens.push(token); }
                return Ok(tokens);
            } else {
                tokens.push(token);
            }
        }
    }

    fn contains_token(tokens : &Vec<Statement>, token_to_look_for : TokenType) -> bool {
        //!
        //! 
        //! should only be used for Statement::Token(_) so shouldn't panic.
        
        for t in tokens.iter() {
            if t.as_token_type() == &token_to_look_for { return true; }
        }
        false
    }

    fn split_by_token(mut tokens : Vec<Statement>, splitter : TokenType) -> Vec<Vec<Statement>> {
        let mut splits : Vec<Vec<Statement>> = Vec::new();

        let mut working : Vec<Statement> = Vec::new();
        loop {
            if tokens.len() <= 0 { break; }

            let token = tokens.remove(0);
            if token.as_token_type() == &splitter {
                splits.push(working);
                working = Vec::new();
            } else {
                working.push(token);
            }
        }

        if working.len() > 0 {
            splits.push(working);
        }

        splits
    }

}

#[cfg(test)]
mod tests {

    use crate::test_crate::*;

    #[test]
    fn unary_simple() {
        assert_eq!(setup_simple!("-5").chunks[0],
            chunk!(unary!("-","5")));
    }

    #[test]
    fn syntax_errors() {
        let raw_code = r"
        5 + 5 - 5 * 3
        do
            225 + 523
        end
        5 + + 5
        ";
        let parser = setup_error!(raw_code);

        match parser {
            Err(error) => {
                let manufacterd_error_msg = crate::parser::InternalError::Syntax(76,83,6,9);
                assert_err!(error,manufacterd_error_msg.render(raw_code));
                println!("{}",error)
            },
            Ok(_) => panic!("Should have failed"),
        }
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
        
        assert_eq!(setup_simple!("while true do 5+4 end").chunks[0],
            chunk!(while_do_end!("true",
                binary!("+","5","4")
            )));
    }

    #[test]
    fn assignment_simple() {

        // single assignment
        assert_eq!(setup_simple!("bob = 5 + 4").chunks[0],
            chunk!(assignment!(
                list!( statement!("bob") ),
                list!( binary!("+","5","4") )
            )));

        // double assignment
        assert_eq!(setup_simple!("bob,linda = 5,4").chunks[0],
            chunk!(assignment!(
                list!( statement!("bob"),statement!("linda") ),
                list!( statement!("5"),statement!("4") )
            )));

        // mismatched assignment
        assert_eq!(setup_simple!("bob,linda,jorge = 5 * 4 + 3,false").chunks[0],
            chunk!(assignment!(
                list!( statement!("bob"),statement!("linda"),statement!("jorge") ),
                list!( 
                    binary!("+",
                        s binary!("*","5","4"),
                        "3"),
                    statement!("false"),
                    empty!()
                )
            )));
    }


    #[test]
    #[ignore]
    fn loops_complex() {
        // TODO : figure out a better way to pair the code with the macro function, 
        // maybe an external crate that contains all of it?

        let code = load_file("loops_complex/do");
        let parser = setup!(&code);

        let check_against = chunk!(do_end!(
            binary!("+",
                "5",
                s binary!("*","4","3")
            ),
            do_end!(
                binary!("+","1","2")
            )
        ));

        assert_eq!(parser.chunks[0],check_against);
    }
}
