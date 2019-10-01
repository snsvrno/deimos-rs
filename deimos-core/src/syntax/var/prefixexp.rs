pub type T = CodeWrap<SyntaxElement>;

use crate::token::Token;
use crate::codewrap::CodeWrap;
use crate::syntax::SyntaxElement;

pub fn process(elements : &mut Vec<T>) -> bool {
    //! prefixexp `.´ Name
    
    // will not check anything if its too short to match the basic 
    // phrase (needs 3 characters)
    if elements.len() < 4 { return false; }

    for i in 0 .. elements.len() - 3 {

        if can_reduce_to_var(&elements[i], &elements[i+1], &elements[i+2], &elements[i+3]) {
            let CodeWrap::CodeWrap(prefixexp, start, _) = elements.remove(i);   // the `prefixexpr`
            elements.remove(i);     // the [
            let CodeWrap::CodeWrap(exp, _, end) = elements.remove(i);     // the `exp`
            elements.remove(i);     // the ]

            let var = SyntaxElement::VarIndex(Box::new(prefixexp), Box::new(exp));
            elements.insert(i, CodeWrap::CodeWrap(var, start, end));
            return true;
        }
    }

    // didn't find anything, so we return false
    false
}

fn can_reduce_to_var(prefixexp : &T, lb : &T, exp : &T, rb : &T) -> bool {
    
    match (prefixexp.item(), lb.item(), exp.item(), rb.item()) {
        (
            SyntaxElement::PrefixExp(_), 
            SyntaxElement::Token(Token::LeftBracket), 
            SyntaxElement::Exp(_),
            SyntaxElement::Token(Token::RightBracket)
        ) => true,
        _ => false,
    }
}