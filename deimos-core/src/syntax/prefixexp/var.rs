pub type T = CodeWrap<SyntaxElement>;

use crate::codewrap::CodeWrap;
use crate::syntax::SyntaxElement;

pub fn process(elements : &mut Vec<T>) -> bool {
    //! checking if the element is a var and then 
    //! converts it to a prefixexp
    
    for i in 0 .. elements.len() {
        if elements[i].item().is_var() {
            // if we find a var then we remove the element, wrap it 
            // in a prefixexp and then put it back in.
            
            let CodeWrap::CodeWrap(element, start, end) = elements.remove(i);
            let prefixexp = SyntaxElement::PrefixExp(Box::new(element));
            elements.insert(i, CodeWrap::CodeWrap(prefixexp, start, end));
            return true;
        }
    }

    // didn't find anything, so we return false
    false
}