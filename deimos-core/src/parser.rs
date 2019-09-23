use crate::error::CodeInformation;
use crate::token::Token;
use crate::codewrap::{CodeWrap, CodeWrappable};
use crate::scanner::{Scanner,TokenWrapped};
use crate::syntax::SyntaxElement;
use crate::parsererror::ParserError;

use failure::Error;

type Block = CodeWrap<SyntaxElement>;

pub struct Parser<'a> {
    pub file_name : String,
    pub raw_code : &'a str,
    pub blocks : Vec<Block>,

    // working 
    pub tokens : Vec<TokenWrapped>,
    pub current_pos : usize,
    pub working_phrase : Vec<Block>,
}

impl<'a> std::default::Default for Parser<'a> {
    fn default() -> Parser<'a> {
        Parser {
            raw_code : "",
            file_name : String::from("buffer"),
            blocks : Vec::new(),

            tokens : Vec::new(),
            current_pos : 0,
            working_phrase : Vec::new(),
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

        loop {
            match self.set_next_phrase() {
                false => break,
                true =>  loop {
                    // we loop as long as this can do something
                    // to it. reduce returns true if it actually
                    // reduces, so we break when it doesn't
                    if !self.reduce()? { break; }
                }
            }
            // START OF DELETE, DELETE BELOW
            // just for debugging and dev, delete this later
            // for p in &self.working_phrase { println!("{:?}",p); }
            for p in &self.working_phrase { println!("{}",p.item()); }
            // END OF THE DELETE, DELETE ABOVE
             
            // needs to check if we reduced it down to a block, every
            // single line MUST be a block, if we are not block then
            // there must have been some parsing error that we didn't
            // catch.
            match self.working_phrase.len() {
                1 => {
                    // we are expecting one element, so maybe we are ok
                    let syntax_element = self.working_phrase.remove(0);
                    if let CodeWrap::CodeWrap(SyntaxElement::Block(_),_,_) = syntax_element {
                        self.blocks.push(syntax_element);
                    } else {
                        // we don't have a block, then there is an error
                        return Err(ParserError::general_error(&self,
                            self.working_phrase[0].start(),
                            self.working_phrase[0].end(),
                            "all statements need to be reduced down to blocks"
                        ));
                    }
                },
                0 => {
                    // we have zero, not sure what happened but this shouldn't be possible.
                    return Err(ParserError::general_error(&self,
                        1,0,"lost our place parsing??? no tokens to reduce to blocks??"
                    ));
                },
                _ => {
                    // more than one, means we didn't do the reduction properly.
                    // if there was a syntax error it should have been called earlier,
                    // so this is the final catch, not really sure whats the problem but we can
                    // say something
                    return Err(ParserError::general_error(&self,
                        self.working_phrase[0].start(),
                        self.working_phrase[self.working_phrase.len()-1].end(),
                        &format!("could not reduce down to a single block (found {}), possible syntax error",self.working_phrase.len())
                    ));
                }
            }
        }

        Ok(self)
    }

    //////////////////////////////////////////////////////////////////////////
    // PRIVATE FUNCTIONS /////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////

    fn set_next_phrase(&mut self) -> bool {
        //! pops the tokens from the scanner (stored in self.tokens) until it 
        //! gets to a EOL or EOF and then sets it as the working_phrase. will
        //! return true or false depending on the result
        //!
        //! if there are no tokens to return, it returns None
        let mut tokens : Vec<CodeWrap<SyntaxElement>> = Vec::new();

        loop {
            // checks that we still have tokens, 
            // if we don't then we leave
            if self.tokens.len() == 0 { break; }

            // gets the next token, breaking up the CodeWrap
            let CodeWrap::CodeWrap(token, start, end) = self.tokens.remove(0);

            if token == Token::EOL || token == Token::EOF {
                // we are at the end of something, so we are done,
                break;
            }

            // if we didn't break (didn't find the end of something)
            // then we are now here
             
            // adds the token
            tokens.push(CodeWrap::CodeWrap(SyntaxElement::Token(token), start, end));
        }

        self.working_phrase = tokens;

        match self.working_phrase.len() {
            0 => false,
            _ => true,
        }
    }

    fn reduce(&mut self) -> Result<bool,Error> {
        //! will attempt to 'reduce' the list of tokens (or syntaxelement)
        //! to some kind of defined syntaxelement. It will consume edit
        //! the sent Vec<> and return a bool to whether it performed any
        //! changes or not
                
        let mut counter = 0;

        // check for chunk
        // check for block
        // check for statement
        counter = 0; loop { if SyntaxElement::process_statement(&mut self.working_phrase)? { counter += 1; }
                            else { if counter > 0 { return Ok(true); } break; }}
        // check for laststatement
        // check for funcname
        // check for varlist
        // check for var
        // check for namelist
        // check for explist
        if SyntaxElement::process_exp_list(&mut self.working_phrase)? { return Ok(true); }
        // check for expression
        counter = 0; loop { if SyntaxElement::process_exp(&mut self.working_phrase)? { counter += 1; }
                            else { if counter > 0 { return Ok(true); } break; }}
        // if SyntaxElement::process_exp(&mut self.working_phrase)? { return Ok(true); }
        // check for prefixexp
        // check for functioncall
        // check for args
        // check for function
        // check for functionbody
        // check for parlist
        // check for tableconstructor
        // check for field list
        // check for field

        Ok(false)
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
        let code = "bob = 3+-4";

        match Scanner::from_str(&code,None).scan() {
            Err(error) => println!("{}",error),
            Ok(scanner) => { 
                match Parser::from_scanner(scanner).parse() {
                    Err(error) => println!("{}",error),
                    Ok(parser) => println!("<done parsing>"),
                }
            },
        }

        assert!(false)
    }

}