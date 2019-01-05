use tokentype::TokenType;
use token::Token;
use grammar::gram::Gram;
use grammar::expression::Expression;

#[derive(PartialEq,Clone,Debug)]
pub struct Binary {
    left_expr : Expression,
    operator : Token,
    right_expr : Expression,
}

impl Binary {
    
    pub fn create_from(left_token : &Gram, operator: &Gram, right_token : &Gram) -> Option<Gram> {
        match (left_token, operator, right_token) {
            (Gram::Expression(left_expr), Gram::Token(token), Gram::Expression(right_expr)) => {
                match token.get_type() {
                    TokenType::Carrot |
                    TokenType::Star | 
                    TokenType::Slash | 
                    TokenType::Plus |
                    TokenType::Minus | 
                    TokenType::DoublePeriod |
                    TokenType::LessThan |
                    TokenType::GreaterThan |
                    TokenType::GreaterEqual |
                    TokenType::LessEqual |
                    TokenType::NotEqual |
                    TokenType::EqualEqual |
                    TokenType::And |
                    TokenType::Or => Some(Gram::Binary(Box::new(Binary{
                        left_expr : *left_expr.clone(),
                        operator : token.clone(),
                        right_expr : *right_expr.clone(),
                    }))),
                    _ => None,
                }
            }
            (_, _, _) => None,
        }
    }

}

mod tests {

    #[test]
    fn basic_parsing() {
        use tokentype::TokenType;
        use token::Token;
        use grammar::binary::Binary;
        use grammar::gram::Gram;

        let exp1 = Gram::Token(Token::simple(TokenType::Nil)).to_literal().to_expr();
        let exp2 = Gram::Token(Token::simple(TokenType::String("what".to_string()))).to_literal().to_expr();

        let carrot = Gram::Token(Token::simple(TokenType::Carrot)); 
        let star = Gram::Token(Token::simple(TokenType::Star)); 
        let slash = Gram::Token(Token::simple(TokenType::Slash)); 
        let plus = Gram::Token(Token::simple(TokenType::Plus));
        let minus = Gram::Token(Token::simple(TokenType::Minus)); 
        let double_period = Gram::Token(Token::simple(TokenType::DoublePeriod));
        let less_than = Gram::Token(Token::simple(TokenType::LessThan));
        let greater_than = Gram::Token(Token::simple(TokenType::GreaterThan));
        let greater_equal = Gram::Token(Token::simple(TokenType::GreaterEqual));
        let less_equal = Gram::Token(Token::simple(TokenType::LessEqual));
        let not_equal = Gram::Token(Token::simple(TokenType::NotEqual));
        let equal_equal = Gram::Token(Token::simple(TokenType::EqualEqual));
        let and = Gram::Token(Token::simple(TokenType::And));
        let or = Gram::Token(Token::simple(TokenType::Or));

        assert!(Binary::create_from(&exp1, &carrot, &exp2).is_some());
        assert!(Binary::create_from(&exp1, &star, &exp2).is_some());
        assert!(Binary::create_from(&exp1, &slash, &exp2).is_some());
        assert!(Binary::create_from(&exp1, &or, &exp2).is_some());
        assert!(Binary::create_from(&exp1, &double_period, &exp2).is_some());
        assert!(Binary::create_from(&exp1, &plus, &exp2).is_some());
        assert!(Binary::create_from(&exp1, &minus, &exp2).is_some());
        assert!(Binary::create_from(&exp1, &less_than, &exp2).is_some());
        assert!(Binary::create_from(&exp1, &and, &exp2).is_some());
        assert!(Binary::create_from(&exp1, &equal_equal, &exp2).is_some());
        assert!(Binary::create_from(&exp1, &greater_equal, &exp2).is_some());
        assert!(Binary::create_from(&exp1, &greater_than, &exp2).is_some());
        assert!(Binary::create_from(&exp1, &less_equal, &exp2).is_some());
        assert!(Binary::create_from(&exp1, &not_equal, &exp2).is_some());

        let left_paren = Gram::Token(Token::simple(TokenType::LeftParen));
        let not = Gram::Token(Token::simple(TokenType::Not));
        assert!(Binary::create_from(&exp1, &left_paren, &exp2).is_none());
        assert!(Binary::create_from(&exp1, &not, &exp2).is_none());
    }
}