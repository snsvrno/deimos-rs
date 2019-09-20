use failure::Error;

use crate::token::Token;
use crate::scannererror::ScannerError;

pub struct Scanner<'a> {
    pub file_name : String,
    pub raw_code : &'a str,
    pub tokens : Vec<Token>,

    // working stuff, is public for error handling
    pub current_pos : usize,
    pub line_number : usize,
} 

impl<'a> std::default::Default for Scanner<'a> {
    fn default() -> Scanner<'a> {
        Scanner {
            raw_code : "",
            file_name : String::from("buffer"),
            tokens : Vec::new(),

            // working stuff
            current_pos : 0,
            line_number : 0
        }
    }
}

impl<'a> Scanner<'a> {

    // PUBLIC FUNCTIONS ///////////////////////

    pub fn init(code : &'a str) -> Scanner<'a> {
        //! creates a scanner object, used to scan the 
        //! raw code
        
        Scanner {
            raw_code : code,
            .. Scanner::default()
        }
    }

    pub fn scan(mut self) -> Result<Self,Error> {
        //! does the scanning, goes through the code and 
        //! breaks it up to valid tokens
        
        loop {
            match self.scan_next_token()? {
                Token::EOF => break,
                token => self.tokens.push(token),
            }
        }

        Ok(self)
    }

    // PRIVATE FUNCTIONS ///////////////////////

    fn scan_next_token(&mut self) -> Result<Token,Error> {
        //! scans the raw code and returns the next token,
        //! will return an error if it cannot build a token
        
        // checks if we are at the end of the code,
        // so that we don't access a slice that doesn't exist
        if self.current_pos == self.raw_code.len() {
            return Ok(Token::EOF);
        }

        // some local stuff for metadata on each token
        let token_start : usize = self.current_pos;

        // gets the slice of the next character
        let char = &self.raw_code[self.current_pos .. self.current_pos + 1];
        self.current_pos +=1;

        // determines what the token could possibly be
        let token = match char {
            "+" => Token::Plus,
            "-" => if self.scan_peek("-") { self.scan_token_comment()? } 
                   else { Token::Minus },
            "*" => Token::Star,
            "/" => Token::Slash,
            "%" => Token::Percent,
            "^" => Token::Carrot,
            "#" => Token::Pound,
            "<" => if self.scan_peek("=") { Token::LessEqual } 
                   else { Token::LessThan },
            ">" => if self.scan_peek("=") { Token::GreaterEqual } 
                   else { Token::GreaterThan },
            "=" => if self.scan_peek("=") { Token::EqualEqual } 
                   else { Token::Equal },
            "(" => Token::LeftParen,
            ")" => Token::RightParen,
            "[" => if let Some(level) = self.scan_peek_multiline_string() { self.scan_token_multiline_string(level)? } 
                   else { Token::LeftBracket },
            "]" => Token::RightBracket,
            "{" => Token::LeftMoustache,
            "}" => Token::RightMoustache,
            ";" => Token::SemiColon,
            ":" => Token::Colon,
            "," => Token::Comma,
            "." => if self.scan_peek("..") { Token::TriplePeriod } 
                   else if self.scan_peek(".") { Token::DoublePeriod } 
                   else if let Some(num) = self.scan_peek_token_number(".")? { num }
                   else { Token::Period },
            "~" => if self.scan_peek("=") { Token::NotEqual } 
                   else { return Err(ScannerError::illegal_character(self)) },
            "\"" => self.scan_token_string("\"")?,
            "'" => self.scan_token_string("'")?,

            " " => { self.scan_peek_all(" "); Token::WhiteSpace },

            char => if Token::is_eol(char) { Token::EOL } 
                    else { 
                        // the catch all part, this needs to check if its a number, string, or identifier
                        match self.scan_peek_token_keyword(char) {
                            Some(keyword) => keyword,
                            None => match self.scan_peek_token_number(char)? {
                                Some(number) => number,
                                None => return Err(ScannerError::illegal_character(self)),
                            }
                        }
                    },
        };

        Ok(token)
    }

    fn scan_peek(&mut self, chars : &str) -> bool {
        //! looks for the next characters provided in the stream, if found
        //! then it will consume them and return true, if not then it will
        //! return false and not do anything
         
        // determines what the search length should be.
        let length = chars.len();


        // checks we are trying to look past whats left in the 
        // raw code, if so then we won't find it because nothing
        // exists in the void after our code, so lets just return
        // false
        if self.raw_code.len() - 1 < self.current_pos + length {
            return false;
        }

        // now will look in only the length of the searching characters, 
        // most of the time this will probably be `1`
        for i in 0 .. length {
            // get the next character slice
            let char = &self.raw_code[self.current_pos + i .. self.current_pos + i + 1];

            // checks if its what we expect so far we will keep doing 
            // this until we hit a point where it doesn't match, 
            // and that means its not the characters we are looking for
            if char != &chars[i .. i + 1] {
                return false;
            }
        }

        // we found what we were looking for, so move the cursor 
        // we don't return the characters because we are counting on
        // the fact that where this is being used will replace them, since
        // we are consuming them by moving the counter forward.
        self.current_pos += length;
        true
    }

    fn scan_peek_all(&mut self, char : &str) -> bool {
        //! a peek that will consume all of the characters, use this 
        //! in places where it doesn't matter how many of these characters
        //! exist, like WHITESPACE or EOL
        
        let mut count_found : usize = 0;
        while self.scan_peek(char) { count_found += 1; }

        if count_found > 0 { true } else { false }
    }

    fn scan_peek_token_keyword(&mut self, first : &str) -> Option<Token> {
        //! acts like peek, where it moves the cursor if it finds what it wants, but 
        //! also returns the token that it finds
        
        if !Token::is_valid_word_char(first,true) { return None; }

        let mut pos = self.current_pos;
        let mut word : String = first.to_string();

        loop {
            // checks we are trying to look past whats left in the 
            // raw code, if so then we won't find it because nothing
            // exists in the void after our code, so lets just return
            // false
            if self.raw_code.len() - 1 < pos {
                break;
            }

            let char = &self.raw_code[pos .. pos + 1];

            match Token::is_valid_word_char(char,false) {
                false => break,
                true => {
                    pos += 1;
                    word = format!("{}{}",word,char);
                }
            }
        }

        self.current_pos = pos;

        let token : Token = match Token::match_keyword(&word) {
            Some(token) => token,
            None => Token::Identifier(word.to_string()),
        };

        Some(token)
    }

    fn scan_peek_token_number(&mut self, first : &str) -> Result<Option<Token>,Error> {
        //! acts like peek, where it moves the cursor if it finds what it wants, but 
        //! also returns the token that it finds
        
        if !Token::is_valid_number_char(&first) { return Ok(None); }

        let mut pos = self.current_pos;
        let mut number : String = first.to_string();
        // need to do this because rust will have a stack overflow if 
        // you try to parse a string as a float with more than 1 decimal
        let mut decimal_number : usize = if first == "." { 1 } else { 0 };

        loop {

            // checks we are trying to look past whats left in the 
            // raw code, if so then we won't find it because nothing
            // exists in the void after our code, so lets just return
            // false
            if self.raw_code.len() - 1 < pos {
                break;
            }

            let char = &self.raw_code[pos .. pos + 1];
            if char == "." { decimal_number += 1; }

            match Token::is_valid_number_char(char) {
                false => break,
                true => {
                    // adds the character to the working word
                    pos += 1; 
                    number = format!("{}{}",number,char);
                },
            }
        }

        if decimal_number > 1 {
            return Err(ScannerError::number_parsing(self));
        }

        if number == "." { return Ok(None); }

        match number.parse::<f32>() {
            Err(error) => Err(ScannerError::number_parsing(self)),
            Ok(num) =>  {
                self.current_pos = pos;
                Ok(Some(Token::Number(num)))
            },
        }
    }

    fn scan_peek_multiline_string(&mut self) -> Option<usize> {
        //! checks if the next few characters defines a multiline string 
        //! using the `[==[` format where the number of `=` is the level
        //! 
        //! per the manual https://www.lua.org/manual/5.1/manual.html#2.1
        //!  
        //!   Literal strings can also be defined using a long 
        //!   format enclosed by long brackets. We define an 
        //!   opening long bracket of level n as an opening 
        //!   square bracket followed by n equal signs followed 
        //!   by another opening square bracket. So, an opening 
        //!   long bracket of level 0 is written as [[, an opening 
        //!   long bracket of level 1 is written as [=[, and so 
        //!   on. A closing long bracket is defined similarly; 
        //!   for instance, a closing long bracket of level 4 
        //!   is written as ]====]. A long string starts with 
        //!   an opening long bracket of any level and ends at 
        //!   the first closing long bracket of the same level. 
        //!   Literals in this bracketed form can run for several 
        //!   lines, do not interpret any escape sequences, and 
        //!   ignore long brackets of any other level. They can 
        //!   contain anything except a closing bracket of the 
        //!   proper level.
        //! 
        //! the level is required for the consuming function later to 
        //! know when to correctly end the string
        
        let mut working_pos = self.current_pos;
        let mut level = 0;

        loop {

            // checks we are trying to look past whats left in the 
            // raw code, if so then we won't find it because nothing
            // exists in the void after our code, so lets just return
            // false
            if self.raw_code.len() - 1< working_pos {
                return None;
            }

            // gets the next character
            let char = &self.raw_code[working_pos .. working_pos + 1];
            working_pos += 1;

            // the only 2 valid characters are 
            //      `=` which is the level
            //      `[` which ends the front of the string marker
            match char {
                "=" => level += 1,
                "[" => break,
                _ => return None,
            }

        }
        
        self.current_pos = working_pos;
        Some(level)
    }

    fn scan_token_string(&mut self, starter : &str) -> Result<Token,Error> {
        //! will assume we are on a string and attempt to find the ending
        //! of that string. doesn't do checking to make sure we are in 
        //! a string but will error if it can't find the end
        //! 
        //! handles ' and " strings currently
        //! 
        //! - starter : expecting a string of len 1, will not work otherwise

        let mut string : String = String::new();

        loop {
            // checks if we reached the end of the code without the comment close
            if self.current_pos == self.raw_code.len() {
                return Err(ScannerError::unterminated_code_segment(self,1,1,"string not terminated"));  
            }

            // the next character
            let char = &self.raw_code[self.current_pos .. self.current_pos + 1];
            self.current_pos += 1;

            match char == starter {
                false => string = format!("{}{}",string,char),
                true => return Ok(Token::String(string)),
            } 
        }
    }

    fn scan_token_multiline_string(&mut self, level : usize) -> Result<Token,Error> {
        //! will act as the rest of what we are getting is a comment
        //! this doesn't do any checking because it assumes you did a peek check
        //! that there is actually a comment
        //! 
        //! Handles the `[==[` style comment where the number of `=` is the level
        
        // creates the string of ending characters so we know what to look for.
        let ending_chars : String = {
            let mut string = String::from("]");

            for i in 0 .. level {
                string = format!("{}{}","=",string);
            }

            string // should look like `==]` where the number of `=` is the level
        };

        let mut string : String = String::new();

        loop {

            // checks if we reached the end of the code without the comment close
            if self.current_pos == self.raw_code.len() {
                return Err(ScannerError::unterminated_code_segment(self,level+2,level+2,"multiline comment has no end, starts here"));  
            }

            let char = &self.raw_code[self.current_pos .. self.current_pos + 1];
            self.current_pos += 1;

            match char {
                "]" => if self.scan_peek(&ending_chars) { break; } 
                       else { string = format!("{}{}",string,char); },
                _ => string = format!("{}{}",string,char)
            }
        }

        Ok(Token::MultiLineString(string))
    }

    fn scan_token_comment(&mut self) -> Result<Token,Error> {
        //! will act as the rest of what we are getting is a comment
        //! this doesn't do any checking because it assumes you did a peek check
        //! that there is actually a comment
        //! 
        //! Handles both `--` and `--[[ ]]` comments
         
        let mut string = String::new();
        
        // first we need to know what kind of comment we are working with
        if self.scan_peek("[[") {
            // looks like its a long comments
            
            loop {
                // checks if we reached the end of the code without the comment close
                if self.current_pos == self.raw_code.len() {
                    return Err(ScannerError::unterminated_code_segment(self,2,2,"long comment has no end"));  
                }

                // gets the next character
                let char = &self.raw_code[self.current_pos .. self.current_pos + 1];
                self.current_pos += 1;

                match char {
                    "]" => if self.scan_peek("]") { break; /* we found the end of the comment */ } 
                           else { string = format!("{}{}",string,"]"); /* we didn't find the end, so still record it */ },
                    c => string = format!("{}{}",string,c),
                }

            }
        } else {
            // we have the simple comment, which terminates at the end of the line
            
            loop {
                // check if we are at the end of the code
                if self.current_pos == self.raw_code.len() { break; }

                let char = &self.raw_code[self.current_pos .. self.current_pos + 1];
                match Token::is_eol(char) {
                    true => break, // we don't want to consume an EOL token in a simple comment
                    false => {
                        self.current_pos += 1;
                        string = format!("{}{}",string,char);
                    }
                }
            }
        }

        Ok(Token::Comment(string))  
    }
}   


#[cfg(test)]
mod tests {
    use crate::token::Token;
    use crate::scanner::Scanner;

