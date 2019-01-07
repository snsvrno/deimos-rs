use crate::tokentype::TokenType;
use crate::token::Token;
use crate::grammar::gram::Gram;

#[derive(PartialEq,Clone,Debug)]
pub struct Expression {
    token : Gram,
}

impl Expression {
    
    pub fn create(token : &Gram) -> Option<Expression> {
        match token {
            Gram::Literal(_) |
            Gram::Unary(_) |
            Gram::Binary(_) |
            Gram::Grouping(_) => Some(Expression { token : token.clone() }),
            _ => None,
        }
    }

    pub fn create_into_gram(token : &Gram) -> Option<Gram> {
        match Expression::create(token) {
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

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! create_expression {
    ($gram:expr) => {
        $crate::grammar::expression::Expression::create_into_gram($gram).unwrap()
    };
}

mod tests {

    #[test]
    fn basic_parsing() {
    }
}