type T = CodeWrap<SyntaxElement>;

use crate::syntax::{SyntaxResult, SyntaxElement};
use crate::codewrap::CodeWrap;
use crate::token::Token;

pub fn process(elements : &mut Vec<T>) -> SyntaxResult {
    //! works through the elements and checks if we are working with a var.
    //!
    //! [x] Name
    //! [ ] prefixexp `[´ exp `]´
    //! [ ] prefixexp `.´ Name

    // looks for Name
    // Name is another name for Identifier
    for i in 0 .. elements.len() {
        let found : bool = if let CodeWrap::CodeWrap(SyntaxElement::Token(Token::Identifier(_)),_,_) = elements[i] { true }
                           else { false };

        if found {
            let CodeWrap::CodeWrap(token, start, end) = elements.remove(i);
            elements.insert(i, CodeWrap::CodeWrap(SyntaxElement::Var(Box::new(token)), start, end));
            return SyntaxResult::Done; // we leave saying that we made a change 
        }
    }

    // looks for prefixexp [exp]
    // looks for prefixexp.Name

    SyntaxResult::None
}