use crate::error::CodeInformation;
use crate::token::Token;
use crate::codewrap::{CodeWrap, CodeWrappable};
use crate::scanner::{Scanner,TokenWrapped};
use crate::syntax::{
    exp, explist, statement, var, varlist,
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
}


impl<'a> std::default::Default for Parser<'a> {
    fn default() -> Parser<'a> {
        Parser {
            raw_code : "",
            file_name : String::from("buffer"),
            blocks : None,

            tokens : Vec::new(),

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
                        // check for statement
                        if Parser::smart_matcher(statement::process(&mut phrase))? { continue; };
                        // check for laststatement
                        // check for funcname
                        // check for varlist
                        if Parser::smart_matcher(varlist::process(&mut phrase))? { continue; };
                        // check for var
                        if Parser::smart_matcher(var::process(&mut phrase))? { continue; };
                        // check for namelist
                        // check for explist
                        if Parser::smart_matcher(explist::process(&mut phrase))? { continue; };
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

                    println!("==");
                    for p in phrase {
                        println!("{}",p.item());
                    }
                }
            }
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

    fn smart_matcher(result : SyntaxResult) -> Result<bool,Error> {
        //! used to wrap process statements that return a SyntaxResult
        //! its intent is to clean up the code a little and reuse this
        //! code here
        
        match result {
            SyntaxResult::Error(start,end,description) => println!("WE HAD ERROR: {} {} {}",start,end,description),
            SyntaxResult::Done => return Ok(true),
            SyntaxResult::None => { },
            SyntaxResult::More => { },
        }
        
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