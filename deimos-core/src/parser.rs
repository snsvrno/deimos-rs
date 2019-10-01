use crate::error::CodeInformation;
use crate::token::Token;
use crate::codewrap::{CodeWrap, CodeWrappable};
use crate::scanner::{Scanner,TokenWrapped};
use crate::syntax::{
    exp, explist, statement, var, varlist,
    final_compress,
    SyntaxElement, SyntaxResult
};
use crate::parsererror::ParserError;

use failure::Error;

type Block = CodeWrap<SyntaxElement>;

pub struct Parser<'a> {
    pub file_name : String,
    pub raw_code : &'a str,
    pub blocks : Option<Block>,

    // working private members,
    tokens : Vec<TokenWrapped>,
    working_block : Vec<Block>,
    block_stack : Vec<(Block, Vec<Block>)>,
}


impl<'a> std::default::Default for Parser<'a> {
    fn default() -> Parser<'a> {
        Parser {
            raw_code : "",
            file_name : String::from("buffer"),
            blocks : None,

            tokens : Vec::new(),
            working_block : Vec::new(),
            block_stack : Vec::new(),

        }
    }
}

impl<'a> CodeInformation for Parser<'a> {
    fn raw_code(&self) -> String { self.raw_code.to_string() }
    fn file_name(&self) -> String { self.file_name.to_string() }
}

