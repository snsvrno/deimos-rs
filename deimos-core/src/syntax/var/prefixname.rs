pub type T = CodeWrap<SyntaxElement>;

use crate::token::Token;
use crate::codewrap::CodeWrap;
use crate::syntax::SyntaxElement;

pub fn process(elements : &mut Vec<T>) -> bool {
    //! prefixexp `.´ Name
    
    // will not check anything if its too short to match the basic 
    // phrase (needs 3 characters)
    if elements.len() < 3 { return false; }

    for i in 0 .. elements.len() - 2 {

        if can_reduce_to_var(&elements[i], &elements[i+1], &elements[i+2]) {
            let CodeWrap::CodeWrap(prefixexp, start, _) = elements.remove(i);   // the `prefixexpr`
            elements.remove(i);     // the dot
            let CodeWrap::CodeWrap(name, _, end) = elements.remove(i);     // the `exp`

            let var = SyntaxElement::VarDot(Box::new(prefixexp), Box::new(name));
            elements.insert(i, CodeWrap::CodeWrap(var, start, end));
            return true;
        }
    }

    // didn't find anything, so we return false
    false
}

fn can_reduce_to_var(prefixexp : &T, dot : &T, name : &T) -> bool {
    println!("{:?},{:?},{:?}",prefixexp.item(), dot.item(), name.item());
    match (prefixexp.item(), dot.item(), name.item()) {
        (
            SyntaxElement::PrefixExp(_), 
            SyntaxElement::Token(Token::Period), 
            SyntaxElement::Token(Token::Identifier(_))
        ) => true,
        _ => false,
    }
}