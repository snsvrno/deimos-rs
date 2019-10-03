pub type T = CodeWrap<SyntaxElement>;

use crate::token::Token;
use crate::codewrap::CodeWrap;
use crate::syntax::SyntaxElement;

pub fn process(elements : &mut Vec<T>) -> bool {
    //! checks for an expression and wraps it with a field.
    //! 
    //! exp
    
    for i in 0 .. elements.len() {
        if elements[i].item().is_exp() {
            let CodeWrap::CodeWrap(expression,s,e) = elements.remove(i);
            let field = SyntaxElement::Field(Box::new(expression));
            let element = CodeWrap::CodeWrap(field, s, e);
            elements.insert(i,element);

            return true;
        }
    }

    // didn't find anything, so we return false
    false
}