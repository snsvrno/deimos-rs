pub type T = CodeWrap<SyntaxElement>;

use crate::token::Token;
use crate::codewrap::CodeWrap;
use crate::syntax::SyntaxElement;

pub fn process(elements : &mut Vec<T>) -> bool {
    //! attemps to combine elements into a binary op

    // checks for binary ops
    if elements.len() > 2 { for i in 0 .. elements.len() - 2 {

        if can_reduce_to_exp_binop(&elements[i], &elements[i+1], &elements[i+2]) {
            let CodeWrap::CodeWrap(left, start, _) = elements.remove(i);
            let CodeWrap::CodeWrap(op, _, _) = elements.remove(i);
            let CodeWrap::CodeWrap(right, _, end) = elements.remove(i);

            // we make the new SyntaxElement element, and add it where 
            // we took it off
            let new_op = SyntaxElement::Binop(Box::new(left),Box::new(op),Box::new(right));
            // since a binop is an expression, and the rest of the matches are expecting this
            // we will wrap it now.
            let new_exp = SyntaxElement::Exp(Box::new(new_op));
            elements.insert(i,CodeWrap::CodeWrap(new_exp, start, end));
            
            return true; // we leave saying that we made a change 
        }
    }}

    // didn't find anyhting, so we return false
    false
}

fn can_reduce_to_exp_binop(left : &T, op : &T, right : &T) -> bool {
    //! checks if the three SyntaxElements can become a binary operation
    //! (binop)
    
    if let SyntaxElement::Token(ref token) = op.item() {
        match token {
            Token::Plus | Token::Minus | Token::Star | Token::Slash |
            Token::Carrot | Token::Percent | Token::DoublePeriod |
            Token::LessThan | Token::LessEqual | Token::GreaterThan |
            Token::GreaterEqual | Token::EqualEqual | Token::NotEqual |
            Token::And | Token::Or => match (left.item().is_exp(), right.item().is_exp()) {
                // checking if the two other parts are expressions
                (true, true) => return true,
                _ => return false,
            }
            _ => return false,
        }
    }
    false
}