pub type T = CodeWrap<SyntaxElement>;

use crate::token::Token;
use crate::codewrap::CodeWrap;
use crate::syntax::SyntaxElement;

pub fn process(elements : &mut Vec<T>) -> bool {
    //! checks for unary ops
        
    if elements.len() > 1 { for i in 0 .. elements.len() - 1 {

        if can_reduce_to_exp_unop(&elements[i], &elements[i+1]) {
            let CodeWrap::CodeWrap(op, start, _) = elements.remove(i);
            let CodeWrap::CodeWrap(right, _, end) = elements.remove(i);

            // we make the new SyntaxElement element, and add it where 
            // we took it off
            let new_op = SyntaxElement::Unop(Box::new(op),Box::new(right));
            // since a unop is an expression, and the rest of the matches are expecting this
            // we will wrap it now.
            let new_exp = SyntaxElement::Exp(Box::new(new_op));
            elements.insert(i,CodeWrap::CodeWrap(new_exp, start, end));

            return true;  // we leave saying that we made a change   
        }
    }}

    // didn't find anyhting, so we return false
    false
}


fn can_reduce_to_exp_unop(op : &T, right : &T) -> bool {
    //! checks if the two SyntaxElements can become a unary operation
    //! (unop)
    
    if let SyntaxElement::Token(ref token) = op.item() {
        match token {
            Token::Minus | Token::Not | Token::Pound => match right.item().is_exp() {
                // checking if the other part is an expression
                true => return true,
                _ => return false,
            }
            _ => return false,
        }
    }
    false
}