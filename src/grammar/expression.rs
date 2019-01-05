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

    pub fn create_into_gram(token : Gram) -> Option<Gram> {
        match Expression::create_from(token) {
            None => None,
            Some(expr) => Some(Gram::Expression(Box::new(expr))),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}",self.token)
    }
}

mod tests {

    #[test]
    fn basic_parsing() {
    }
}