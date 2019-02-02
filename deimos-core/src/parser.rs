use crate::elements::{Chunk, Statement, TokenType};
use crate::scanner::Scanner;

use failure::{Error,format_err};

pub struct Parser<'a> {
    raw_code : &'a str,
    chunks : Vec<Chunk>
}

impl<'a> Parser<'a> {
    
    pub fn from_scanner(scanner : Scanner <'a>) -> Result<Parser,Error> {
        /// creates a parser from a scanner.
        ///
        /// takes all the scanned tokens and organizes them into a tree 
        /// that represents the code.

        // explodes the scanner object and gets the pieces that matter
        let (raw_code, tokens) = scanner.disassemble();
        // converts the tokens into Statements so they can be processessed
        let mut raw_statements = Statement::tokens_to_statements(tokens);
        // empty chunks for processesing
        let mut chunks : Vec<Chunk> = Vec::new();

        
        let mut working_phrase : Vec<Statement> = Vec::new();
        let mut current_chunk : Chunk = Chunk::empty();

        loop {
            // if there are no more tokens left in the raw_statements then
            // we are finished processing tokens / statements
            if raw_statements.len() <= 0 { break; }

            // gets the current token
            let token = raw_statements.remove(0);

            // this works because all `statements` in the raw_statement are just
            // wrapped tokens, so this function should never fail
            //
            // this section is creating the chunks. it should only be processessing
            // things that would cause a creation of a different chunk, like `do-end`
            // `if-else-end`, etc ... things that are different loops and scopes
            match token.as_token_type() {
                TokenType::EOF |
                TokenType::EOL => {
                    if working_phrase.len() > 0 {
                        let state = Parser::collapse_statement(working_phrase)?;
                        current_chunk.add(state);
                        // creating new objects that we just used.
                        working_phrase = Vec::new();
                    }
                }, 
                _ => working_phrase.push(token),
            }
        }

        // adds the dangling chunk if not empty
        if !current_chunk.is_empty() { chunks.push(current_chunk); }

        Ok(Parser {
            raw_code : raw_code,
            chunks : chunks,
        })
        
    }

    pub fn disassemble(self) -> (&'a str, Vec<Chunk>) {
        (self.raw_code,self.chunks)
    }

    fn collapse_statement(mut working_phrase : Vec<Statement>) -> Result<Statement,Error> {
        //! takes a list of `Statement` that should be able to be collapsed down into 
        //! a single new statement.
        //!
        //! like `x = 5 + 4` can be representated as a single statement
        //!
        //! ```text
        //!
        //!     vec<"5","+","3","*","3" => binary("+","5",binary("*","3","3"))
        //!
        //! ```
        let mut pos = 0;

        loop {
            if pos >= working_phrase.len() { break; }

            // unary operation
            if working_phrase[pos].is_unop() {
               if pos == 0 || (if pos > 0 { working_phrase[pos-1].is_expr() == false } else { true }) {
                    let expr = Parser::consume_check_for_grouping(&mut working_phrase, pos+1)?;
                    let op = working_phrase.remove(pos);

                    working_phrase.insert(pos,op.into_unary(expr));

                    pos = 0;
                    continue;
               }
            }

            // binary operation
            if working_phrase[pos].is_binop() {
                if pos > 0 && working_phrase.len() > pos {
                    // expr2 could not be a collapsed grouping here
                    // so we need to do some stuff in order to check it and then collapse
                    // it if it needs collapsing
                    //
                    // expr1 should have already been collapsed, so we don't need to check it.
                    let expr2 = Parser::consume_check_for_grouping(&mut working_phrase, pos+1)?; 
                    let op = working_phrase.remove(pos);
                    let expr1 = working_phrase.remove(pos-1);

                    working_phrase.insert(pos-1,op.into_binary(expr1,expr2));

                    pos = 0;
                    continue;
                }
            }

            // commas
            // creates a list of some sort.
            if working_phrase[pos].is_token(TokenType::Comma) {
                if pos > 0 && working_phrase.len() > pos {
                    let next_section = Parser::consume_until_check_for_grouping(&mut working_phrase, pos+1, &[TokenType::Comma, TokenType::Equal])?;
                    let mut pre_section_list = working_phrase.remove(pos-1).into_list().explode_list();
                    pre_section_list.push(Box::new(next_section));
                    working_phrase.insert(pos-1,Statement::create_list(pre_section_list));             
                    
                    working_phrase.remove(pos); // removes the comma;

                    pos = 0;
                    continue;
                }
            }
            
            // assignments
            if working_phrase[pos].is_token(TokenType::Equal) {
                if pos > 0 && working_phrase.len() > pos {
                    println!("ASSINGMENT");
                    for i in 0 .. working_phrase.len() { println!("{} : {}",i,working_phrase[i]); }
                    
                    // assignment is the end of the statement, there isn't anything else to do
                    // really.
                    let right_hand : Statement = Parser::collapse_statement(working_phrase.drain(pos+1 ..).collect())?.into_list();
                    let _op = working_phrase.remove(pos);
                    let left_hand = working_phrase.remove(pos-1).into_list();
                    let local = if pos > 1 {
                        if working_phrase[pos-2].is_token(TokenType::Local) {
                            working_phrase.remove(pos-2);
                            true
                        } else { false }
                    } else { false } ;
                    
                    // makes sure we have the right kind of list.
                    // a local assignment must be a namelist,
                    // a non-local assignment must be a varlist,
                    // and the right hand must be exprlist
                    if local && !left_hand.is_namelist() {
                        return Err(format_err!("Left side of a local assignment must be a name / namelist"));
                    }
                    if !local && !left_hand.is_varlist() {
                        return Err(format_err!("Left side of an assignment must be a var / varlist"));
                    }
                    if !right_hand.is_exprlist() {
                        return Err(format_err!("Right side of an assignment must be an expr / exprlist"));
                    }
                    
                    let assignment = Statement::create_assignment(left_hand,right_hand,local);
                    
                    working_phrase.insert(pos-1,assignment);
                    pos = 0;
                    continue;
                }
            }

            // grouping
            if working_phrase[pos].is_token(TokenType::LeftParen) {
                let grouped_statement = Parser::consume_check_for_grouping(&mut working_phrase,pos)?;
                
                // primative function call check, so we can get printing working
                // for testing
                if pos > 0 {
                    if working_phrase[pos-1].is_name(){
                        let function_name = working_phrase.remove(pos-1);
                        if let Statement::Group(group) = grouped_statement {
                            
                            working_phrase.insert(pos-1,
                                Statement::FunctionCall(
                                    Box::new(function_name),
                                    Box::new(*group)));

                            pos = 0;
                            continue;
                        }

                    }
                }
                
                working_phrase.insert(pos,grouped_statement);
                
                pos += 1;
                continue;
            }

            pos += 1;
        }

        match working_phrase.len() {
            0 => panic!("weird error"),
            1 => Ok(working_phrase.remove(0)),
            _ => Err(format_err!("Failed to collapse into a single statement, left with {} statements",working_phrase.len())),
        }
    }