    fn tokens_type(t1 : &Token, t2 : &Token) -> bool {
        //! testing helping function that lets use check token types
        //! easier, and displays them when it fails assertion
        
        if !t1.is_same_type(t2) {
            println!("tokens are not the same type\n  left:  {:?}\n  right: {:?}",t1,t2);
            return false;
        } 

        true
    }

    #[test]
    fn scanning_string() {
        let code : String = String::from(
            "\"string1\"'string number 2'[====[another string,\n\n\n longer \"which stuff\" in it]====]"
        );

        let scanner = Scanner::init(&code).scan();

        match scanner {
            Ok(scanner) => {
                assert!(tokens_type(&Token::String("".to_string()), &scanner.tokens[0]));
                assert!(tokens_type(&Token::String("".to_string()), &scanner.tokens[1]));
                assert!(tokens_type(&Token::MultiLineString("".to_string()), &scanner.tokens[2]));

                assert_eq!("string1",&scanner.tokens[0].inner_text());
                assert_eq!("string number 2",&scanner.tokens[1].inner_text());
                assert_eq!("another string,\n\n\n longer \"which stuff\" in it",&scanner.tokens[2].inner_text());
            },
            Err(error) => println!("{}",error),
        }
    }

    #[test]
    fn scanning_numbers() {
        let code : String = String::from("1 2 3 4 5 0.1233 .1232 123.3");
        let scanner = Scanner::init(&code).scan();

        match scanner {
            Ok(scanner) => {
                assert_eq!(scanner.tokens[0],Token::Number(1.0));
                assert_eq!(scanner.tokens[2],Token::Number(2.0));
                assert_eq!(scanner.tokens[4],Token::Number(3.0));
                assert_eq!(scanner.tokens[6],Token::Number(4.0));
                assert_eq!(scanner.tokens[8],Token::Number(5.0));
                assert_eq!(scanner.tokens[10],Token::Number(0.1233));
                assert_eq!(scanner.tokens[12],Token::Number(0.1232));
                assert_eq!(scanner.tokens[14],Token::Number(123.3));
            },
            Err(error) => println!("{}",error),
        }


        let failing_code : String = String::from("123.2.3");
        let scanner = Scanner::init(&failing_code).scan();

        assert!(scanner.is_err());
    }

