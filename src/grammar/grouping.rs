use crate::tokentype::TokenType;
use crate::token::Token;
use crate::grammar::gram::Gram;
use crate::grammar::expression::Expression;
use failure::Error;

#[derive(PartialEq,Clone,Debug)]
pub struct Grouping {
    expr : Gram,
}

impl Grouping {
    
    pub fn create(left_token : &Gram, expression: &Gram, right_token : &Gram) -> Option<Grouping> {
        match (left_token, &expression, right_token) {
            (Gram::Token(left_token), Gram::Expression(_), Gram::Token(right_token)) => {
                if left_token == &TokenType::LeftParen && right_token == &TokenType::RightParen {
                    Some(Grouping { expr : expression.clone() })
                } else {
                    None
                }
            }
            (_, _, _) => None,
        }
    }

    pub fn create_into_gram(left_token : &Gram, expression: &Gram, right_token : &Gram) -> Option<Gram> {
        match Grouping::create(left_token,expression,right_token) {
            None => None,
            Some(group) => Some(Gram::Grouping(Box::new(group)))
        }
    }

 

}

impl std::fmt::Display for Grouping {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"({})",self.expr)
    }
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! create_grouping {
    ($gram:expr) => {
        $crate::grammar::grouping::Grouping::create_into_gram($gram).unwrap()
    };
}

mod tests {

    #[test]
    fn basic_parsing() {
    }
}