pub type T = CodeWrap<SyntaxElement>;

use crate::token::Token;
use crate::codewrap::CodeWrap;
use crate::syntax::SyntaxElement;

pub fn process(elements : &mut Vec<T>) -> bool {
    //! checks if the item is a table constructor.
    
    for i in 0 .. elements.len() {
        if elements[i].item().is_table() {
            let CodeWrap::CodeWrap(table, code_start, code_end) = elements.remove(i);
            let exp = SyntaxElement::Exp(Box::new(table));
            let element = CodeWrap::CodeWrap(exp, code_end, code_end);
            elements.insert(i, element);

            return true;
        }
    }

    // didn't find anything, so we return false
    false
}