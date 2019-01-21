use failure::{Error,format_err};

use crate::elements::{ CodeSlice, Statement, Token, TokenType };

pub fn plus(s1 : &Statement, s2 : &Statement) -> Result<Statement,Error> {
    if s1.is_number() && s2.is_number() {
        let result = s1.as_number() + s2.as_number();
        Ok(Statement::Token(Token::new(
            TokenType::Number(result),
            CodeSlice::create_from(&s1.get_code_slice(),&s2.get_code_slice())
        )))
    } else {
        Err(format_err!("Don't know how to add {} and {}",s1,s2))
    }
}