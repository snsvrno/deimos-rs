use token::Token;
use tokentype::TokenType;
use codeslice::CodeSlice;

use failure::Error;

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
        let mut working_pos = self.current_pos;
        let mut word : String = starter.to_string();

        loop {
            if working_pos == self.raw_code.len() { return None; }

            let n_char = &self.raw_code[working_pos .. working_pos + 1];
            working_pos += 1; 

            word = format!("{}{}",word,n_char);
            
            if working_pos == self.raw_code.len() || self.peek(" ") {
                let keyword : Option<TokenType> = match word.as_str() {
                    "and" => Some(TokenType::And),
                    "break" => Some(TokenType::Break),
                    "do" => Some(TokenType::Do),
                    "else" => Some(TokenType::Else),
                    "elseif" => Some(TokenType::Elseif),
                    "end" => Some(TokenType::End),
                    "false" => Some(TokenType::False),
                    "for" => Some(TokenType::For),
                    "function" => Some(TokenType::Function),
                    "if" => Some(TokenType::If),
                    "in" => Some(TokenType::In),
                    "local" => Some(TokenType::Local),
                    "nil" => Some(TokenType::Nil),
                    "not" => Some(TokenType::Not),
                    "or" => Some(TokenType::Or),
                    "repeat" => Some(TokenType::Repeat),
                    "return" => Some(TokenType::Return),
                    "then" => Some(TokenType::Then),
                    "true" => Some(TokenType::True),
                    "until" => Some(TokenType::Until),
                    "while" => Some(TokenType::While),
                    _ => None,
                };

                match keyword {
                    None => return None,
                    Some(keyword) => {
                        self.current_pos = working_pos;
                        return Some(keyword);
                    }
                }

            }
        }
    }

    fn peek_string(&mut self, starter : &str) -> Result<TokenType,Error> {
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

            
            "\n" => { /* self.peak("\r"); */TokenType::EOL },
            "\r" => { /* self.peak("\n"); */TokenType::EOL },
            " " => TokenType::WhiteSpace,
            
            _ => match self.peek_keyword(char) {
                Some(keyword) => keyword,
                None => return Err(format_err!("Illegal character '{}' found at {}:{}",char,self.current_line,self.current_pos)),
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

    pub fn assert(&self,token : TokenType) -> bool {
        //! used for testing, to check that only 1 single token was matched, 
        //! and it is the correct token
        
        if self.tokens[0] == token && self.tokens.len() == 1{
            true
        } else {
            println!("token[0] = {:?}, tokens.len() = {}, expecting {:?}",self.tokens[0].get_type(), self.tokens.len(), token);
            false
        }
    }

}

mod test {

    #[test]
    pub fn token_test() {
        use scanner::Scanner;
        use tokentype::TokenType;

        // +     -     *     /     %     ^     #
        assert!(Scanner::new("+").scan().expect("error").assert(TokenType::Plus));
        assert!(Scanner::new("-").scan().expect("error").assert(TokenType::Minus));
        assert!(Scanner::new("*").scan().expect("error").assert(TokenType::Star));
        assert!(Scanner::new("/").scan().expect("error").assert(TokenType::Slash));
        assert!(Scanner::new("%").scan().expect("error").assert(TokenType::Percent));
        assert!(Scanner::new("^").scan().expect("error").assert(TokenType::Carrot));
        assert!(Scanner::new("#").scan().expect("error").assert(TokenType::Pound));
        // ==    ~=    <=    >=    <     >     =
        assert!(Scanner::new("==").scan().expect("error").assert(TokenType::EqualEqual));
        assert!(Scanner::new("~=").scan().expect("error").assert(TokenType::NotEqual));
        assert!(Scanner::new("<=").scan().expect("error").assert(TokenType::LessEqual));
        assert!(Scanner::new(">=").scan().expect("error").assert(TokenType::GreaterEqual));
        assert!(Scanner::new("<").scan().expect("error").assert(TokenType::LessThan));
        assert!(Scanner::new(">").scan().expect("error").assert(TokenType::GreaterThan));
        assert!(Scanner::new("=").scan().expect("error").assert(TokenType::Equal));
        // (     )     {     }     [     ]
        assert!(Scanner::new("(").scan().expect("error").assert(TokenType::LeftParen));
        assert!(Scanner::new(")").scan().expect("error").assert(TokenType::RightParen));
        assert!(Scanner::new("{").scan().expect("error").assert(TokenType::LeftMoustache));
        assert!(Scanner::new("}").scan().expect("error").assert(TokenType::RightMoustache));
        assert!(Scanner::new("[").scan().expect("error").assert(TokenType::LeftBracket));
        assert!(Scanner::new("]").scan().expect("error").assert(TokenType::RightBracket));
        // ;     :     ,     .     ..    ...
        assert!(Scanner::new(";").scan().expect("error").assert(TokenType::SemiColon));
        assert!(Scanner::new(":").scan().expect("error").assert(TokenType::Colon));
        assert!(Scanner::new(",").scan().expect("error").assert(TokenType::Comma));
        assert!(Scanner::new(".").scan().expect("error").assert(TokenType::Period));
        assert!(Scanner::new("..").scan().expect("error").assert(TokenType::DoublePeriod));
        assert!(Scanner::new("...").scan().expect("error").assert(TokenType::TriplePeriod));

        // a string
        assert!(Scanner::new("\"ashortstring\"").scan().expect("error").assert(TokenType::String("".to_string())));
        assert!(Scanner::new("\"a longer string\"").scan().expect("error").assert(TokenType::String("".to_string())));
        
        // and       break     do        else      elseif
        assert!(Scanner::new("and").scan().expect("error").assert(TokenType::And));
        assert!(Scanner::new("break").scan().expect("error").assert(TokenType::Break));
        assert!(Scanner::new("do").scan().expect("error").assert(TokenType::Do));
        assert!(Scanner::new("else").scan().expect("error").assert(TokenType::Else));
        assert!(Scanner::new("elseif").scan().expect("error").assert(TokenType::Elseif));
        // end       false     for       function  if
        assert!(Scanner::new("end").scan().expect("error").assert(TokenType::End));
        assert!(Scanner::new("false").scan().expect("error").assert(TokenType::False));
        assert!(Scanner::new("for").scan().expect("error").assert(TokenType::For));
        assert!(Scanner::new("function").scan().expect("error").assert(TokenType::Function));
        assert!(Scanner::new("if").scan().expect("error").assert(TokenType::If));
        // in        local     nil       not       or
        assert!(Scanner::new("in").scan().expect("error").assert(TokenType::In));
        assert!(Scanner::new("local").scan().expect("error").assert(TokenType::Local));
        assert!(Scanner::new("nil").scan().expect("error").assert(TokenType::Nil));
        assert!(Scanner::new("not").scan().expect("error").assert(TokenType::Not));
        assert!(Scanner::new("or").scan().expect("error").assert(TokenType::Or));
        // repeat    return    then      true      until     while
        assert!(Scanner::new("repeat").scan().expect("error").assert(TokenType::Repeat));
        assert!(Scanner::new("return").scan().expect("error").assert(TokenType::Return));
        assert!(Scanner::new("then").scan().expect("error").assert(TokenType::Then));
        assert!(Scanner::new("true").scan().expect("error").assert(TokenType::True));
        assert!(Scanner::new("until").scan().expect("error").assert(TokenType::Until));
        assert!(Scanner::new("while").scan().expect("error").assert(TokenType::While));
    }

}