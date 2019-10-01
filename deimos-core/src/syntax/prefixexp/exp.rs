pub type T = CodeWrap<SyntaxElement>;

use crate::token::Token;
use crate::codewrap::CodeWrap;
use crate::syntax::SyntaxElement;

pub fn process(elements : &mut Vec<T>) -> bool {
    //! `(´ exp `)´
    
    // will not check anything if its too short to match the basic 
    // phrase (needs 3 characters)
    if elements.len() < 3 { return false; }

    for i in 0 .. elements.len() - 2 {

        if can_reduce_to_prefixexp(&elements[i], &elements[i+1], &elements[i+2]) {
            let CodeWrap::CodeWrap(_, start, _) = elements.remove(i);   // the `(`
            let CodeWrap::CodeWrap(exp, _, _) = elements.remove(i);     // the expression
            let CodeWrap::CodeWrap(_, _, end) = elements.remove(i);     // the `)`

            let prefix_exp = SyntaxElement::PrefixExp(Box::new(exp));
            elements.insert(i, CodeWrap::CodeWrap(prefix_exp, start, end));
            return true;
        }

    }

    // didn't find anything, so we return false
    false
}

fn can_reduce_to_prefixexp(left : &T, exp : &T, right : &T) -> bool {
    //! checks if the three tokens match the pattern `( exp )`
    
    match (left.item(), exp.item(), right.item()) {
        (SyntaxElement::Token(Token::LeftParen), exp, SyntaxElement::Token(Token::RightParen)) => { 
            return exp.is_exp();
        },
        _ => { },
    }

    false
}