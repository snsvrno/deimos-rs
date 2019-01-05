use tokentype::TokenType;
use token::Token;
use grammar::gram::Gram;

#[derive(PartialEq,Clone,Debug)]
pub struct Expression {
    token : Gram,
}

impl Expression {
    
    pub fn create_from(token : Gram) -> Option<Expression> {
        match token {
            Gram::Literal(_) |
            Gram::Unary(_) |
            Gram::Binary(_) |
            Gram::Grouping(_) => Some(Expression { token }),
            _ => None,
        }
    }
}

mod tests {

    #[test]
    fn basic_parsing() {
    }
}