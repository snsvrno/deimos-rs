use crate::tokentype::TokenType;
use crate::token::Token;
use crate::chunk::Chunk;

use crate::grammar::gram::Gram;
use crate::grammar::expression::Expression;

use failure::{Error,format_err};

#[derive(PartialEq,Clone)]
pub struct Grouping {
    expr : Expression,
}

impl Grouping {
    
    pub fn create(left_token : &Gram, expression: &Gram, right_token : &Gram) -> Option<Grouping> {
        match (left_token, &expression, right_token) {
            (Gram::Token(left_token), Gram::Expression(expression), Gram::Token(right_token)) => {
                if left_token == &TokenType::LeftParen && right_token == &TokenType::RightParen {
                    Some(Grouping { expr : *expression.clone() })
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

    pub fn process_set(chunk : &mut Chunk) -> Result<bool,Error> {

        // needs at least chunk in order to match a binary, since the binary 
        // is 3 Expr (op) Expr, else it will just return.
        if chunk.len() < 3 { return Ok(false); }

        let mut found_a_match = false;

        loop {
            let mut local_match : Option<usize> = None;

            // get a group of 3 chunk and check it against all of the operators in the group
            for i in 0 .. (chunk.len()-2) {
                
                // first we check if it matches the general patter for a binary,
                // if the 1st and 3rd chunk aren't expressions we move on to the next
                // group of chunk
                if !chunk.at(i).is_token() || !chunk.at(i+2).is_token() { continue; }
                
                if let Gram::Token(ref token_left) = chunk.at(i) {
                    if let Gram::Token(ref token_right) = chunk.at(i+2) {
                        if token_left == &TokenType::LeftParen && token_right == &TokenType::RightParen {
                            found_a_match = true;
                            local_match = Some(i);
                            break;
                        }
                    }      
                }
            }

            if let Some(i) = local_match {
                let mut removed_tokens : Vec<Gram> = chunk.remove(i,i+3);
                let _right : Gram = if let Some(gram) = removed_tokens.pop() { gram } else { 
                    return Err(format_err!("Failed to build Binary, tried to remove 1/3 chunk but failed.")); };
                let middle : Gram = if let Some(gram) = removed_tokens.pop() { gram } else { 
                    return Err(format_err!("Failed to build Binary, tried to remove 2/3 chunk but failed.")); };
                let _left : Gram = if let Some(gram) = removed_tokens.pop() { gram } else { 
                    return Err(format_err!("Failed to build Binary, tried to remove 3/3 chunk but failed.")); };

                let new_gram = Gram::Grouping(Box::new(Grouping {
                    expr : middle.unwrap_expr()?,
                }));

                match Expression::create_into_gram(&new_gram) {
                    None => return Err(format_err!("You shouldn't ever see this error!")), 
                    Some(expr_gram) => { chunk.insert(i,expr_gram); }
                }
            } else {
                break;
            }
        }
    
        Ok(found_a_match)
    }

}

impl std::fmt::Display for Grouping {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"<{}>",self.expr)
    }
}


impl std::fmt::Debug for Grouping {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"G<{:?}>",self.expr)
    }
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! grouping {
    ($gram:expr) => {
        $crate::grammar::grouping::Grouping::create_into_gram(
            &token!(TokenType::LeftParen),
            &$gram,
            &token!(TokenType::RightParen)
            ).unwrap()
    };
}

mod tests {

    #[test]
    fn basic_parsing() {
        use crate::tokentype::TokenType;
        use crate::chunk::Chunk;
        use crate::grammar::grouping::Grouping;
        
        // should do some grouping
        // need to precreate the binaries because that segment isn't here.
        // (5 + 6) * (2 - 3)

        let mut tokens = Chunk::new_from(vec![
            token!(TokenType::LeftParen),

            expression!(
                &binary!(
                    &token!(TokenType::Plus),
                    &expression!(&literal!(TokenType::Number(5.0))),
                    &expression!(&literal!(TokenType::Number(6.0)))
                )
            ),
            
            token!(TokenType::RightParen),
            
            token!(TokenType::Star),
            
            token!(TokenType::LeftParen),
            
            expression!(
                &binary!(
                    &token!(TokenType::Plus),
                    &expression!(&literal!(TokenType::Number(2.0))),
                    &expression!(&literal!(TokenType::Number(3.0)))
                )
            ),
            
            token!(TokenType::RightParen)
        ]);

        match Grouping::process_set(&mut tokens) {
            Ok(b) => assert!(b),
            Err(error) => panic!("ERROR : {}",error),
        }

        assert_eq!(3, tokens.len());
    }
}