use crate::elements::CodeSlice;
use crate::elements::TokenType;

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

    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn get_code_slice(&self) -> &CodeSlice {
        &self.code_slice
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
            (TokenType::Comment(ca), TokenType::Comment(cb)) => ca == cb,
            (_, _) => self.get_type() == other.get_type() 
        }
    }
}

impl PartialEq<TokenType> for Token {
    fn eq(&self, other: &TokenType) -> bool {
        match (&self.token_type,other) {
            (TokenType::String(_), TokenType::String(_)) => true,
            (TokenType::Number(_), TokenType::Number(_)) => true,
            (TokenType::Comment(_), TokenType::Comment(_)) => true,
            (_, _) => self.get_type() == other,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{:?}",self.token_type)
    }
}
