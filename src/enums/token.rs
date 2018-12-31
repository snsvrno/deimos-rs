use enums::operator::Operator;
use enums::tokentype::TokenType;

use failure::Error;

#[derive(Debug,Clone)]
pub enum Token {
    Int(i32),
    Operator(Operator),
    EOF,
    EOL,
    None,
}

impl Token {
    pub fn combine(mut token1 : Token, mut token2 : Token) -> Result<Token,Error> {
        if token1.token_type() == token2.token_type() && token1.token_type() == TokenType::Int {
            return Ok(Token::Int(token1.consume_as_int()? * 10 + token2.consume_as_int()?));
        }

        Err(format_err!("Cannot combine of type {} and {}",token1.type_is(),token2.type_is()))
    }

    pub fn can_combine(token1 : &Token, token2 : &Token) -> bool {
        if token1.token_type() == token2.token_type() && token1.token_type() == TokenType::Int {
            return true;
        }

        false
    }

    pub fn combine_into(&mut self, mut token : Token) -> Result<(),Error> {
        if self.token_type() == token.token_type() && self.token_type() == TokenType::Int {
            *self = Token::Int(self.consume_as_int()? * 10 + token.consume_as_int()?);
        }

        Err(format_err!("Cannot combine of type {} and {}",self.type_is(),token.type_is()))
    }

    pub fn consume_as_int(&mut self) -> Result<i32,Error> {
        if let Token::Int(value) = self {
            Ok(*value)
        } else {
            Err(format_err!("Cannot consume as int if type is {}",self.type_is()))
        }
    }

    pub fn type_is(&self) -> &'static str {
        match self {
            Token::Int(_) => "int",
            Token::Operator(ref op) => op.type_is(),
            Token::EOF => "EOF",
            Token::EOL => "EOL",
            Token::None => "None",
        }
    }

    pub fn token_type(&self) -> TokenType {
        match self {
            Token::Int(_) => TokenType::Int,
            Token::Operator(ref op) => TokenType::Operator,
            _ => TokenType::Other
        }
    }
}