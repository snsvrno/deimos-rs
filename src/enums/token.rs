use enums::operator::Operator;
use enums::tokentype::TokenType;

use structs::tree::Tree;

use failure::Error;

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum Token {
    WhiteSpace(usize),
    Int(i32),
    String(String),
    Word(String),
    Function(String),
    Operator(Operator),
    Tree(Tree),
    EOF,
    EOL,
    None,
}

impl Token {
    pub fn combine(mut token1 : Token, mut token2 : Token) -> Result<Token,Error> {  
        match (token1.token_type(),token2.token_type()) {
            (TokenType::Int, TokenType::Int) => Ok(Token::Int(token1.as_int()? * 10 + token2.as_int()?)),
            (TokenType::String, TokenType::String) => Ok(Token::String(format!("{}{}",token1.as_string()?,token2.as_string()?))),
            (TokenType::WhiteSpace, TokenType::WhiteSpace) => Ok(Token::WhiteSpace(token1.as_whitespace()? + token2.as_whitespace()?)),
            (TokenType::Word, TokenType::Word) => Ok(Token::Word(format!("{}{}",token1.as_string()?,token2.as_string()?))),
            (_,_) => Err(format_err!("Cannot combine of type {} and {}",token1.type_is(),token2.type_is())),
        }
    }

    pub fn can_combine(token1 : &Token, token2 : &Token) -> bool {                
        match (token1.token_type(),token2.token_type()) {
            (TokenType::Int, TokenType::Int) => true,
            (TokenType::String, TokenType::String) => true,
            (TokenType::WhiteSpace, TokenType::WhiteSpace) => true,
            (TokenType::Word, TokenType::Word) => true,
            (_,_) => false,
        }
    }

    pub fn combine_into(&mut self, mut token : Token) -> Result<(),Error> {
        let new_token = Token::combine(self.clone(),token)?;
        *self = new_token;
        Ok(())
    }

    pub fn to_function(mut self) -> Token {
        if let Token::Word(string) = self {
            Token::Function(string)
        } else {
            self
        }
    } 

    fn as_int(&mut self) -> Result<i32,Error> {
        match self {
            Token::Int(value) => Ok(*value),
            _ => Err(format_err!("Cannot consume as int if type is {}",self.type_is())),
        }
    }

    fn as_string(&mut self) -> Result<String,Error> {
        match self {
            Token::String(value) | Token::Word(value) => Ok(value.clone()),
            _ => Err(format_err!("Cannot consume as String if type is {}",self.type_is())),
        }
    }

    fn as_whitespace(&mut self) -> Result<usize,Error> {
        match self {
            Token::WhiteSpace(value) => Ok(*value),
            _ => Err(format_err!("Cannot consume as white space if type is {}",self.type_is())),
        }
    }

    pub fn type_is(&self) -> &'static str {
        match self {
            Token::WhiteSpace(_) => "White Space",
            Token::Int(_) => "int",
            Token::Operator(ref op) => op.type_is(),
            Token::EOF => "EOF",
            Token::EOL => "EOL",
            Token::None => "None",
            Token::String(_) => "String",
            Token::Word(_) => "Word",
            Token::Function(_) => "Function",
            Token::Tree(_) => "Tree",
        }
    }

    pub fn token_type(&self) -> TokenType {
        match self {
            Token::Int(_) => TokenType::Int,
            Token::Operator(ref op) => TokenType::Operator,
            Token::String(_) => TokenType::String,
            Token::Word(_) => TokenType::Word,
            Token::WhiteSpace(_) => TokenType::WhiteSpace,
            _ => TokenType::Other
        }
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
}