use crate::token::Token;
use crate::tokentype::TokenType;
use crate::codeslice::CodeSlice;

use failure::{Error,format_err};

/// scanner that converts raw source code into 
/// tokens
pub struct Scanner<'a> {
    raw_code : &'a str,

    tokens : Vec<Token>,

    start_pos : usize,
    current_line : usize,
    current_line_pos : usize,
    current_pos : usize,
}

impl<'a> Scanner<'a> {
    pub fn new(code : &'a str) -> Scanner<'a> {
        //! creates a new scanner object from a &str of code
        
        Scanner {
            raw_code : code,

            tokens : Vec::new(),

            start_pos : 0,
            current_line : 1,
            current_line_pos : 0,
            current_pos : 0,
        }
    }

    pub fn scan(mut self) -> Result<Self,Error> {
        //! runs the scan, and returns itself. used as a chained
        //! operator after creating a new scanner so mut isn't needed.
        //! 
        //! ```rust,ignore
        //! # extern crate lua_interpreter;
        //! # use lua_interpreter::scanner::Scanner;
        //! // prefered to use .scan()? to pass the error onward, or you can use match
        //! // to capture the error and repeat it.
        //! let scanner = Scanner::new("some code").scan().expect("scanner failed");
        //! ```
        //! 
        //! Errors from `scan()` will be lexiconical errors from bad lua code.

        loop {
            let token = self.next_token()?;

            // println!("{:?}",token);

            if token == TokenType::EOL { 
                self.current_line += 1;
                self.current_line_pos = self.current_pos;
            }
            if token == TokenType::EOF { break; }

            self.tokens.push(token);
        }

        Ok(self)
    }
    
    pub fn explode(self) -> (&'a str, Vec<Token>) {
        (self.raw_code,self.tokens)
    }

    //////////////////////////////////////////////////////////////////////////////////
    /// INTERNAL PRIVATE FUNCTIONS /////////////////////////////////////

    fn peek(&mut self, chars : &str) -> bool {
        //! looks for the next characters provided in the stream, if found
        //! then it will consume them and return true, if not then it will
        //! return false and not do anything
        
        let length = chars.len();

        if self.raw_code.len() < self.current_pos + length {
            return false 
        }

        for i in 0 .. length {
            let char = &self.raw_code[self.current_pos + i .. self.current_pos + i + 1];
            let to_match = &chars[i .. i + 1];

            if char != to_match { return false; }
        }

        self.current_pos += length;
        true
    }

    fn peek_keyword(&mut self, starter : &str) -> Option<TokenType> {
        //! peek variant designed to match against keywords and words
        //! doesn't edit self unless it finds a match
         
        let mut working_pos = self.current_pos;
        let mut word : String = starter.to_string();

        if !Token::valid_word_char(starter,true) { return None; }

        loop {
            if working_pos == self.raw_code.len() { break; }

            let n_char = &self.raw_code[working_pos .. working_pos + 1];

            match Token::valid_word_char(n_char,false) {
                false => break,
                true => {
                    // adds the character to the working word
                    working_pos += 1; 
                    word = format!("{}{}",word,n_char);
                },
            }
        }

        self.current_pos = working_pos;

        let keyword : TokenType = match TokenType::match_keyword(word.as_str()) {
            Some(token) => token,
            None => TokenType::Identifier(word.to_string()),
        };

        Some(keyword)
    }

    fn peek_number(&mut self, starter : &str) -> Option<TokenType> {
        //! peek variant designed to match against numbers
        //! doesn't edit self unless it finds a match
        
        let mut working_pos = self.current_pos;
        let mut number : String = starter.to_string();
        let mut decimal_count = if starter == "." { 1 } else { 0 };

        if !Token::valid_number_char(&number) { return None; }

        loop {
            if working_pos == self.raw_code.len() { break; }

            let n_char = &self.raw_code[working_pos .. working_pos + 1];
            if n_char == "." { decimal_count += 1; }

            match Token::valid_number_char(n_char) {
                false => break,
                true => {
                    // adds the character to the working word
                    working_pos += 1; 
                    number = format!("{}{}",number,n_char);
                },
            }
        }

        if decimal_count > 1 { return None; }

        match number.parse::<f32>() {
            Err(_) => None,
            Ok(number) =>  {
                self.current_pos = working_pos;
                Some(TokenType::Number(number)) 
            },
        }
    }

    fn peek_string(&mut self, starter : &str) -> Result<TokenType,Error> {
        //! peek variant designed to match against strings
        //! doesn't edit self unless it finds a match
        //! 
        //! will error if cannot find the end of a string literal (no close)
        
        let mut working_pos = self.current_pos;
        let mut string : String = "".to_string();

        loop {
            if working_pos == self.raw_code.len() {
                return Err(format_err!("Reached EOF while processing String at {}:{}",self.current_line,self.start_pos));
            }

            let n_char = &self.raw_code[working_pos .. working_pos + 1];
            working_pos += 1; 
            
            if n_char == starter {
                self.current_pos = working_pos;
                return Ok(TokenType::String(string));
            } else {
                string = format!("{}{}",string,n_char);
            }

        }
    }

    fn next_token(&mut self) -> Result<Token,Error> {
        //! gets the next token in the series, used internally
        
        // at the end of the file / code string
        if self.current_pos == self.raw_code.len() {
            return Ok(Token::simple(TokenType::EOF));
        }
        
        // gets the slice of the next char
        let char = &self.raw_code[self.current_pos .. self.current_pos + 1];
        self.start_pos = self.current_pos;
        self.current_pos +=1;

        let token = match char {
            "+" => TokenType::Plus,
            "-" => TokenType::Minus,
            "*" => TokenType::Star,
            "/" => TokenType::Slash,
            "%" => TokenType::Percent,
            "^" => TokenType::Carrot,
            "#" => TokenType::Pound,
            "<" => if self.peek("=") { TokenType::LessEqual } else { TokenType::LessThan },
            ">" => if self.peek("=") { TokenType::GreaterEqual } else { TokenType::GreaterThan },
            "=" => if self.peek("=") { TokenType::EqualEqual } else { TokenType::Equal },
            "(" => TokenType::LeftParen,
            ")" => TokenType::RightParen,
            "[" => TokenType::LeftBracket,
            "]" => TokenType::RightBracket,
            "{" => TokenType::LeftMoustache,
            "}" => TokenType::RightMoustache,
            ";" => TokenType::SemiColon,
            ":" => TokenType::Colon,
            "," => TokenType::Comma,
            "." => if self.peek("..") { TokenType::TriplePeriod } else if self.peek(".") { TokenType::DoublePeriod } else { TokenType::Period },
            "~" => if self.peek("=") { TokenType::NotEqual } else { return Err(format_err!("Illegal character '{}' found at {}:{}",char,self.current_line,self.current_pos)) },
            
            "\"" => self.peek_string("\"")?,
            "'" => self.peek_string("'")?,
            
            "\n" => { /* self.peak("\r"); */TokenType::EOL },
            "\r" => { /* self.peak("\n"); */TokenType::EOL },
            " " => TokenType::WhiteSpace,
            
            _ => match self.peek_keyword(char) {
                Some(keyword) => keyword,
                None => match self.peek_number(char) {
                    Some(number) => number,
                    None => return Err(format_err!("Illegal character '{}' found at {}:{}",char,self.current_line,self.current_pos)),
                },
            },
        };

        let mut sending_token = Token::new(
            token,
            CodeSlice::new(self.start_pos,self.current_pos,self.current_line,self.current_line_pos)
        );

        while sending_token == TokenType::WhiteSpace {
            sending_token = self.next_token()?;
        }

        Ok(sending_token)
    }
}

//////////////////////////////////////////////////////////////////////////////////
// TESTING MACROS ///////////////////////////////

#[macro_export]
macro_rules! assert_scanner {
    ($scanner:expr,$($checker:expr),*) => {
        match $scanner.scan() {
            Err(error) => panic!("\n{}",error),
            Ok(scanner) => {
                let vec = vec![$($checker),*];
                let length = vec.len();

                if length != scanner.tokens.len() {
                    panic!("\n\nCode Segment:\n{}\n\nNumber of Tokens ({}) doesn't match number of Checkers ({}) provided. \n  Tokens : {:?} \n  Checkers : {:?}",
                        scanner.raw_code,
                        scanner.tokens.len(),
                        length,
                        scanner.tokens,
                        vec
                    );
                }

                for i in 0 .. length {
                    if scanner.tokens[i] != vec[i] {
                        panic!("\n\nCode Segment:\n{}\n\nToken #{} doesn't match.\n  Result: {:?}\n  Expected: {:?}",
                            scanner.raw_code,
                            i,
                            scanner.tokens[i].get_type(),
                            vec[i]
                        );
                    }
                }
            }
        }
    };
}

mod tests {

