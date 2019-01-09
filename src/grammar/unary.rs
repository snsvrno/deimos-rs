use crate::tokentype::TokenType;
use crate::token::Token;
use crate::grammar::gram::Gram;
use crate::grammar::expression::Expression;

use failure::{Error,format_err};

#[derive(PartialEq,Clone,Debug)]
pub struct Unary {
    modifier : Token,
    expr : Expression,
}

impl Unary {
    
    // order of operation constants
    // taken from https://www.lua.org/pil/3.5.html
    const ORDER_TIER_2 : [TokenType; 3] = [ TokenType::Minus, TokenType::Not, TokenType::Local ];
    const OPERATION_ORDER : [ &'static [TokenType]; 1] = [
        &Unary::ORDER_TIER_2,
    ];

    pub fn create(left_token : &Gram, right_token : &Gram) -> Option<Unary> {

        match (left_token, right_token) {
            (Gram::Token(token), Gram::Expression(expr)) => {
                for tier in Unary::OPERATION_ORDER.iter() {
                    // might as well use the ones defined up there and not define it twice.
                    for op in tier.iter() {
                        if token == op {
                            return Some(Unary{
                                modifier : token.clone(),
                                expr : *expr.clone(),
                            });
                        }
                    }
                }
            },
            _ => (),
        }
        None
    }

    pub fn create_into_gram(left_token : &Gram, right_token : &Gram) -> Option<Gram> {
        match Unary::create(left_token,right_token) {
            None => None,
            Some(unary) => Some(Gram::Unary(Box::new(unary)))
        }
    }

    pub fn process_set(grams : &mut Vec<Gram>) -> Result<(),Error> {

        // needs at least Grams in order to match a binary, since the binary 
        // is 3 Expr (op) Expr, else it will just return.
        if grams.len() < 2 { return Ok(()); }

        // goes through the order of operations, for all operations
        let mut tier : Option<usize> = Some(0);
        loop {
            
            let ops = match tier {
                Some(t) => {
                    match Unary::OPERATION_ORDER.len() > t {
                        true => Unary::OPERATION_ORDER[t],
                        false => break,
                    }
                },
                None => return Err(format_err!("Tier is None!! Shouldn't have happened.")),
            };

            // decided to put a loop in here so once we get a match we will start 
            // over again with that operator in case we were chaining that operator
            // for example : 2 + 3 + 4 + 5, would ignore (2+3) + 4 because of the 
            // way the for loop works, and in a case where there was some other operation, 
            // it could possibly perform that grouping before causing the order to not
            // be correct.
            loop {

                // used to go through this loop again if we found a match.
                // the usize is the position of the matching set of Grams
                let mut reset_loop : Option<usize> = None;

                // get a group of 3 grams and check it against all of the operators in the group
                for i in 0 .. (grams.len()-1) {
                    // first we check if it matches the general patter for a binary,
                    // if the 1st and 3rd grams aren't expressions we move on to the next
                    // group of grams
                    if !grams[i].is_token() || !grams[i+1].is_expression() { continue; }
                    
                    // goes through each operator
                    for op in ops.iter() {
                        if let Gram::Token(ref token) = grams[i] {
                            if token.get_type() == op {
                                // found a match!

                                // resetting the loop
                                reset_loop = Some(i);
                                break;
                            }
                        }
                    }

                    // continuing to break the loop from a positive operator match
                    if reset_loop.is_some() { break; }
                }

                // modifying the gram vec if we found a match in the above loop
                if let Some(i) = reset_loop {

                    // removing the 3 Grams and putting them in a format that can be used.
                    let mut removed_tokens : Vec<Gram> = grams.drain(i .. i + 2).collect();

                    let expr : Gram = if let Some(gram) = removed_tokens.pop() { gram } else { 
                        return Err(format_err!("Failed to build Unary, tried to remove 1/2 Grams but failed.")); };
                    let modifier : Gram = if let Some(gram) = removed_tokens.pop() { gram } else { 
                        return Err(format_err!("Failed to build Unary, tried to remove 2/2 Grams but failed.")); };

                    // creates the new gram, needs to unwrap the pieces, they will error
                    // if somehow we got mismatched types, but this shouldn't happen
                    // because we previously check these when we were checking the operator.
                    let new_gram = Gram::Unary(Box::new(Unary{
                        modifier : modifier.unwrap_token()?,
                        expr : expr.unwrap_expr()?,
                    }));

                    match Expression::create_into_gram(&new_gram) {
                        None => return Err(format_err!("You shouldn't ever see this error!")), 
                        Some(expr_gram) => { grams.insert(i,expr_gram); }
                    }

                    // need to check if we have enough Grams to actually continue, if we get less than 3 there is 
                    // no way to match anything anymore so we should finish.
                    if grams.len() < 2 { return Ok(()); }

                    // counts as a reset for the tier, we need to do this because we just matched an operation,
                    // maybe there was another operation further up the stack that we didn't match because it
                    // couldn't have matched, and we would now miss it.
                    // example : 
                    // tier = None;

                } else {

                    // should be that we looked at all of the tokens and didn't find what we 
                    // were looking for, so lets move on. 
                    //
                    // we will only be here (and always be here) when the inner loop doesn't foind a match, meaning
                    // the reset_loop var will be none, and we will be in this part. This means we went through the
                    // inner loop completely and didn't find anything, so we should break and go to the next operator
                    // set (tier)
                    break;
                }
            }
            // increment the operator tier.
            tier = match tier {
                None => Some(0),
                Some(t) => Some(t+1),
            };
        }

        Ok(())
    }

}

impl std::fmt::Display for Unary {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"({} {})",self.modifier,self.expr)
    }
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! unary {
    ($left:expr, $right:expr) => {
        $crate::grammar::unary::Unary::create_into_gram(&$left,&$right).unwrap()
    };
}

mod tests {

    #[test]
    fn basic_parsing() {
        use crate::tokentype::TokenType;
        use crate::grammar::unary::Unary;

        // depth = -0.1234
        let token_stream = vec![
            token!(TokenType::Local),
            expression!(&literal!(TokenType::Identifier("depth".to_string()))),
            token!(TokenType::Equal),
            token!(TokenType::Minus),
            expression!(&literal!(TokenType::Number(0.1234)))
        ];

        assert!(Unary::create(&token_stream[0], &token_stream[1]).is_some()); // local depth
        assert!(Unary::create(&token_stream[1], &token_stream[2]).is_none()); // depth = 
        assert!(Unary::create(&token_stream[2], &token_stream[3]).is_none()); // = -
        assert!(Unary::create(&token_stream[3], &token_stream[4]).is_some()); // - 0.1234
    }
}