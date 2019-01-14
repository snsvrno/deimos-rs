use crate::elements::CodeSlice;
use crate::elements::TokenType;

#[derive(Debug)]
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

    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }

    #[cfg(test)]
    pub fn simple(token : TokenType)-> Token {
        Token {
            token_type : token,
            code_slice : CodeSlice::default(),
        }
    }

    pub fn slice_code<'a>(&self, raw_code : &'a str) -> &'a str {
        &self.code_slice.slice_code(raw_code)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        match (&self.token_type,&other.token_type) {
            (TokenType::String(sa), TokenType::String(sb)) => sa == sb,
            (TokenType::Number(na), TokenType::Number(nb)) => na == nb,
            (_, _) => self.get_type() == other.get_type() 
        }
    }
}