use failure::Error;
use crate::token::{CodeToken, Token};
use crate::error::{
    codeinfo::CodeInformation,
    scanner::ScannerError,};
use crate::coderef::CodeRef::CodeRef;

pub struct Scanner<'a> {
    pub file_name : String,
    pub raw_code : &'a str,
    pub tokens : Vec<CodeToken>,

    // private things
    cursor_pos : usize,
    line_number : usize,
}

impl<'a> std::default::Default for Scanner<'a> {
    fn default() -> Scanner<'a> {
        Scanner {
            raw_code : "",
            file_name : String::from("buffer"),
            tokens : Vec::new(),

            cursor_pos : 0,
            line_number : 1,
        }
    }
}

impl<'a> CodeInformation for Scanner<'a> {
    fn raw_code(&self) -> String { self.raw_code.to_string() }
    fn cursor_pos(&self) -> usize { self.cursor_pos }
    fn file_name(&self) -> String { self.file_name.to_string() }
    fn line_number(&self) -> usize { self.line_number }
}

impl<'a> Scanner<'a> {
    pub fn from_str(raw_code : &'a str, file_name : Option<&str>) -> Result<Scanner<'a>,Error> {
        //! creates a scanner object from a string of code, optionally
        //! you can give it a file_name so the errors will tie back to
        //! a file_name.
        
        let scanner = Scanner {
            file_name : if let Some(name) = file_name { name.to_string() } else { String::new() },
            raw_code : raw_code,
            .. Scanner::default()
        };

        scanner.scan()
    }

    // PRIVATE FUNCTIONS /////////////////////////////////////
    //////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////

    fn scan(mut self) -> Result<Self,Error> {
        //! will work through the raw code and create tokens
        //! from it.
 
        // checks to see if there are already tokens, if there 
        // are then something is wrong? you shouldn't be calling
        // this thing twice on the same object.
        if self.tokens.len() != 0 {
            return Err(ScannerError::general("can't run scan more than once."));
        }

        loop {
            // requests the next token from the stream
            match self.get_next_token()? {
                // if there are no more tokens then we are 
                // at the end of the stream
                None => break,
                // if we get a token we can just add it to the 
                // list of tokens
                Some(token) => {
                    if token.item() == Token::EOL { self.line_number += 1; }
                    self.tokens.push(token);
                }
            }
        }
        
        Ok(self)
    }

    fn get_next_token(&mut self) -> Result<Option<CodeToken>,Error> {
        //! returns the next token in the stream, will error
        //! if finds something it doesn't know how to tokenize.
        //! will return None when it reaches the end of the stream.

        // checks if we are at the end of the stream
        if self.cursor_pos >= self.raw_code.len() {
            return Ok(None);
        }

        // the starting position of the token, so we can
        // place it inside the token when finished
        let code_start : usize = self.cursor_pos;
        let mut code_end : Option<usize> = None;

         // gets a slice of the next character
        let character = &self.raw_code[self.cursor_pos .. self.cursor_pos + 1];
        self.cursor_pos += 1;

        // determines what the token could possibly be
        let token = match character {
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
            "[" => if let Some(level) = self.scan_peek_multiline_block() { self.scan_token_multiline_string(level)? } 
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
                   else { return Err(ScannerError::illegal_character(self,None)) },
            "\"" => self.scan_token_string("\"")?,
            "'" => self.scan_token_string("'")?,

            " " => { code_end = Some(self.scan_peek_all(" ") + 1); Token::WhiteSpace },

            character => if Token::is_eol(character) { Token::EOL } 
                    else { 
                        // the catch all part, this needs to check if its a number, string, or identifier
                        match self.scan_peek_token_keyword(character) {
                            Some(keyword) => keyword,
                            None => match self.scan_peek_token_number(character)? {
                                Some(number) => number,
                                None => return Err(ScannerError::illegal_character(self,None)),
                            }
                        }
                    },
        };

        let token_length = if let Some(length) = code_end { length } else { token.len() };
        let code_token = CodeRef { 
            item : token, 
            code_start, 
            code_end : token_length + code_start,
            line_number : self.line_number 
        };
        Ok(Some(code_token))
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
        if self.raw_code.len() - 1 < self.cursor_pos + (length-1) {
            return false;
        }

        // now will look in only the length of the searching characters, 
        // most of the time this will probably be `1`
        //for i in 0 .. length {
            // get the next character slice
            let char = &self.raw_code[self.cursor_pos .. self.cursor_pos + length];

            // checks if its what we expect so far we will keep doing 
            // this until we hit a point where it doesn't match, 
            // and that means its not the characters we are looking for
            //if char != &chars[i .. i + 1] {
            //    return false;
            //}
            if char != chars { return false; }
        //}

        // we found what we were looking for, so move the cursor 
        // we don't return the characters because we are counting on
        // the fact that where this is being used will replace them, since
        // we are consuming them by moving the counter forward.
        self.cursor_pos += length;
        true
    }

    fn scan_peek_all(&mut self, char : &str) -> usize {
        //! a peek that will consume all of the characters, use this 
        //! in places where it doesn't matter how many of these characters
        //! exist, like WHITESPACE or EOL
        
        let mut count_found : usize = 0;
        while self.scan_peek(char) { count_found += 1; }

        count_found
    }

    fn scan_peek_token_keyword(&mut self, first : &str) -> Option<Token> {
        //! acts like peek, where it moves the cursor if it finds what it wants, but 
        //! also returns the token that it finds
        
        if !Token::is_valid_word_char(first,true) { return None; }

        let mut pos = self.cursor_pos;
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

        self.cursor_pos = pos;

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

        let mut pos = self.cursor_pos;
        let mut number : String = first.to_string();
        // need to do this because rust will have a stack overflow if 
        // you try to parse a string as a float with more than 1 decimal
        let mut decimal_number : usize = if first == "." { 1 } else { 0 };
        // to check if we found it in exponential formatting.
        let mut exponent_format = false;  

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
                false => match char {
                    "e" | "E" => { 
                        // if we find an e in the number, it might be a number still, we just can
                        // only have 1 and can't have any decimal places anymore.
                        if !exponent_format && decimal_number == 0 {
                            pos += 1;
                            number = format!("{}{}",number,char);
                            exponent_format = true;
                        } else {
                            break
                        }
                    },
                    "-" => {
                        if exponent_format {
                            pos += 1;
                            number = format!("{}{}",number,char);
                        } else { break; }
                    }
                    _ => break,
                },
                true => {
                    // adds the character to the working word
                    pos += 1; 
                    number = format!("{}{}",number,char);
                },
            }
        }

