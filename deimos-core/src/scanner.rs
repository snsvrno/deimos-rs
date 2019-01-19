use failure::{format_err,Error};

use crate::elements::{ Token, TokenType, CodeSlice };

pub struct Scanner<'a> {
    raw_code : &'a str,
    tokens : Vec<Token>,

    // for processing, unimportant otherwise
    start_pos : usize,
    current_line : usize,
    current_line_pos : usize,
    current_pos : usize,
}

impl<'a> std::default::Default for Scanner<'a> {
    fn default() -> Scanner<'a> {
        Scanner {
            raw_code : "",
            tokens : Vec::new(),
            start_pos : 0,
            current_line : 1,
            current_line_pos : 0,
            current_pos : 0,
        }
    }
}

impl<'a> Scanner <'a> {

    //////////////////////////////////////////////////////////////////////////////////
    // building functions

    pub fn init(code : &'a str) -> Scanner<'a> {
        Scanner {
            raw_code : code,
            .. Scanner::default()
        }
    }

    //////////////////////////////////////////////////////////////////////////////////
    // public functions

    pub fn scan(mut self) -> Result<Self,Error> {
        
        loop {
            
            let token = self.next_token()?;

            match token.get_type() {
                TokenType::EOF => {
                    self.tokens.push(token);
                    break;
                },
                TokenType::EOL => { 
                    self.current_line += 1;
                    self.current_line_pos = self.current_pos;
                },
                _ => (),
            }

            self.tokens.push(token);
        }

        Ok(self)
    }

    pub fn disassemble(self) -> (&'a str, Vec<Token>) {
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

        if !TokenType::valid_word_char(starter,true) { return None; }

        loop {
            if working_pos == self.raw_code.len() { break; }

            let n_char = &self.raw_code[working_pos .. working_pos + 1];

            match TokenType::valid_word_char(n_char,false) {
                false => break,
                true => {
                    // adds the character to the working word
                    working_pos += 1; 
                    word = format!("{}{}",word,n_char);
                },
            }
        }

        self.current_pos = working_pos;

        let keyword : TokenType = match TokenType::match_keyword(&word) {
            Some(t) => t,
            None => TokenType::Identifier(word.to_string())
        };

        Some(keyword)
    }

    fn peek_number(&mut self, starter : &str) -> Option<TokenType> {
        //! peek variant designed to match against numbers
        //! doesn't edit self unless it finds a match
        
        let mut working_pos = self.current_pos;
        let mut number : String = starter.to_string();
        let mut decimal_count = if starter == "." { 1 } else { 0 };

        if !TokenType::valid_number_char(&number) { return None; }

        loop {
            if working_pos == self.raw_code.len() { break; }

            let n_char = &self.raw_code[working_pos .. working_pos + 1];
            if n_char == "." { decimal_count += 1; }

            match TokenType::valid_number_char(n_char) {
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

    fn peek_comment(&mut self) -> Result<TokenType,Error> {

        let mut string_stream = String::new();
        let mut working_pos = self.current_pos;

        if self.peek("[[") {
            // a multi-line'd comment.

            // need to do this because when you peek and it matches
            // it will "consume" those tokens, so the global position now
            // has moved 2 characters to the right, but my local one (working_pos)
            // is locked before the peek.
            working_pos = self.current_pos;
            
            loop {
                // checks if we reach the end without finding the end comment
                if working_pos == self.raw_code.len() {
                    return Err(format_err!("Unterminated comment block, starting at :"));
                }

                let n_char = &self.raw_code[working_pos .. working_pos + 1];
                working_pos += 1;

                // end of the comment
                if n_char == "]" { 
                    if &self.raw_code[working_pos .. working_pos + 1] == "]" {
                        working_pos +=1;
                        break;
                    }
                } else {
                    string_stream = format!("{}{}",string_stream,n_char);
                }
            }
        } else {

            // a single line comment, ends with the line
            loop {
                if working_pos == self.raw_code.len() {
                    // the comment is on the last line, weird but ok. is it weird? IDK i don't do this.
                    break;
                }

                let n_char = &self.raw_code[working_pos .. working_pos + 1];

                if n_char == "\n" || n_char == "\r" {
                    // don't want to consume the new line.
                    break;
                } else {
                    working_pos += 1;
                    string_stream = format!("{}{}",string_stream,n_char);
                }
            }
        }
        
        self.current_pos = working_pos;
        Ok(TokenType::Comment(string_stream))
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
        // at the end of the file / code string
        if self.current_pos == self.raw_code.len() {
            let code_slice = CodeSlice::new(self.start_pos,self.current_pos,self.current_line,self.current_line_pos);
            return Ok(Token::new(TokenType::EOF,code_slice));
        }

        // gets the slice of the next char
        let char = &self.raw_code[self.current_pos .. self.current_pos + 1];
        self.start_pos = self.current_pos;
        self.current_pos +=1;

        let token = match char {
            "+" => TokenType::Plus,
            "-" => if self.peek("-") { self.peek_comment()? } else { TokenType::Minus },
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

        while sending_token.get_type() == &TokenType::WhiteSpace {
            sending_token = self.next_token()?;
        }

        Ok(sending_token)
    }

    //////////////////////////////////////////////////////////////////////////////////
    // access functions
    pub fn code(&self) -> &str {
        self.raw_code
    }
}

#[cfg(test)] 
mod tests {

    use crate::test_crate::*;

    #[test]
    fn simple_scan() {
        use crate::scanner::Scanner;
        
        let scanner = Scanner::init("5+5").scan().unwrap();
        assert_eq!(scanner.tokens,vec![
            token!("5"),token!("+"),token!("5"),token!("EOF")
        ]);

        assert_eq!("5",scanner.tokens[0].slice_code(&scanner.raw_code));
        assert_eq!("+",scanner.tokens[1].slice_code(&scanner.raw_code));
        assert_eq!("5",scanner.tokens[2].slice_code(&scanner.raw_code));

        assert_eq!(scanner.tokens.len(),4);
    }

    #[test]
    fn comments() {
        use crate::scanner::Scanner;

        let scanner = Scanner::init("--what is this going to do?
        -- this is another comment
        local bob = 12 + 2 -- some comment here
        
        --[[ mutli line comment
        with some stuf here
        local linda = 12+4
        while lind > 10 do
            linda = linda - 1
        end
        ]]
        
        jim = (12 * 3) + -34").scan().unwrap();

        assert_eq!(scanner.tokens.len(), 27);
        assert!(scanner.tokens[0] == comment_tt!(""));
        assert!(scanner.tokens[2] == comment_tt!(""));
        assert!(scanner.tokens[10] == comment_tt!(""));
        assert!(scanner.tokens[13] == comment_tt!(""));
    }

    #[test]
    fn complex_scan() {
        use crate::scanner::Scanner;
        
        let scanner = Scanner::init(r"
        local bob = 23;
        bob = bob + 4;
        do
            bob = bob + 1
        end

        if bob >= 10 then
            bob = -1233
        end
        ").scan().unwrap();
        
        // scanner doesn't know what to with ';' or '\n' so
        // it includes them all, the parser will process and 
        // remove them. the scanner does ignore whitespace.
        let token_stream = vec!["\n",
            "local","bob","=","23",";","\n",
            "bob","=","bob","+","4",";","\n",
            "do","\n",
            "bob","=","bob","+","1","\n",
            "end","\n","\n",
            "if","bob",">=","10","then","\n",
            "bob","=","-","1233","\n",
            "end","\n","EOF"
        ];


        assert_eq!(token_stream.len(),scanner.tokens.len());

        for i in 0 .. token_stream.len()-1 {
            assert_eq!(token_stream[i],scanner.tokens[i].slice_code(&scanner.raw_code));
            assert_eq!(token!(token_stream[i]),scanner.tokens[i])
        }
    }
}