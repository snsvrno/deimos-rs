use crate::tokentype::TokenType;
use crate::token::Token;
use crate::chunk::Chunk;

use crate::grammar::gram::Gram;
use crate::grammar::expression::Expression;

use failure::{Error,format_err};

#[derive(PartialEq,Clone,Debug)]
pub struct Binary {
    left_expr : Expression,
    operator : Token,
    right_expr : Expression,
}

impl Binary {

    // order of operation constants
    // taken from https://www.lua.org/pil/3.5.html
    const ORDER_TIER_1 : [TokenType; 1] = [ TokenType::Carrot ];
    const ORDER_TIER_3 : [TokenType; 2] = [ TokenType::Star, TokenType::Slash ];
    const ORDER_TIER_4 : [TokenType; 2] = [ TokenType::Plus, TokenType::Minus ];
    const ORDER_TIER_5 : [TokenType; 1] = [ TokenType::DoublePeriod ];
    const ORDER_TIER_6 : [TokenType; 6] = [ 
        TokenType::GreaterThan, TokenType::LessThan,
        TokenType::GreaterEqual, TokenType::LessEqual,
        TokenType::NotEqual, TokenType::EqualEqual
    ];
    const ORDER_TIER_7 : [TokenType; 1] = [ TokenType::And ];
    const ORDER_TIER_8 : [TokenType; 1] = [ TokenType::Or ];
    const ORDER_TIER_9 : [TokenType; 1] = [ TokenType::Equal ];

    const OPERATION_ORDER : [ &'static [TokenType]; 8] = [
        &Binary::ORDER_TIER_1,
        &Binary::ORDER_TIER_3,
        &Binary::ORDER_TIER_4,
        &Binary::ORDER_TIER_5,
        &Binary::ORDER_TIER_6,
        &Binary::ORDER_TIER_7,
        &Binary::ORDER_TIER_8,
        &Binary::ORDER_TIER_9
    ];
    
    pub fn create(left_token : &Gram, operator: &Gram, right_token : &Gram) -> Option<Binary> {
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
                    TokenType::Or |
                    TokenType::Equal => Some(Binary{
                        left_expr : *left_expr.clone(),
                        operator : token.clone(),
                        right_expr : *right_expr.clone(),
                    }),
                    _ => None,
                }
            }
            (_, _, _) => None,
        }
    }

    pub fn create_into_gram(left_token : &Gram, operator: &Gram, right_token : &Gram) -> Option<Gram> {
        match Binary::create(left_token,operator,right_token) {
            None => None,
            Some(binary) => Some(Gram::Binary(Box::new(binary))),
        }
    }

    pub fn process_set_increment(chunk : &mut Chunk, tier : usize) -> Result<Option<usize>,Error> {

        // needs at least chunk in order to match a binary, since the binary 
        // is 3 Expr (op) Expr, else it will just return.
        if chunk.len() < 3 { return Ok(None); }

        // goes through the order of operations, for all operations
        // let mut tier : Option<usize> = Some(0);
            
            let ops = match Binary::OPERATION_ORDER.len() > tier {
                true => Binary::OPERATION_ORDER[tier],
                false => { return Ok(None); },
            };

            // decided to put a loop in here so once we get a match we will start 
            // over again with that operator in case we were chaining that operator
            // for example : 2 + 3 + 4 + 5, would ignore (2+3) + 4 because of the 
            // way the for loop works, and in a case where there was some other operation, 
            // it could possibly perform that grouping before causing the order to not
            // be correct.
            loop {

            // used to go through this loop again if we found a match.
            // the usize is the position of the matching set of chunk
            let mut reset_loop : Option<usize> = None;

            // get a group of 3 chunk and check it against all of the operators in the group
            for i in 0 .. (chunk.len()-2) {
                // first we check if it matches the general patter for a binary,
                // if the 1st and 3rd chunk aren't expressions we move on to the next
                // group of chunk
                if !chunk.at(i).is_expression() || !chunk.at(i+2).is_expression() { continue; }
                
                // goes through each operator
                for op in ops.iter() {
                    if let Gram::Token(ref token) = chunk.at(i+1) {
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

                // removing the 3 chunk and putting them in a format that can be used.
                let mut removed_tokens : Vec<Gram> = chunk.remove(i,i+3);

                let right : Gram = if let Some(gram) = removed_tokens.pop() { gram } else { 
                    return Err(format_err!("Failed to build Binary, tried to remove 1/3 chunk but failed.")); };
                let middle : Gram = if let Some(gram) = removed_tokens.pop() { gram } else { 
                    return Err(format_err!("Failed to build Binary, tried to remove 2/3 chunk but failed.")); };
                let left : Gram = if let Some(gram) = removed_tokens.pop() { gram } else { 
                    return Err(format_err!("Failed to build Binary, tried to remove 3/3 chunk but failed.")); };

                // creates the new gram, needs to unwrap the pieces, they will error
                // if somehow we got mismatched types, but this shouldn't happen
                // because we previously check these when we were checking the operator.
                let new_gram = Gram::Binary(Box::new(Binary{
                    left_expr : left.unwrap_expr()?,
                    operator : middle.unwrap_token()?,
                    right_expr : right.unwrap_expr()?,
                }));

                match Expression::create_into_gram(&new_gram) {
                    None => return Err(format_err!("You shouldn't ever see this error!")), 
                    Some(expr_gram) => { chunk.insert(i,expr_gram); }
                }

                // need to check if we have enough chunk to actually continue, if we get less than 3 there is 
                // no way to match anything anymore so we should finish.
                if chunk.len() < 3 { return Ok(None); }

                // counts as a reset for the tier, we need to do this because we just matched an operation,
                // maybe there was another operation further up the stack that we didn't match because it
                // couldn't have matched, and we would now miss it.
                // example : 
                // tier = None;

            } else {
                break;
            }
        }
        // increment the operator tier.
        match Binary::OPERATION_ORDER.len() > tier  +1 {
            true => Ok(Some(tier + 1)),
            false => Ok(None),
        }
    }

    pub fn process_set(chunk : &mut Chunk) -> Result<(),Error> {
        let mut tier = 0;
        
        loop {
            match Binary::process_set_increment(chunk,tier)? {
                None => break,
                Some(t) => {
                    tier = t;
                }
            }
        }

        Ok(())
    }

}

impl std::fmt::Display for Binary {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"({} {} {})",self.operator,self.left_expr,self.right_expr)
    }
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! binary {
    ($op:expr, $left:expr, $right:expr) => {
        $crate::grammar::binary::Binary::create_into_gram(&$left,&$op,&$right).unwrap()
    };
}