impl<'a> Parser<'a>{
    pub fn from_scanner(scanner : Scanner<'a>) -> Parser<'a> {

        Parser {
            raw_code : scanner.raw_code,
            tokens : scanner.tokens,

            .. Parser::default()
        }
    }


    pub fn parse(mut self) -> Result<Self,Error> {
        //! works through the tokens and makes them into statements, chunks and blocks
        //! 1. it will loop through and attempt to collect all the tokens into statements
        //! 2. next it will try to group the statements as chunks
        //! 3. if we only get one chunk at the end it will then wrap it as a block
        //!    and call it a day.

        loop {
            // we get the next set of tokens that defines a phrase
            match self.get_next_working_phrase() {
                None => break,
                Some(mut phrase) => {

                    loop {
                        // check for chunk
                        // check for block
                        // don't do these here, just leaving this so i remember about them

                        // check for statement
                        match statement::process(&mut phrase) { 
                            SyntaxResult::Done => continue,
                            SyntaxResult::Wrap(wrap_type) => {
                                // we go inside a block, need to track that here
                                self.block_stack.push((wrap_type, Vec::new()));
                                
                                // and need to remove the current phrase, it should
                                // always be garbage (or already stored in the stack)
                                // because of the way the scanner breaks tokens and 
                                // statements, TODO : need to make tests to prove this
                                phrase = Vec::new();
                                break;
                            }
                            _ => { },
                        }
                        // check for laststatement
                        // check for funcname
                        
                        // check for varlist
                        match varlist::process(&mut phrase) { 
                            SyntaxResult::Done => continue,
                            _ => { },
                        }
                        
                        // check for var
                        match var::process(&mut phrase) { 
                            SyntaxResult::Done => continue,
                            _ => { },
                        }
                        // check for namelist
                        
                        // check for explist
                        match explist::process(&mut phrase) { 
                            SyntaxResult::Done => continue,
                            _ => { },
                        }

                        // check for expression
                        if exp::process(&mut phrase) { continue; }
                        // check for prefixexp
                        // check for functioncall
                        // check for args
                        // check for function
                        // check for functionbody
                        // check for parlist
                        // check for tableconstructor
                        // check for field list
                        // check for field
                        
                        break;
                    }

                    // checks for any statement closes in the stack
                    if phrase.len() > 0 && self.block_stack.len() > 0 {
                        // we check the last element of the phrase if its a token,
                        // if it is we check against all the statement types that can contain
                        // other statements inside of it and checks if we have the closer for the 
                        // current block_stack item
                        let pos = phrase.len() - 1;
                        if let CodeWrap::CodeWrap(SyntaxElement::Token(ref token),_, end) = phrase[pos] {
                            // gets the stack_item
                            let stack_pos = self.block_stack.len() - 1;
                            let (CodeWrap::CodeWrap(ref stack_item, start, _), ref mut stack) = self.block_stack[stack_pos];

                            if &stack_item.ending_token() == token {
                                // now we are going to construct the insides, and the end,
                                // taking the start and anything addition from inside the `stack_item`

                                // makes a block out of all the inside pieces
                                let inner_block = match final_compress(stack) {
                                    SyntaxResult::Error(start, end, description) => return Err(ParserError::general_error(&self, start, end, &description)),
                                    SyntaxResult::Wrap(CodeWrap::CodeWrap(inner_block, _, _)) => inner_block,
                                    _ => unimplemented!(),
                                };

                                // checks if the insides are a block, because they need to be a block
                                if !inner_block.is_block() {
                                    return Err(ParserError::general_error(&self, start, end, "must be able to reduce down to a block"));
                                }

                                // builds the DoEnd element
                                let new_item = match stack_item {
                                    SyntaxElement::StatementDoEnd(_) => SyntaxElement::StatementDoEnd(Box::new(inner_block)),
                                    _ => { unimplemented!(); }
                                };
                                // creates the piece we will inject upwards.
                                let code_item = CodeWrap::CodeWrap(new_item, start, end);

                                // checks were we are going to put this, either we go one way up
                                // the stack, or we add it to the main working_block
                                match pos {
                                    // adds it to the main block
                                    0 => self.working_block.push(code_item),
                                    _ => {
                                        // adds it one up the stack
                                        let (_, ref mut stack_parent) = self.block_stack[pos-1];
                                        stack_parent.push(code_item);
                                    },
                                }
                            }

                        }
                    }


                    // adds the phrases to wherever they need to go
                    if phrase.len() > 0 {
                        // only does this if we have items in the phrase

                        if self.block_stack.len() > 0 {
                            // we have a stack, so add the items to the 
                            // points in the stack
                            let item_no = self.block_stack.len() - 1;
                            let (_, ref mut working_phrase) = self.block_stack[item_no];
                            working_phrase.append(&mut phrase);
                        } else {
                            // merges the items into the working_block
                            self.working_block.append(&mut phrase);
                        }    
                    }
                }
            }
        }

        println!("===");
        for p in &self.working_block {
            println!("{}",p.item());
        }

        Ok(self)
    }

    ///////////////////////////////////////////////////////////////////////////////////
    // PRIVATE FUNCTIONS //////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////////////

    fn get_next_working_phrase(&mut self) -> Option<Vec<CodeWrap<SyntaxElement>>> {
        //! pops the tokens from the scanner (stored in self.tokens) until it 
        //! gets to a EOL or EOF and then sets it as the working_phrase. will
        //! return true or false depending on the result
        //!
        //! if there are no tokens to return, it returns None

        let mut tokens : Vec<CodeWrap<SyntaxElement>> = Vec::new();

        loop {
            // checks that we still have tokens left to pop
            if self.tokens.len() == 0 { break; }

            match self.tokens.remove(0) {
                CodeWrap::CodeWrap(Token::EOL, _, _) | 
                CodeWrap::CodeWrap(Token::EOF, _, _) => break,
                CodeWrap::CodeWrap(token, start, end) => tokens.push(CodeWrap::CodeWrap(SyntaxElement::Token(token), start, end)),
            }

        }

        match tokens.len() {
            0 => None,
            _ => Some(tokens),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::token::Token;
    use crate::parser::Parser;
    use crate::scanner::Scanner;

    #[test]
    //#[ignore]
    pub fn quick_failure_to_see_parse() {
        let code = "do\nbob = 4 + -2\nend";

        match Scanner::from_str(&code,None).scan() {
            Err(error) => println!("{}",error),
            Ok(scanner) => { 
                match Parser::from_scanner(scanner).parse() {
                    Err(error) => println!("{}",error),
                    Ok(parser) => { },
                }
            },
        }

        assert!(false)
    }

}