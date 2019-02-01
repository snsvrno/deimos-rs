use failure::{ Error, format_err };

use crate::scanner::Scanner;
use crate::elements::Chunk;
use crate::elements::Statement;
use crate::elements::TokenType;
use crate::elements::Token;
use crate::elements::CodeSlice;

enum InternalError {
    Syntax(CodeSlice),
    SyntaxMsg(String,CodeSlice),
    General(String),
}

impl InternalError {
    fn render(self, raw_code : &str) -> String {
        match self {
            InternalError::General(string) => format!("{}",string),
            InternalError::Syntax(code_slice) => format!("Syntax error parsing code:\n      {}\n      ^ line: {} col: {}",
                code_slice.slice_code(raw_code),
                code_slice.get_line(),
                code_slice.get_column()
            ),
            InternalError::SyntaxMsg(prefix,code_slice) => format!("{}:\n      {}\n      ^ line: {} col: {}",
                prefix,
                code_slice.slice_code(raw_code),
                code_slice.get_line(),
                code_slice.get_column()
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

    ////////////////////////////////////////////////////////////////////
    /// PUBLIC FUNCTIONS

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

    pub fn disassemble(self) -> (&'a str, Vec<Chunk>) {
        (self.raw_code,self.chunks)
    }

    ////////////////////////////////////////////////////////////////////
    /// PRIVATE FUNCTIONS
    
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
                TokenType::Function => {
                    let function_statement = Parser::collapse_function(&mut raw_statements)?;
                    Option3::Some(function_statement)
                },
                TokenType::Do => {
                    let do_statement = Parser::collapse_block_statement(&mut raw_statements,TokenType::Do)?;
                    Option3::Some(do_statement)
                },
                TokenType::While => {
                    let while_statement = Parser::collapse_block_statement(&mut raw_statements, TokenType::While)?;
                    Option3::Some(while_statement)
                },
                /*TokenType::LeftMoustache => {
                    let table_constructor = Parser::collapse_table_constructor(&mut raw_statements)?;
                    Option3::Some(table_constructor)
                },*/
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

    fn collapse_assignment(mut before : Vec<Statement>, after : Vec<Statement>) -> Result<Statement,InternalError> {
        //! processess the assignment statement format, is triggered when a '=' is found.
        //! 
        //! checks the following validations when creating the assignment
        //! 
        //! ``` text
        //! 
        //!     [x]    varlist `=´ explist
        //!     [x]    varlist ::= var {`,´ var}
        //!     [x]    explist ::= {exp `,´} exp
        //!     [x]    local namelist [`=´ explist] 
        //!     [x]    namelist ::= Name {`,´ Name}
        //! 
        //! ```

        let local = Parser::contains_token(&before, TokenType::Local);
        Parser::remove_token(&mut before, TokenType::Local);

        let before_list : Statement = { 
            let mut new_list : Vec<Box<Statement>> = Vec::new();
            for b in before { new_list.push(Box::new(b)); } 
            Statement::create_list(new_list)
        };

        // checking each element making sure its the right stuff
        match local {
            false => {
                // if it is not a local, then the left can be a varlist / var
                if !before_list.is_varlist() {
                    return Err(InternalError::SyntaxMsg("Left side of '=' must be a var".to_string(), before_list.get_code_slice()))
                }
            },
            true => {
                // if it is local then it must be a namelist, not a varlist.  
                if !before_list.is_namelist() {
                    return Err(InternalError::SyntaxMsg("Left side of a local '=' must be a name".to_string(), before_list.get_code_slice()))
                }
            }
        }

        // splits the expressions by ',' and the collapses each piece.
        let expr_list : Statement = { 
            
            let potential_list = Parser::collapse_statement(after)?;
            if potential_list.is_a_list() {
                if !potential_list.is_exprlist() {
                    return Err(InternalError::SyntaxMsg("Right side of '=' must be an expression".to_string(), potential_list.get_code_slice()));
                }
                potential_list
                // else its a exprlist and good to go.
            } else {
                // need to convert it to a list.
                let list = Statement::create_list(vec![Box::new(potential_list)]);
                if !list.is_exprlist() {
                    return Err(InternalError::SyntaxMsg("Right side of '=' must be an expression".to_string(), list.get_code_slice()));
                }
                list
            }
      };

        let assignment = Statement::create_assignment(before_list,expr_list,local);
        Ok(assignment)
    }

    fn collapse_function(mut raw_statements : &mut Vec<Statement>) -> Result<Statement,InternalError> { 
        //! reads the stream and collects the function, will collapse the interals of the function if
        //! required to
        
        let mut function_body : Vec<Box<Statement>> = Vec::new();
        let mut working_statement : Vec<Statement> = Vec::new();

        let name : Option<String> = {
            match Parser::consume_until_token(&mut raw_statements, TokenType::LeftParen, false) {
                Err(code_slice) => return Err(InternalError::Syntax(code_slice)),
                Ok(toks) => {
                    if toks.len() == 1 {
                        Some(toks[0].as_name().to_string())
                    } else {
                        None
                    }
                }
            }
        };

        
        let arguements : Vec<String> = {
            match Parser::consume_until_token(&mut raw_statements, TokenType::RightParen, false) {
                Err(code_slice) => return Err(InternalError::Syntax(code_slice)),
                Ok(toks) => {
                    let mut names : Vec<String> = Vec::new();
                    for i in 0 .. toks.len() {
                        // starts with 0, so that means 0 is even, confusing because you think
                        // that the first arguement is 1 ...
                        if i % 2 == 1 {
                            // is odd, must be a comma
                            if !toks[i].is_token(TokenType::Comma){
                                return Err(InternalError::SyntaxMsg("Function arguements must be separated with a comma".to_string(), toks[i].get_code_slice()));
                            } 
                        } else {
                            // is even, is the identifier
                            match toks[i].is_name() {
                                true => names.push(toks[i].as_name().to_string()),
                                false => return Err(InternalError::SyntaxMsg("Function arguements in a definition can only be names".to_string(),toks[i].get_code_slice())),
                            }
                        }
                    }
                    names
                }
            }
        };

        loop {
            let token = raw_statements.remove(0);
            match token.as_token_type() {
                TokenType::End => {
                    if working_statement.len() > 0 {
                        function_body.push(Box::new(Parser::collapse_statement(working_statement)?));
                    }
                    break;
                },
                TokenType::EOL => {
                    if working_statement.len() > 0 {
                        function_body.push(Box::new(Parser::collapse_statement(working_statement)?));
                        working_statement = Vec::new();
                    }
                },
                _ => {
                    working_statement.push(token);
                }
            }
        }

        match name {
            Some(name) => Ok(Statement::FunctionNamed(name,arguements,function_body)),
            None => Ok(Statement::Function(arguements,function_body)),
        }
    }

    fn collapse_return_statement(working_statement : Vec<Statement>) -> Result<Statement,InternalError> {
        //! peeks forward to get the correct return statement
        
        let statement = {
            let mut statement = Parser::collapse_statement(working_statement)?;
            if statement.is_expr() { 
                statement = Statement::ExprList(vec![Box::new(statement)]);
            }
            statement
        };
        
        if statement.is_exprlist() {
            return Ok(Statement::Return(Box::new(statement)));
        } else {
            return Err(InternalError::SyntaxMsg("Return must be an expression list if defined.".to_string(),statement.get_code_slice()));
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
                    Err(code_slice) => return Err(InternalError::SyntaxMsg("Couldn't find the `do` in the `while .. do .. end`".to_string(),code_slice)),
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
        //let mut list : Vec<Box<Statement>> = Vec::new();

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

            if statement[pos].is_token(TokenType::Comma) {
                if pos == 0 { return Err(InternalError::General("Error collapsing statement, comma not where it should be.".to_string())); }

                let prefix : Option<Vec<Statement>> = if pos >= 2 {
                    let mut prefix : Vec<Statement> = Vec::new();
                    for i in (0 .. pos-1).rev() {
                        prefix.insert(0,statement.remove(i));
                    }
                    Some(prefix)
                } else { None };

                let pre = statement.remove(0); // removes the statement before
                statement.remove(0); // removes the token
                let post = match Parser::consume_until_tokens_with_grouping(&mut statement, &[TokenType::Comma, TokenType::RightParen, TokenType::EOL, TokenType::Equal] , false) {
                    Err(_) => panic!("parsing comma phrase, can't find the end of the comma group"),
                    Ok(post) => {
                        Parser::collapse_statement(post)?
                    }
                };

                let list : Option<Statement> = match pre.is_a_list() {
                    true => {
                        pre.add_to_list(post)
                    },
                    false => {
                        let new_list = Statement::create_list(vec![ Box::new(pre), Box::new(post)]);
                        Some(new_list)
                    },
                };

                match list {
                    Some(list) => statement.insert(0,list),
                    None => return Err(InternalError::SyntaxMsg("Can't have a list of different types".to_string(),statement[0].get_code_slice())), 
                }

                if let Some(mut prefix) = prefix {
                    for i in (0 .. prefix.len()).rev() {
                        statement.insert(0,prefix.remove(i));
                    }
                }

                pos = pos-1;
                continue;
            }

            // checks if it is an assignment
            if statement[pos].is_token(TokenType::Equal) {
                statement.remove(pos); // need to remove the equals otherwise we will overflow the stack

                let before : Vec<Statement> = statement.drain(0 .. pos).collect();

                let assignment_statement = Parser::collapse_assignment(before, statement)?;
                return Ok(assignment_statement);
            }

            // table constructor
            if statement[pos].is_token(TokenType::LeftMoustache) {
                
                let mut before : Vec<Statement> = statement.drain(0 .. pos).collect();
                statement.remove(0); // removes the { token
 
                let table = match Parser::consume_until_token(&mut statement, TokenType::RightMoustache, false) {
                    Err(code_slice) => return Err(InternalError::Syntax(code_slice)),
                    Ok(tokens) => {
                        let insides = Parser::collapse_statement(tokens)?;
                        match insides.is_fieldlist() {
                            false => return Err(InternalError::SyntaxMsg(
                                "Tables must be made with a field list".to_string(),
                                insides.get_code_slice())),
                            true => Statement::create_table(insides),
                        }
                    }
                };

                statement.insert(0,table);
                for i in (0 .. before.len()).rev() {
                    statement.insert(0,before.remove(i));
                }

                pos += 1;
                continue;
            }

            if statement[pos].is_token(TokenType::LeftParen) {

                // remove everything before the parenthesis
                let mut pre_tokens : Vec<Statement> = statement.drain(0 .. pos).collect();
                let touching_token : Option<&Statement> = if pre_tokens.len() > 0 { Some(&pre_tokens[pre_tokens.len()-1]) } else { None };
                statement.remove(0); // remove the Left Parenthesis

                // if a parenthesis it could either be a function call, or it could be a grouping.
                match Parser::consume_until_token(&mut statement, TokenType::RightParen, false) {
                    Err(code_slice) => return Err(InternalError::SyntaxMsg("Could not find the end of the parenthesis".to_string(),code_slice)),
                    Ok(insides) => {
                        // checking if its a function call
                        // TODO : add stuff for bob.func() and bob:func()
                        if let Some(ref token) = touching_token {
                            if token.is_prefixexp() {
                                let function_ident = pre_tokens.remove(pre_tokens.len()-1);
                                let insides_collapsed = if insides.len() == 0 { 
                                    Statement::Empty 
                                } else { 
                                    Parser::collapse_statement(insides)?
                                };
                                
                                if !insides_collapsed.is_args() && insides_collapsed != Statement::Empty {
                                    return Err(InternalError::SyntaxMsg("Function call requires arguements".to_string(),insides_collapsed.get_code_slice()));
                                }

                                let funccal = Statement::FunctionCall(Box::new(function_ident),Box::new(insides_collapsed));
                                
                                statement.insert(0,funccal);
                                
                                // returning the prestuff
                                for i in (0 .. pre_tokens.len()).rev() {
                                    statement.insert(0,pre_tokens.remove(i));
                                }
                                
                            }
                        }                       

                        pos = 0;
                        continue;
                    }
                }
            }

            if statement[pos].is_token(TokenType::Return) {
                statement.remove(pos);
                let mut prestuff : Vec<Statement> = statement.drain(0 .. pos).collect();

                match Parser::consume_until_tokens(&mut statement,&[ TokenType::End,TokenType::EOL,TokenType::While ],false) {
                    Err(code_slice) => return Err(InternalError::SyntaxMsg("IDK What this error is, but its in a return statement".to_string(),code_slice)),
                    Ok(slice) => {
                        let return_statement = Parser::collapse_return_statement(slice)?;
                        statement.insert(0,return_statement);
                        for i in (0 .. prestuff.len()).rev() { statement.insert(0,prestuff.remove(i)); }

                        pos = 0;
                        continue;
                    }
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
            _ => Err(InternalError::Syntax(CodeSlice::create_from(
                    &statement[0].get_code_slice(),
                    &statement[statement.len()-1].get_code_slice()
                ))),
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

    // TODO : remove the result on this function and its other version `consume_until_token`
    fn consume_until_tokens(buffer : &mut Vec<Statement>, desired_tokens : &[TokenType], include : bool) -> Result<Vec<Statement>,CodeSlice> {
        let mut tokens : Vec<Statement> = Vec::new();
        loop {
            if buffer.len() <= 0 { return Ok(tokens); }

            let token = buffer.remove(0);
            
            // checks if the token is the desired token, but if the desired token is EOL then it will also match on EOF token).
            for desired_token in desired_tokens {
                if token.as_token_type() == desired_token || (desired_token == &TokenType::EOL && token.as_token_type() == &TokenType::EOF) {
                    if include { tokens.push(token); }
                    return Ok(tokens);
                }
            }
            tokens.push(token);
        }
    }

    fn consume_until_tokens_with_grouping(buffer : &mut Vec<Statement>, desired_tokens : &[TokenType], consume_desired : bool) ->  Result<Vec<Statement>,CodeSlice> {
        let mut tokens : Vec<Statement> = Vec::new();
        let mut depth = 0;
        loop {
            if buffer.len() <= 0 { return Ok(tokens); }

            let token = buffer.remove(0);

            if token.as_token_type() == &TokenType::LeftParen {
                tokens.push(token);
                depth += 1;
                continue;
            } else if token.as_token_type() == &TokenType::RightParen {
                if depth >= 0 {
                    tokens.push(token);
                    depth -= 1;
                    continue;
                }
            }

            // checks if the token is the desired token, but if the desired token is EOL then it will also match on EOF token).
            for desired_token in desired_tokens {
                if token.as_token_type() == desired_token || (desired_token == &TokenType::EOL && token.as_token_type() == &TokenType::EOF) {
                    if !consume_desired { buffer.insert(0,token); } 
                    return Ok(tokens);
                }
            }
            tokens.push(token);
        }
    }

    fn consume_until_token(buffer : &mut Vec<Statement>, desired_token : TokenType, include : bool) -> Result<Vec<Statement>,CodeSlice> {
        Parser::consume_until_tokens(buffer, &[desired_token], include)
    }

    fn consume_until_token_with_grouping(buffer : &mut Vec<Statement>, desired_token : TokenType, consume_desired : bool) -> Result<Vec<Statement>,CodeSlice> {
        Parser::consume_until_tokens_with_grouping(buffer, &[desired_token], consume_desired)
    }

    fn remove_token(buffer : &mut Vec<Statement>, desired_token : TokenType) {
        for i in (0 .. buffer.len()).rev() {
            if buffer[i].is_a_token() {
                if buffer[i].as_token_type() == &desired_token {
                    buffer.remove(i);
                }
            }
        }
    }

    fn contains_token(tokens : &Vec<Statement>, token_to_look_for : TokenType) -> bool {
        //!
        //! 
        //! should only be used for Statement::Token(_) so shouldn't panic.
        
        for t in tokens.iter() {
            if t.is_a_token() {
                if t.as_token_type() == &token_to_look_for { return true; }
            }
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
    #[ignore]
    fn assignment_simple() {

        // single assignment
        assert_eq!(setup_simple!("bob = 5 + 4").chunks[0],
            chunk!(assignment!(
                ( "bob" ),
                ( binary!("+","5","4") )
            )));

        // single assignment, local
        assert_eq!(setup_simple!("local bob = 5 + 4").chunks[0],
            chunk!(assignment_local!(
                ( "bob" ),
                ( binary!("+","5","4") )
            )));

        // double assignment
        assert_eq!(setup_simple!("bob,linda = 5,4").chunks[0],
            chunk!(assignment!(
                ( "bob","linda" ),
                ( statement!("5"),statement!("4") )
            )));

        // double assignment, local
        assert_eq!(setup_simple!("local bob,linda = 5,4").chunks[0],
            chunk!(assignment_local!(
                ( "bob","linda" ),
                ( statement!("5"),statement!("4") )
            )));

        // mismatched assignment
        assert_eq!(setup_simple!("bob,linda,jorge = 5 * 4 + 3,false").chunks[0],
            chunk!(assignment!(
                ( "bob","linda","jorge" ),
                ( 
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

        let code = include_str!("../../lua/loops_complex/do.lua");
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

    #[test]
    
    fn functions() {

        let function_def = setup_simple!("
        function test(a,b,c)
            local temp = a + b
            return c * temp
        end

        result = test(1,2,3)
        ");

        let check_against = chunk!(
            function!(
                "test",("a","b","c"),
                assignment_local!(
                    ( "temp" ),
                    ( binary!("+","a","b") )
                ),
                return_stat!(
                    binary!("*","c","temp")
                )
            ),
            assignment!(
                ( "result" ),
                (function_call!("test", 
                    ( statement!("1"), statement!("2"), statement!("3")) 
                ))
            )
        );

        print_em!(function_def.chunks[0]);
        print_em!(check_against);

        assert_eq!(function_def.chunks[0],check_against);
    }

    #[test]
    #[ignore]
    fn tables() {
        let ss = setup!("
        bob = { 10,20,30,40 }
        ");

        print_em!(ss.chunks[0]);

        assert!(false);
    }
}