mod tests {

    #[test]
    fn basic_parsing() {
        use crate::tokentype::TokenType;
        use crate::token::Token;
        use crate::grammar::binary::Binary;
        use crate::grammar::gram::Gram;

        let exp1 = expression!(&literal!(TokenType::Nil));
        let exp2 = expression!(&literal!(TokenType::String("What".to_string())));

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

        assert!(Binary::create(&exp1, &carrot, &exp2).is_some());
        assert!(Binary::create(&exp1, &star, &exp2).is_some());
        assert!(Binary::create(&exp1, &slash, &exp2).is_some());
        assert!(Binary::create(&exp1, &or, &exp2).is_some());
        assert!(Binary::create(&exp1, &double_period, &exp2).is_some());
        assert!(Binary::create(&exp1, &plus, &exp2).is_some());
        assert!(Binary::create(&exp1, &minus, &exp2).is_some());
        assert!(Binary::create(&exp1, &less_than, &exp2).is_some());
        assert!(Binary::create(&exp1, &and, &exp2).is_some());
        assert!(Binary::create(&exp1, &equal_equal, &exp2).is_some());
        assert!(Binary::create(&exp1, &greater_equal, &exp2).is_some());
        assert!(Binary::create(&exp1, &greater_than, &exp2).is_some());
        assert!(Binary::create(&exp1, &less_equal, &exp2).is_some());
        assert!(Binary::create(&exp1, &not_equal, &exp2).is_some());

        let left_paren = Gram::Token(Token::simple(TokenType::LeftParen));
        let not = Gram::Token(Token::simple(TokenType::Not));
        assert!(Binary::create(&exp1, &left_paren, &exp2).is_none());
        assert!(Binary::create(&exp1, &not, &exp2).is_none());
    }

    #[test]
    fn order_of_operations() {
        use crate::tokentype::TokenType;
        use crate::chunk::Chunk;

        use crate::grammar::binary::Binary;
        
        // 5 + 6 * 2 - 3
        // should do the correct order of operations and create something that looks like
        // 5 + (6 * 2) - 3
        // (5 + (6*2)) - 3
        // ((5+(6*2))-3)

        let mut tokens = Chunk::new_from(vec![
            expression!(&literal!(TokenType::Number(5.0))),
            token!(TokenType::Plus),
            expression!(&literal!(TokenType::Number(6.0))),
            token!(TokenType::Star),
            expression!(&literal!(TokenType::Number(2.0))),
            token!(TokenType::Minus),
            expression!(&literal!(TokenType::Number(3.0)))
        ]);

        if let Err(error) = Binary::process_set(&mut tokens) {
            panic!("ERROR : {}",error);
        }

        assert_eq!(1, tokens.len());
    }
}