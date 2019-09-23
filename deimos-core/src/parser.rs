use crate::error::CodeInformation;
use crate::token::Token;
use crate::codewrap::{CodeWrap, CodeWrappable};
use crate::scanner::{Scanner,TokenWrapped};
use crate::language::SyntaxElement;

use failure::Error;

type Block = CodeWrap<SyntaxElement>;

pub struct Parser<'a> {
    pub file_name : String,
    pub raw_code : &'a str,
    pub blocks : Vec<Block>,

    // working 
    pub tokens : Vec<TokenWrapped>,
    pub current_pos : usize,
}

impl<'a> std::default::Default for Parser<'a> {
    fn default() -> Parser<'a> {
        Parser {
            raw_code : "",
            file_name : String::from("buffer"),
            blocks : Vec::new(),

            tokens : Vec::new(),
            current_pos : 0,
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
            let phrase : Option<Vec<CodeWrap<SyntaxElement>>> = self.get_next_phrase();
            match phrase {
                None => break,
                Some(mut phrase) => {
                    loop {
                        // we loop as long as this can do something
                        // to it.
                        if !SyntaxElement::reduce(&mut phrase) {
                            break;
                        }
                    }

                    for p in phrase {
                        println!("{}",p.item());
                    }

                }
            }
        }

        Ok(self)
    }

    pub fn get_next_phrase(&mut self) -> Option<Vec<CodeWrap<SyntaxElement>>> {
        //! pops the tokens from the scanner (stored in self.tokens) until it 
        //! gets to a EOL or EOF and then returns that set.
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
        let code = "bob = 4 + 2";

        match Scanner::from_str(&code,None).scan() {
            Err(error) => println!("{}",error),
            Ok(scanner) => { 
                match Parser::from_scanner(scanner).parse() {
                    Err(error) => println!("{}",error),
                    Ok(parser) => println!("done"),
                }
            },
        }

        assert!(false)
    }

}