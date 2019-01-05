use tokentype::TokenType;
use token::Token;
use grammar::gram::Gram;
use grammar::expression::Expression;

#[derive(PartialEq,Clone,Debug)]
pub struct Grouping {
    expr : Gram,
}

impl Grouping {
    
    pub fn create_from(left_token : Gram, expression: Gram, right_token : Gram) -> Option<Grouping> {
        match (left_token, &expression, right_token) {
            (Gram::Token(left_token), Gram::Expression(_), Gram::Token(right_token)) => {
                if left_token == TokenType::LeftParen && right_token == TokenType::RightParen {
                    Some(Grouping { expr : expression })
                } else {
                    None
                }
            }
            (_, _, _) => None,
        }
    }

}

mod tests {

    #[test]
    fn basic_parsing() {
    }
}