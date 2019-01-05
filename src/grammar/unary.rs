use tokentype::TokenType;
use token::Token;
use grammar::gram::Gram;
use grammar::expression::Expression;

#[derive(PartialEq,Clone,Debug)]
pub struct Unary {
    modifier : Token,
    expr : Expression,
}

impl Unary {
    
    pub fn create_from(left_token : &Gram, right_token : &Gram) -> Option<Gram> {

        match (left_token, right_token) {
            (Gram::Token(token), Gram::Expression(expr)) => {
                if token == &TokenType::Minus {
                    return Some(Gram::Unary(Box::new(Unary{
                        modifier : token.clone(),
                        expr : *expr.clone(),
                    })));
                }
            },
            _ => (),
        }
        None
    }

}

impl std::fmt::Display for Unary {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"({} {})",self.modifier,self.expr)
    }
}

mod tests {

    #[test]
    fn basic_parsing() {
        use tokentype::TokenType;
        use token::Token;
        use grammar::unary::Unary;
        use grammar::gram::Gram;

        // depth = -0.1234
        let token_stream = vec![
            Gram::Token(Token::simple(TokenType::Identifier("depth".to_string()))),
            Gram::Token(Token::simple(TokenType::Equal)),
            Gram::Token(Token::simple(TokenType::Minus)),
            Gram::Token(Token::simple(TokenType::Number(0.1234))).to_literal().unwrap().to_expr().unwrap(),
        ];

        assert!(Unary::create_from(&token_stream[0], &token_stream[1]).is_none());
    }
}