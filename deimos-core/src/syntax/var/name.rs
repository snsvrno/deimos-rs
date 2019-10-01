pub type T = CodeWrap<SyntaxElement>;

use crate::codewrap::CodeWrap;
use crate::syntax::SyntaxElement;
use crate::token::Token;

pub fn process(elements : &mut Vec<T>) -> bool {
    //! checking if the element is a var and then 
    //! converts it to a prefixexp
    
    // Name is another name for Identifier
    for i in 0 .. elements.len() {
        let found : bool = if let CodeWrap::CodeWrap(SyntaxElement::Token(Token::Identifier(_)),_,_) = elements[i] { true }
                           else { false };

        if found {
            let CodeWrap::CodeWrap(token, start, end) = elements.remove(i);
            elements.insert(i, CodeWrap::CodeWrap(SyntaxElement::Var(Box::new(token)), start, end));
            return true; // we leave saying that we made a change 
        }
    }

    // didn't find anything, so we return false
    false
}


