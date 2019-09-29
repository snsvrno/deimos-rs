pub type T = CodeWrap<SyntaxElement>;

use crate::token::Token;
use crate::codewrap::{CodeWrap, CodeWrappable};
use crate::syntax::SyntaxElement;

pub fn process(elements : &mut Vec<T>, check_token: Token) -> bool {
    //! a shared function that looks for the following token and processes it into
    //! an expression, used for the following cases
    //!         
    //! nil | false | true | Number | String | `...´
    
    for i in 0 .. elements.len() {
        let found : bool = if let CodeWrap::CodeWrap(SyntaxElement::Token(ref token),_,_) = elements[i] { 
                               check_token.is_same_type(&token) 
                           } else { false };

        if found {
            let CodeWrap::CodeWrap(token, start, end) = elements.remove(i);
            elements.insert(i, CodeWrap::CodeWrap(SyntaxElement::Exp(Box::new(token)), start, end));
            return true; // we leave saying that we made a change 
        }
    }

    // didn't find anything, so we return false
    false
}