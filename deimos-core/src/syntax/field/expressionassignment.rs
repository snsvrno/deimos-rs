pub type T = CodeWrap<SyntaxElement>;

use crate::token::Token;
use crate::codewrap::CodeWrap;
use crate::syntax::SyntaxElement;

pub fn process(elements : &mut Vec<T>) -> bool {
    //! checks for an expression assignment and wraps it with a field.
    //! 
    //! `[´ exp `]´ `=´ exp 
    
    if elements.len() > 4 { for i in 0 .. elements.len() - 4 {
        if is_exp_assignment(&elements[i], &elements[i+1], &elements[i+2], &elements[i+3], &elements[i+4]) {
            let CodeWrap::CodeWrap(_, code_start, _) = elements.remove(i); // '['
            let CodeWrap::CodeWrap(exp1, _, _) = elements.remove(i); // 'exp'
            elements.remove(i); // ']'
            elements.remove(i); // '='
            let CodeWrap::CodeWrap(exp2, _, code_end) = elements.remove(i); // 'exp'

            let field = SyntaxElement::FieldAssignment(Box::new(exp1), Box::new(exp2));
            let element = CodeWrap::CodeWrap(field, code_start, code_end);
            elements.insert(i, element);
            return true;
        }
    }}

    // didn't find anything, so we return false
    false
}

fn is_exp_assignment(lb : &T, exp1 : &T, rb : &T, eq : &T, exp2 : &T) -> bool {
    //! checks the form to see if it fits the expression format.
    match (lb.item(), rb.item(), eq.item()) {
        // first we check the tokens because those are easiest
        (
            SyntaxElement::Token(Token::LeftBracket), 
            SyntaxElement::Token(Token::RightBracket), 
            SyntaxElement::Token(Token::Equal)
        ) => {
            return exp1.item().is_exp() && exp2.item().is_exp();
        },
        _ => { },
    }
    false
}