    fn consume_until_check_for_grouping(working_phrase : &mut Vec<Statement>, start : usize, tokens : &[TokenType]) -> Result<Statement, Error> {
        let mut stop_point = working_phrase.len();

        for token in tokens.iter() {
            if let Ok(found) = Parser::find_token_with_depth(working_phrase,start,token.clone()) {
                if found < stop_point { stop_point = found; }
            }
        }

        let tokens : Vec<Statement> = working_phrase.drain(start .. stop_point).collect();
        let collapsed_tokens : Statement = Parser::collapse_statement(tokens)?;
        Ok(collapsed_tokens) 
    }

    fn consume_check_for_grouping(working_phrase : &mut Vec<Statement>, start : usize) -> Result<Statement,Error> {
        //! gets the next statement, and checks if it is a start of a grouping
        //! if it is then it will do the grouping magic stuff and create a new statement
        //! with the grouping, if it isn't then it will just return the next statement
        //!
        //! it will consume and remove the tokens / statements as it works

        let expr = working_phrase.remove(start);
        
        if expr.is_token(TokenType::LeftParen) {
            let until_pos = Parser::find_token_with_depth(&working_phrase, start, TokenType::RightParen)?;
            let insides : Vec<Statement> = working_phrase.drain(start .. until_pos).collect();

            let collapsed_inside : Statement = Parser::collapse_statement(insides)?;
            working_phrase.remove(start); // remove the right parenthesis
                         
            Ok(Statement::Group(Box::new(collapsed_inside)))
        } else { 
            Ok(expr)
        }
    }

    fn find_token_with_depth(working_phrase : &Vec<Statement>, start : usize, token : TokenType) -> Result<usize,Error> {
        let mut pos = start;
        let mut depth = if token == TokenType::RightParen { 1 } else { 0 };
        
        loop {
            if pos >= working_phrase.len() { break; }
            
            if working_phrase[pos].is_token(TokenType::LeftParen) { depth += 1; }
            if working_phrase[pos].is_token(TokenType::RightParen) { depth -=1; }

            if working_phrase[pos].is_token_ref(&token) && depth == 0 {
                return Ok(pos);
            }

            pos +=1;
        }

        Err(format_err!("Can't find expected end of phrase {}",token))
    }
}
