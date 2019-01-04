use tokentype::TokenType;
use codeslice::CodeSlice;

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

    pub fn simple(token : TokenType) -> Token {
        Token {
            token_type : token,

            code_slice : CodeSlice::empty()
        }
    }

    pub fn get_type<'a>(&'a self) -> &'a TokenType {
        &self.token_type
    }
}

impl PartialEq<TokenType> for Token {
    fn eq(&self, other: &TokenType) -> bool {
        match (&self.token_type,other) {
            (TokenType::String(_), TokenType::String(_)) => true,
            // (TokenType::Number(_), TokenType::Number(_)) => true,
            (_, _) => self.token_type == *other 
        }
    }
}
