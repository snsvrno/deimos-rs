use grammar::gram::Gram;
use token::Token;
use tokentype::TokenType;

// can be a number, string, bool, nil, 
#[derive(PartialEq,Clone,Debug)]
pub struct Literal {
    token : Token,
}

impl Literal {

    pub fn create_from(token : Token) -> Option<Gram> {
        match token.get_type() {
            TokenType::True |
            TokenType::False |
            TokenType::Nil |
            TokenType::Number(_) |
            TokenType::String(_) => Some(Gram::Literal(Box::new(Literal{ token }))),
            _ => None,
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}",self.token)
    }
}

mod tests {

    #[test]
    fn basic_parsing() {
        use grammar::literal::Literal;
        use token::Token;
        use tokentype::TokenType;

        // creates some tokens
        let token_1 = Token::simple(TokenType::Equal);
        let token_2 = Token::simple(TokenType::True);
        let token_3 = Token::simple(TokenType::False);
        let token_4 = Token::simple(TokenType::Nil);
        let token_5 = Token::simple(TokenType::Number(123.32));
        let token_6 = Token::simple(TokenType::LeftParen);

        assert!(Literal::create_from(token_1).is_none());
        assert!(Literal::create_from(token_2).is_some());
        assert!(Literal::create_from(token_3).is_some());
        assert!(Literal::create_from(token_4).is_some());
        assert!(Literal::create_from(token_5).is_some());
        assert!(Literal::create_from(token_6).is_none());
    }
    
}