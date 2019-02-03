use crate::elements::{Chunk, Statement, TokenType};
use crate::scanner::Scanner;

use failure::{Error,format_err};

#[derive(PartialEq,Copy,Clone)]
enum ParserMode {
    Normal,
    Table,
}

pub struct Parser<'a> {
    raw_code : &'a str,
    chunk : Chunk,
}

impl<'a> Parser<'a> {
    
    pub fn from_scanner(scanner : Scanner <'a>) -> Result<Parser,Error> {
        //! creates a parser from a scanner.
        //!
        //! takes all the scanned tokens and organizes them into a tree 
        //! that represents the code.

        // explodes the scanner object and gets the pieces that matter
        let (raw_code, tokens) = scanner.disassemble();
        // converts the tokens into Statements so they can be processessed
        let mut raw_statements = Statement::tokens_to_statements(tokens);

        // there is ultimately just 1 single chunk in a program, this is that chunk
        // all other statements and such are organized inside this chunk or recusively
        // deeper in there as additional chunks.
        let master_chunk = Parser::process(&mut raw_statements)?;

        Ok(Parser {
            raw_code : raw_code,
            chunk : master_chunk,
        })
        
    }

    pub fn disassemble(self) -> (&'a str, Chunk) {
        //! explodes the parser object for its next step in the life journey
        
        (self.raw_code,self.chunk)
    }