    #[test]
    pub fn basic_token_matching() {
        //! tests the token scanning, making sure if we pass a string of the 
        //! exact token it will correctly identify which token we want.
        
        use crate::scanner::Scanner;
        use crate::tokentype::TokenType;

        //////////////////////////////////////////////////////////////
        // +     -     *     /     %     ^     #
        assert_scanner!(Scanner::new("+"),TokenType::Plus);
        assert_scanner!(Scanner::new("-"),TokenType::Minus);
        assert_scanner!(Scanner::new("*"),TokenType::Star);
        assert_scanner!(Scanner::new("/"),TokenType::Slash);
        assert_scanner!(Scanner::new("%"),TokenType::Percent);
        assert_scanner!(Scanner::new("^"),TokenType::Carrot);
        assert_scanner!(Scanner::new("#"),TokenType::Pound);
        // ==    ~=    <=    >=    <     >     =
        assert_scanner!(Scanner::new("=="),TokenType::EqualEqual);
        assert_scanner!(Scanner::new("~="),TokenType::NotEqual);
        assert_scanner!(Scanner::new("<="),TokenType::LessEqual);
        assert_scanner!(Scanner::new(">="),TokenType::GreaterEqual);
        assert_scanner!(Scanner::new("<"),TokenType::LessThan);
        assert_scanner!(Scanner::new(">"),TokenType::GreaterThan);
        assert_scanner!(Scanner::new("="),TokenType::Equal);
        // (     )     {     }     [     ]
        assert_scanner!(Scanner::new("("),TokenType::LeftParen);
        assert_scanner!(Scanner::new(")"),TokenType::RightParen);
        assert_scanner!(Scanner::new("{"),TokenType::LeftMoustache);
        assert_scanner!(Scanner::new("}"),TokenType::RightMoustache);
        assert_scanner!(Scanner::new("["),TokenType::LeftBracket);
        assert_scanner!(Scanner::new("]"),TokenType::RightBracket);
        // ;     :     ,     .     ..    ...
        assert_scanner!(Scanner::new(";"),TokenType::SemiColon);
        assert_scanner!(Scanner::new(":"),TokenType::Colon);
        assert_scanner!(Scanner::new(","),TokenType::Comma);
        assert_scanner!(Scanner::new("."),TokenType::Period);
        assert_scanner!(Scanner::new(".."),TokenType::DoublePeriod);
        assert_scanner!(Scanner::new("..."),TokenType::TriplePeriod);
 
        //////////////////////////////////////////////////////////////       
        // and       break     do        else      elseif
        assert_scanner!(Scanner::new("and"),TokenType::And);
        assert_scanner!(Scanner::new("break"),TokenType::Break);
        assert_scanner!(Scanner::new("do"),TokenType::Do);
        assert_scanner!(Scanner::new("else"),TokenType::Else);
        assert_scanner!(Scanner::new("elseif"),TokenType::Elseif);
        // end       false     for       function  if
        assert_scanner!(Scanner::new("end"),TokenType::End);
        assert_scanner!(Scanner::new("false"),TokenType::False);
        assert_scanner!(Scanner::new("for"),TokenType::For);
        assert_scanner!(Scanner::new("function"),TokenType::Function);
        assert_scanner!(Scanner::new("if"),TokenType::If);
        // in        local     nil       not       or
        assert_scanner!(Scanner::new("in"),TokenType::In);
        assert_scanner!(Scanner::new("local"),TokenType::Local);
        assert_scanner!(Scanner::new("nil"),TokenType::Nil);
        assert_scanner!(Scanner::new("not"),TokenType::Not);
        assert_scanner!(Scanner::new("or"),TokenType::Or);
        // repeat    return    then      true      until     while
        assert_scanner!(Scanner::new("repeat"),TokenType::Repeat);
        assert_scanner!(Scanner::new("return"),TokenType::Return);
        assert_scanner!(Scanner::new("then"),TokenType::Then);
        assert_scanner!(Scanner::new("true"),TokenType::True);
        assert_scanner!(Scanner::new("until"),TokenType::Until);
        assert_scanner!(Scanner::new("while"),TokenType::While);

        //////////////////////////////////////////////////////////////
        // strings
        assert_scanner!(Scanner::new("\"ashortstring\""),TokenType::String("".to_string()));
        assert_scanner!(Scanner::new("\"a longer string\""),TokenType::String("".to_string()));

        //////////////////////////////////////////////////////////////
        // numbers
        assert_scanner!(Scanner::new("123"),TokenType::Number(123 as f32));
        assert_scanner!(Scanner::new("0.123"),TokenType::Number(0.123));
        assert_scanner!(Scanner::new(".123"),TokenType::Period,TokenType::Number(123 as f32));
        assert_scanner!(Scanner::new("1.23"),TokenType::Number(1.23));
        assert_scanner!(Scanner::new("123."),TokenType::Number(123 as f32));

        //////////////////////////////////////////////////////////////
        // variable names (assignment)
        assert_scanner!(Scanner::new("local bob = 23"),
            TokenType::Local,
            TokenType::Identifier("bob".to_string()),
            TokenType::Equal,
            TokenType::Number(23 as f32)
        );
    }

}