        if decimal_number > 1 {
            return Err(ScannerError::number_parsing(self,number.len(),
                &format!("a number can't have more than 1 decimal point, found {}",decimal_number)));
        }

        if decimal_number == 1 && exponent_format {
            return Err(ScannerError::number_parsing(self,number.len(),
                &format!("a number can't have more a decimal and be in exp format")));
        }

        if number == "." { return Ok(None); }

        match number.parse::<f32>() {
            Err(_) => { 
                #[cfg(feature = "dev-testing")]
                {
                    println!("attempted to parse : {}",number);
                }

                Err(ScannerError::number_parsing(self,number.len(),"can't parse as number")) 
            },
            Ok(num) =>  {
                self.cursor_pos = pos;
                Ok(Some(Token::Number(num)))
            },
        }
    }

    fn scan_peek_multiline_block(&mut self) -> Option<usize> {
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
        
        let mut working_pos = self.cursor_pos;
        let mut level = 0;

        // will check if the previous character is the first '[' or if the
        // current character is the first '[', then moves so the cursor is 
        // currently right after the first '['
        if &self.raw_code[working_pos - 1 .. working_pos] != "[" {
            working_pos += 1;
        }

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
        
        self.cursor_pos = working_pos;
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
            if self.cursor_pos == self.raw_code.len() {
                return Err(ScannerError::unterminated_code_segment(self,1,1,"string not terminated"));  
            }

            // the next character
            let char = &self.raw_code[self.cursor_pos .. self.cursor_pos + 1];
            self.cursor_pos += 1;

            match char == starter {
                false => string = format!("{}{}",string,char),
                true => return Ok(Token::String(string)),
            } 
        }
    }

    fn scan_token_multiline_string(&mut self, level : usize) -> Result<Token,Error> {
        //! returns a multiline string Token
        
        match self.scan_token_multiline(level) {
            Err(error) => Err(error),
            Ok(string) => Ok(Token::MultiLineString(string)),
        }
    }

    fn scan_token_multiline(&mut self, level : usize) -> Result<String,Error> {
        //! will act as the rest of what we are getting is a comment
        //! this doesn't do any checking because it assumes you did a peek check
        //! that there is actually a comment
        //! 
        //! Handles the `[==[` style where the number of `=` is the level
        
        // creates the string of ending characters so we know what to look for.
        let ending_chars : String = {
            let mut string = String::from("]");

            for _ in 0 .. level {
                string = format!("{}{}","=",string);
            }

            string // should look like `==]` where the number of `=` is the level
        };

        let mut string : String = String::new();

        loop {

            // checks if we reached the end of the code without the comment close
            if self.cursor_pos == self.raw_code.len() {
                return Err(ScannerError::unterminated_code_segment(self,level+2,level+2,"multiline comment has no end, starts here"));  
            }

            let char = &self.raw_code[self.cursor_pos .. self.cursor_pos + 1];
            self.cursor_pos += 1;

            match char {
                "]" => if self.scan_peek(&ending_chars) { break; } 
                       else { string = format!("{}{}",string,char); },
                _ => string = format!("{}{}",string,char)
            }
        }

        Ok(string)
    }

    fn scan_token_comment(&mut self) -> Result<Token,Error> {
        //! will act as the rest of what we are getting is a comment
        //! this doesn't do any checking because it assumes you did a peek check
        //! that there is actually a comment
        //! 
        //! Handles both `--` and `--[[ ]]` comments
         
        let mut string = String::new();
        
        // first we need to know what kind of comment we are working with
        if let Some(level) = self.scan_peek_multiline_block() {
            // looks like its a long comments, we are going to cheat here and use
            // the string stuff too, because it uses the same logic except it doesn't
            // have the -- in front
            
            match self.scan_token_multiline(level) {
                Err(error) => return Err(error),
                Ok(in_string) => string = in_string
            }

        } else {
            // we have the simple comment, which terminates at the end of the line
            
            loop {
                // check if we are at the end of the code
                if self.cursor_pos == self.raw_code.len() { break; }

                let char = &self.raw_code[self.cursor_pos .. self.cursor_pos + 1];
                match Token::is_eol(char) {
                    true => break, // we don't want to consume an EOL token in a simple comment
                    false => {
                        self.cursor_pos += 1;
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

    use crate::scanner::Scanner;

    #[test]
    pub fn scan_lua_test_suite() {
        use std::fs::File;
        use std::io::Read;
        use std::str;

        let file_names = vec![
            // "all.lua", // fails because of #! is invalid rust, TODO : figure out what to do, if anything
            "api.lua",
            "attrib.lua",
            "big.lua",
            "calls.lua",
            "checktable.lua",
            "closure.lua",
            "code.lua",
            "constructs.lua",
            // "db.lua", // fails because not UTF-8, has unicode? TODO : unicode support
            "errors.lua",
            "events.lua",
            // "files.lua", // fails because not UTF-8, has unicode? TODO : unicode support
            "gc.lua",
            // "literals.lua", // fails because not UTF-8, has unicode? TODO : unicode support
            "locals.lua",
            "main.lua",
            "math.lua",
            "nextvar.lua",
            // "pm.lua", // fails because not UTF-8, has unicode? TODO : unicode support
            // "sort.lua", // fails because not UTF-8, has unicode? TODO : unicode support
            // "strings.lua", // fails because not UTF-8, has unicode? TODO : unicode support
            "vararg.lua",
            "verybig.lua",
        ];

        // checks each of the test files, makes sure
        // that we can read it without error
        for file_name in file_names {
            let code_stream : Vec<u8> = { 
                // loads the contents of the file
                let mut contents : Vec<u8> = Vec::new();
                let mut file = File::open(&format!("../lua/lua-test-suite/{}",file_name)).expect(&format!("{}: can't open file",file_name));
                file.read_to_end(&mut contents).expect(&format!("{}: can't read file",file_name));

                contents

            };

            let code = match str::from_utf8(&code_stream) {
                Ok(c) => c,
                Err(error) =>  { println!("{}: {}",file_name,error); assert!(false); "" },
            };

            match Scanner::from_str(&code,Some(file_name)) {
                Err(error) => { println!("{}: {}",file_name,error); assert!(false); }
                Ok(_) => assert!(true),
            }
        }
    }

}