    #[test]
    fn small_code_sample() {
        let code : String = String::from(r#"
            -- taken from my old pixelscreen-love library
            local PIXELSCREEN = { }

            PIXELSCREEN.primeInstance = nil

            function PIXELSCREEN:new(overrides)
              local ps = { }
              setmetatable(ps, self)
              self.__index = self
              ps:load(overrides)
              return ps
            end
        "#);

        let scanner = Scanner::init(&code).scan();
        let tokens : Vec<Token> = vec![Token::EOL,
        /* 1  */    Token::WhiteSpace, Token::Comment(" taken from my old pixelscreen-love library".to_string()), Token::EOL,
        /* 4  */    Token::WhiteSpace, Token::Local, Token::WhiteSpace, Token::Identifier("PIXELSCREEN".to_string()),
        /* 8  */        Token::WhiteSpace, Token::Equal, Token::WhiteSpace, Token::LeftMoustache, Token::WhiteSpace, 
        /* 13 */        Token::RightMoustache, Token::EOL,
        /* 15 */    Token::EOL,
        /* 16 */    Token::WhiteSpace, Token::Identifier("PIXELSCREEN".to_string()), Token::Period, 
        /* 19 */        Token::Identifier("primeInstance".to_string()), Token::WhiteSpace, Token::Equal, Token::WhiteSpace, 
        /* 23  */       Token::Nil, Token::EOL  
        ];
        // got tired and didn't do any more...

        match scanner {
            Err(error) => {
                println!("{}",error);
                assert!(false);
            },
            Ok(scanner) => {
                println!("{:?}",scanner.tokens);

                for i in 0 .. tokens.len() {
                    println!("{}",i);
                    assert_eq!(tokens[i],scanner.tokens[i]);
                }
            },
        }
    }

    #[test]
    fn scanning_comments() {
       
        {
            // single line simple
            let code : String = String::from("--this is an example code");
            let scanner = Scanner::init(&code).scan();

            match scanner {
                Ok(scanner) => {
                    assert!(tokens_type(&Token::Comment("".to_string()), &scanner.tokens[0]));
                    assert_eq!("this is an example code",&scanner.tokens[0].inner_text());
                },
                Err(error) => println!("{}",error),
            }
        }

        {
            // single line two of em
            let code : String = String::from("--[[ annother comment here ]]--this is an example code");
            let scanner = Scanner::init(&code).scan();

            match scanner {
                Ok(scanner) => {
                    assert!(tokens_type(&Token::Comment("".to_string()), &scanner.tokens[0]));
                    assert!(tokens_type(&Token::Comment("".to_string()), &scanner.tokens[1]));
                    assert_eq!("this is an example code",&scanner.tokens[1].inner_text());
                    assert_eq!(" annother comment here ",&scanner.tokens[0].inner_text());
                },
                Err(error) => println!("{}",error),
            }
        }

        {
            // multiline
            let code : String = String::from("--[[ annother\n\n\n\n comment here ]]");
            let scanner = Scanner::init(&code).scan();

            match scanner {
                Ok(scanner) => {
                    assert!(tokens_type(&Token::Comment("".to_string()), &scanner.tokens[0]));
                    assert_eq!(" annother\n\n\n\n comment here ",&scanner.tokens[0].inner_text());
                },
                Err(error) => println!("{}",error),
            }
        }
    }
}