    fn process(mut raw_statements : &mut Vec<Statement>) -> Result<Chunk,Error> {
        //! the main processessing function, looks at the statements and returns a chunk
        //! 
        //! this function process the major blocks of codes, things that separate chunks or
        //! blocks, such as `do-end` loops, `if-else-end` blocks, function definitions, etc.
        //! this shouldn't be processess variable assignments or the smaller things that happen
        //! within a statement

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
                // statement enders.
                TokenType::EOF |
                TokenType::EOL => {
                    if working_phrase.len() > 0 {
                        
                        let state = Parser::collapse_statement(ParserMode::Normal, working_phrase)?;
                        current_chunk.add(state);
                        // creating new objects that we just used.
                        working_phrase = Vec::new();
                    }
                }, 

                // function definition block
                TokenType::Function => {

                    // need to get the function name and parameters.
                    let func_name =  {
                        let function_name = Parser::consume_until(&mut raw_statements, 0, &[TokenType::LeftParen])?;
                        Parser::remove_all_until(&mut raw_statements, 0, TokenType::LeftParen)?; // removes the left Parenthesis that remains.
                        Parser::collapse_statement(ParserMode::Normal, function_name)?
                    };

                    // function parameters / arglist, checks type to make sure it is a namelist.
                    let args = {
                        let preamble_section = Parser::consume_until(&mut raw_statements, 0, &[TokenType::RightParen])?;
                        Parser::remove_all_until(&mut raw_statements, 0, TokenType::RightParen)?; // removes the right Parenthesis that remains.
                            let args = Parser::collapse_statement(ParserMode::Normal, preamble_section)?.into_list_or_empty();

                        if !(args.is_a_list() && args.is_namelist() || args.is_empty()) {
                            return Err(format_err!("Function arguments must be names! : {}",args));
                        }
                        args
                    };

                    // function content, the chunk, and creates the function object
                    let func = {
                        let mut insides = Parser::consume_until_check_for_grouping(&mut raw_statements, 1, &[TokenType::End])?;
                        Parser::remove_all_until(&mut raw_statements, 0, TokenType::End)?; // removes the end that remains.
                        let collapsed_insides = Parser::process(&mut insides)?;
                        
                        Statement::Function(Box::new(args), collapsed_insides)
                    };

                    // all functions are nameless, and are assigned to things like variables (at least that is how they are being treated)
                    let assignment = Statement::create_assignment(func_name,func,false);
                    working_phrase.push(assignment);
                },

                // nothing here, so continue to add to the working_phrase.
                _ => working_phrase.push(token),
            }
        }

        Ok(current_chunk)
    }

    fn collapse_statement(mode : ParserMode, mut working_phrase : Vec<Statement>) -> Result<Statement,Error> {
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
            //      `not true`
            if working_phrase[pos].is_unop() {
               if pos == 0 || (if pos > 0 { working_phrase[pos-1].is_expr() == false } else { true }) {
                    let expr = Parser::consume_check_for_grouping(mode, &mut working_phrase, pos+1)?;
                    let op = working_phrase.remove(pos);

                    working_phrase.insert(pos,op.into_unary(expr));

                    pos = 0;
                    continue;
               }
            }

            // binary operation
            //      `5 + 3`
            if working_phrase[pos].is_binop() {
                if pos > 0 && working_phrase.len() > pos {
                    // expr2 could not be a collapsed grouping here
                    // so we need to do some stuff in order to check it and then collapse
                    // it if it needs collapsing
                    //
                    // expr1 should have already been collapsed, so we don't need to check it.
                    let expr2 = Parser::consume_check_for_grouping(mode, &mut working_phrase, pos+1)?; 
                    let op = working_phrase.remove(pos);
                    let expr1 = working_phrase.remove(pos-1);

                    working_phrase.insert(pos-1,op.into_binary(expr1,expr2));

                    pos = 0;
                    continue;
                }
            }

            // commas
            // creates a list of some sort.
            //      `x,y,z,a` and `1,2,3,4` of `x,y,z,a = 1,2,3,4`
            if working_phrase[pos].is_token(TokenType::Comma) {
                if pos > 0 && working_phrase.len() > pos {
                    
                    // changes the search terms if we are in the table mode, this is because normally when
                    // you see a comma is probably an assignment thing
                    //      x,y,z = 1,2,3
                    // so you want to stop when you see an `equals` because its the end of the list, but if
                    // you are in a table the equals is part of that section between the commas
                    //      x = { y = 1, z = 2, 30 }
                    // so we need to collect the equals and move to the next comma section
                    
                    let search_terms = if mode == ParserMode::Table {
                        vec![TokenType::Comma]
                    } else {    
                        vec![TokenType::Comma, TokenType::Equal]
                    };

                    let next_section = Parser::consume_until_check_for_grouping(&mut working_phrase, pos + 1, &search_terms)?;
                    
                    let collapsed_tokens : Statement = Parser::collapse_statement(mode, next_section)?;
                    let mut pre_section_list = working_phrase.remove(pos-1).into_list().explode_list();

                    pre_section_list.push(Box::new(collapsed_tokens));
                    working_phrase.insert(pos-1,Statement::create_list(pre_section_list));             
                    
                    working_phrase.remove(pos); // removes the comma;

                    pos = 0;
                    continue;
                }
            }

            // table access, bracket
            //      `x[4]`
            if working_phrase[pos].is_token(TokenType::LeftBracket) {
                if pos > 0 && working_phrase.len() > pos {

                    let insides = Parser::consume_until_check_for_grouping(&mut working_phrase, pos + 1, &[TokenType::RightBracket])?;
                    let collapsed_insides = Parser::collapse_statement(mode, insides)?;

                    let identifier = working_phrase.remove(pos-1);
                    // creates the inside list of items
                    let list_of_stuff : Vec<Box<Statement>> = if identifier.is_complex_var() {
                        let mut list = identifier.explode_list();
                        list.push(Box::new(collapsed_insides));
                        list
                    } else {
                        let mut list : Vec<Box<Statement>> = Vec::new();
                        list.push(Box::new(identifier));
                        list.push(Box::new(collapsed_insides));
                        list
                    };

                    working_phrase.remove(pos); // removes the `]` bracket
                    working_phrase.remove(pos - 1); // removes the `[` bracket
                    working_phrase.insert(pos - 1,Statement::ComplexVar(list_of_stuff));

                    pos = 0;
                    continue;
                }
            }

            // table access, dot
            //      `x.y`
            if working_phrase[pos].is_token(TokenType::Period) {
                if pos > 0 && working_phrase.len() > pos {
                    if working_phrase[pos+1].is_name() {
                        let next : Statement = { 
                            let id = working_phrase.remove(pos+1);
                            let string = id.as_user_output().unwrap();
                            Statement::Token(crate::elements::Token::new(
                                TokenType::String(string),
                                id.get_code_slice()
                            ))
                        };
                        working_phrase.remove(pos); // removes the `.`
                        let identifier = working_phrase.remove(pos-1);

                        // creates the inside list of items
                        let list_of_stuff : Vec<Box<Statement>> = if identifier.is_complex_var() {
                            let mut list = identifier.explode_list();
                            list.push(Box::new(next));
                            list
                        } else {
                            let mut list : Vec<Box<Statement>> = Vec::new();
                            list.push(Box::new(identifier));
                            list.push(Box::new(next));
                            list
                        };

                        working_phrase.insert(pos - 1,Statement::ComplexVar(list_of_stuff));
                        pos = 0;
                        continue;
                    }
                }
            }

            // table constructor
            //      `{ 1,2,3,4 }`
            //      `{ 1,2,x=4,y={1,2}}`
            if working_phrase[pos].is_token(TokenType::LeftMoustache) {
                if working_phrase.len() > pos {
                    // lets a flag to let it know that is trying to work inside a table.
                    // so far only does this because of the ability to do assignments to parts 
                    // of a table inside a table.
                    //
                    //  x = { y = 5, 1, 2 }

                    let insides = Parser::consume_until_check_for_grouping(&mut working_phrase, pos + 1, &[TokenType::RightMoustache])?;

                    let collapsed_tokens : Statement = Parser::collapse_statement(ParserMode::Table, insides)?;
                    let table = Statement::create_table(collapsed_tokens);
                    
                    working_phrase.remove(pos+1); // removes the '}' moustache
                    working_phrase.remove(pos); // removes the `{` moustache
                    working_phrase.insert(pos,table);


                    pos = 0;
                    continue;
                }
            }
            
            // assignments
            //      `x = 1`
            if working_phrase[pos].is_token(TokenType::Equal) {
                if pos > 0 && working_phrase.len() > pos {
                    
                    // assignment is the end of the statement, there isn't anything else to do
                    // really.
                    let right_hand : Statement = Parser::collapse_statement(mode, working_phrase.drain(pos+1 ..).collect())?.into_list();
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
            //      `(3+4)`
            if working_phrase[pos].is_token(TokenType::LeftParen) {

                let grouped_statement = Parser::consume_check_for_grouping(mode, &mut working_phrase,pos)?;

                // primative function call check, so we can get printing working
                // for testing
                // TODO : change this, make it better so we can handle the other calls
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
            0 => Ok(Statement::Empty),
            1 => Ok(working_phrase.remove(0)),
            _ => Err(format_err!("Failed to collapse into a single statement, left with {} statements",working_phrase.len())),
        }
    }

    fn consume_until(working_phrase : &mut Vec<Statement>, start : usize, tokens : &[TokenType]) -> Result<Vec<Statement>, Error> {
        //! basic consuming function that will read the tokens and stop at one of the desired tokens in `tokens`.
        //! 
        //! it will NOT consume the token define in `tokens` and will need to manually be removed (such as with `working_phrase.remove(0)`)
        //! it WILL include the token at `pos = start`
        //! 
        //! does not do any grouping or smart parsing, just finds the first occurance of the token in question and returns a list of tokens
        
        let mut stop_point = working_phrase.len();

        for token in tokens.iter() {
            if let Ok(found) = Parser::find_next_token_of(working_phrase,start,token.clone()) {
                if found < stop_point { stop_point = found; }
            }
        }

        let tokens : Vec<Statement> = working_phrase.drain(start .. stop_point).collect();
        Ok(tokens) 
    }

    fn consume_until_check_for_grouping(working_phrase : &mut Vec<Statement>, start : usize, tokens : &[TokenType]) -> Result<Vec<Statement>, Error> {
        //! smarter consuming function that will listen to grouping rules when consuming. similar to `consume_until` except it cares about grouping.
        
        let mut stop_point = working_phrase.len();
        
        for token in tokens.iter() {
            if let Ok(found) = Parser::find_token_with_depth(working_phrase,start,token.clone()) {
                if found < stop_point { stop_point = found; }
            }
        }

        let tokens : Vec<Statement> = working_phrase.drain(start .. stop_point).collect();
        Ok(tokens) 
    }

    fn consume_check_for_grouping(mode : ParserMode, working_phrase : &mut Vec<Statement>, start : usize) -> Result<Statement,Error> {
        //! gets the next statement, and checks if it is a start of a grouping
        //! if it is then it will do the grouping magic stuff and create a new statement
        //! with the grouping, if it isn't then it will just return the next statement
        //!
        //! it will consume and remove the tokens / statements as it works

        let expr = working_phrase.remove(start);
        
        if expr.is_token(TokenType::LeftParen) || expr.is_token(TokenType::LeftBracket) || expr.is_token(TokenType::LeftMoustache) {
            let until_pos = Parser::find_token_with_depth(&working_phrase, start, TokenType::RightParen)?;
            let insides : Vec<Statement> = working_phrase.drain(start .. until_pos).collect();

            let collapsed_inside : Statement = Parser::collapse_statement(mode, insides)?;
            working_phrase.remove(start); // remove the right parenthesis
                         
            Ok(Statement::Group(Box::new(collapsed_inside)))
        } else { 
            Ok(expr)
        }
    }

    fn find_next_token_of(working_phrase : &Vec<Statement>, start : usize, token : TokenType) -> Result<usize,Error> {
        //! basic search function that finds the next occurance of the desired `token` and returns its position
        
        let mut pos = start;

        loop {
            if pos >= working_phrase.len() { break; }

            if working_phrase[pos].is_token_ref(&token){
                return Ok(pos);
            }

            pos += 1;
        }

        Err(format_err!("Can't find expected end of phrase {}",token))
    }

    fn remove_all_until(working_phrase : &mut Vec<Statement>, start : usize, token : TokenType) -> Result<(),Error> {
        //! goes through the stream and removes all tokens until the one `token` desired, removing that token aswell.
        
        loop {
            if working_phrase.len() <= start { break; }

            let current_token = working_phrase.remove(start);
            if current_token.as_token_type() == &token {
                return Ok(())
            }
        }

        Err(format_err!("Couldn't find desired token {}",token))
    }

    fn find_token_with_depth(working_phrase : &Vec<Statement>, start : usize, token : TokenType) -> Result<usize,Error> {
        //! smart search function that will only return the position if it finds the `token` with the acceptable `depth`
        //! 
        //! respects grouping performed by `() {} []`, so if requesting a RightParen `)` for the following ...
        //! ```text
        //!     x = 5 + (4 * (6-4))
        //! ```
        //! it will find the last parentheses, and return that position, not the first right parenthesis because 
        //! the grouping isn't finished yet

        let mut pos = start;
        let mut depth = match token {
            TokenType::RightParen | 
            TokenType::RightMoustache | 
            TokenType::RightBracket => 1,
            _ => 0,
        };
        
        loop {
            if pos >= working_phrase.len() { break; }
            
            // modifies the depth
            if working_phrase[pos].is_token(TokenType::LeftParen) 
            || working_phrase[pos].is_token(TokenType::LeftMoustache) 
            || working_phrase[pos].is_token(TokenType::LeftBracket) {
                depth += 1;
            }
            if working_phrase[pos].is_token(TokenType::RightParen) 
            || working_phrase[pos].is_token(TokenType::RightMoustache) 
            || working_phrase[pos].is_token(TokenType::RightBracket) {
                depth -= 1;
            }
            
            if working_phrase[pos].is_token_ref(&token) && depth == 0 {
                return Ok(pos);
            }

            pos +=1;
        }

        Err(format_err!("Can't find expected end of phrase {}",token))
    }
}