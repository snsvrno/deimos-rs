use crate::grammar::gram::Gram;
use crate::token::Token;
use crate::tokentype::TokenType;

// can be a number, string, bool, nil, 
#[derive(PartialEq,Clone)]
pub struct Literal {
    token : Token,
}

impl Literal {

    pub fn create(token : Token) -> Option<Literal> {
        match token.get_type() {
            TokenType::True |
            TokenType::False |
            TokenType::Nil |
            TokenType::Number(_) |
            TokenType::Identifier(_) | 
            TokenType::String(_) => Some(Literal{ token }),
            _ => None,
        }
    }

    pub fn create_into_gram(token : Token) -> Option<Gram> {
        match Literal::create(token) {
            None => None,
            Some(lit) => Some(Gram::Literal(Box::new(lit)))
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}",self.token)
    }
}

impl std::fmt::Debug for Literal {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"L<{:?}>",self.token)
    }
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! literal {
    ($tokentype:expr) => {
        $crate::grammar::literal::Literal::create_into_gram(
            $crate::token::Token::simple($tokentype)
        ).unwrap()
    };
}

mod tests {

    #[test]
    fn basic_parsing() {
        use crate::grammar::literal::Literal;
        use crate::token::Token;
        use crate::tokentype::TokenType;

        // creates some tokens
        let token_1 = Token::simple(TokenType::Equal);
        let token_2 = Token::simple(TokenType::True);
        let token_3 = Token::simple(TokenType::False);
        let token_4 = Token::simple(TokenType::Nil);
        let token_5 = Token::simple(TokenType::Number(123.32));
        let token_6 = Token::simple(TokenType::LeftParen);

        assert!(Literal::create(token_1).is_none());
        assert!(Literal::create(token_2).is_some());
        assert!(Literal::create(token_3).is_some());
        assert!(Literal::create(token_4).is_some());
        assert!(Literal::create(token_5).is_some());
        assert!(Literal::create(token_6).is_none());
    }
    
}