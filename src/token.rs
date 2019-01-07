use crate::tokentype::TokenType;
use crate::codeslice::CodeSlice;

#[derive(Debug,Clone)]
pub struct Token {
    token_type : TokenType,
    
    // for debugging and error messaging
    code_slice : CodeSlice,
}

impl Token {
    pub fn new(token_type : TokenType, code_slice : CodeSlice) -> Token {
        Token {
            token_type,
            code_slice
        }
    }

    pub fn simple(token : TokenType) -> Token {
        Token {
            token_type : token,

            code_slice : CodeSlice::empty()
        }
    }

    pub fn get_type<'a>(&'a self) -> &'a TokenType {
        &self.token_type
    }

    pub fn valid_word_char(char : &str, first : bool) -> bool {
        let allowable_ranges = vec![
            // (u start, u end, can start)
            (65,90,true), // A-Z
            (97,122,true), // a-z
            (48,57,false), // 0-9
            (95,95,false) // _
        ];

        if char.len() == 1 {
            if let Some(c) = char.chars().next(){
                let code = c as u32;
                for range in allowable_ranges {
                    if range.0 <= code && code <= range.1 {
                        if first && range.2 == false {
                            return false;
                        } else {
                            return true
                        }
                    }
                }
            }
        }
        
        false
    }

    pub fn valid_number_char(char : &str) -> bool {
        let allowable_ranges = vec![
            // (u start, u end, can start)
            (48,57), // 0-9
            (46,46), // .
        ];

        if char.len() == 1 {
            if let Some(c) = char.chars().next(){
                let code = c as u32;
                for range in allowable_ranges {
                    if range.0 <= code && code <= range.1 {
                        return true;
                    }
                }
            }
        }
        
        false
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}",self.token_type)
    }
}

impl PartialEq<TokenType> for Token {
    fn eq(&self, other: &TokenType) -> bool {
        match (&self.token_type,other) {
            (TokenType::String(_), TokenType::String(_)) => true,
            // (TokenType::Number(_), TokenType::Number(_)) => true,
            (_, _) => self.get_type() == other 
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        match (&self.token_type,&other.token_type) {
            (TokenType::String(_), TokenType::String(_)) => true,
            // (TokenType::Number(_), TokenType::Number(_)) => true,
            (_, _) => self.get_type() == other.get_type() 
        }
    }
}
