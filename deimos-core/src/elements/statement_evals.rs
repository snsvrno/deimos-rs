
pub mod unop {
    use failure::{format_err, Error};
    use crate::elements:: { Statement, Token, TokenType, CodeSlice };

    pub fn minus(s1 : &Statement) -> Result<Statement,Error> {
        if let Some(a) = s1.cast_to_number() {
            return Ok(Statement::Token(Token::new(
                TokenType::Number(a * -1.0),
                s1.get_code_slice().clone()
            )));
        }

        Err(format_err!("Don't know how to do '- {}'",s1))
    }

    pub fn not(s1 : &Statement) -> Result<Statement,Error> {
        if let Some(a) = s1.cast_to_bool() {
            let token = if a { TokenType::False } else { TokenType::True };
            return Ok(Statement::Token(Token::new(
                token,
                s1.get_code_slice().clone()
            )));
        }

        Err(format_err!("Don't know how to do 'not {}'",s1))
    }
}

pub mod binop {
    use failure::{Error,format_err};
    use crate::elements::{ CodeSlice, Statement, Token, TokenType };

    pub fn plus(s1 : &Statement, s2 : &Statement) -> Result<Statement,Error> {
        if let Some(a) = s1.cast_to_number() {
            if let Some(b) = s2.cast_to_number() {
                let result = a + b;
                return Ok(Statement::Token(Token::new(
                    TokenType::Number(result),
                    CodeSlice::create_from(&s1.get_code_slice(),&s2.get_code_slice())
                )));
            }
        }

        Err(format_err!("Don't know how to add {} and {}",s1,s2))
    }

    pub fn minus(s1 : &Statement, s2 : &Statement) -> Result<Statement,Error> {
        if let Some(a) = s1.cast_to_number() {
            if let Some(b) = s2.cast_to_number() {
                let result = a - b;
                return Ok(Statement::Token(Token::new(
                    TokenType::Number(result),
                    CodeSlice::create_from(&s1.get_code_slice(),&s2.get_code_slice())
                )));
            }
        }
        
        Err(format_err!("Don't know how to subtract {} and {}",s1,s2))
    }

    pub fn star(s1 : &Statement, s2 : &Statement) -> Result<Statement,Error> {
        if let Some(a) = s1.cast_to_number() {
            if let Some(b) = s2.cast_to_number() {
                let result = a * b;
                return Ok(Statement::Token(Token::new(
                    TokenType::Number(result),
                    CodeSlice::create_from(&s1.get_code_slice(),&s2.get_code_slice())
                )));
            }
        }
        
        Err(format_err!("Don't know how to multiple {} and {}",s1,s2))
    }
    pub fn slash(s1 : &Statement, s2 : &Statement) -> Result<Statement,Error> {
        if let Some(a) = s1.cast_to_number() {
            if let Some(b) = s2.cast_to_number() {
                let result = a / b;
                return Ok(Statement::Token(Token::new(
                    TokenType::Number(result),
                    CodeSlice::create_from(&s1.get_code_slice(),&s2.get_code_slice())
                )));
            }
        }
        
        Err(format_err!("Don't know how to divide {} and {}",s1,s2))
    }
    pub fn carrot(s1 : &Statement, s2 : &Statement) -> Result<Statement,Error> {
        if let Some(a) = s1.cast_to_number() {
            if let Some(b) = s2.cast_to_number() {
                let result = f32::powf(a,b);
                return Ok(Statement::Token(Token::new(
                    TokenType::Number(result),
                    CodeSlice::create_from(&s1.get_code_slice(),&s2.get_code_slice())
                )));
            }
        }
        
        Err(format_err!("Don't know how to pow({},{})",s1,s2))
    }
    pub fn percent(s1 : &Statement, s2 : &Statement) -> Result<Statement,Error> {
        if let Some(a) = s1.cast_to_number() {
            if let Some(b) = s2.cast_to_number() {
                let result = a % b;
                return Ok(Statement::Token(Token::new(
                    TokenType::Number(result),
                    CodeSlice::create_from(&s1.get_code_slice(),&s2.get_code_slice())
                )));
            }
        }
        
        Err(format_err!("Don't know how to pow({},{})",s1,s2))
    }
    pub fn double_period(s1 : &Statement, s2 : &Statement) -> Result<Statement,Error> {

        if s1.is_a_token() && s2.is_a_token() {
            Ok(Statement::Token(Token::new(
                TokenType::String(format!("{}{}",s1.as_token_type(),s2.as_token_type())),
                CodeSlice::create_from(&s1.get_code_slice(),&s2.get_code_slice())
            )))
        } else {
            Err(format_err!("Don't know how to concatenate {} and {}",s1,s2))
        }
        
    }
}
