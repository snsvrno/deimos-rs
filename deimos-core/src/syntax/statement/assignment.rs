pub type T = CodeWrap<SyntaxElement>;

use crate::token::Token;
use crate::codewrap::CodeWrap;
use crate::syntax::SyntaxElement;

pub fn process(elements : &mut Vec<T>) -> bool {
    if elements.len() > 2 { for i in 0 .. elements.len() - 2 {
        if can_reduce_to_statement_assign(&elements[i], &elements[i+1], &elements[i+2]) {
            let CodeWrap::CodeWrap(v_list,start,_) = elements.remove(i);
            elements.remove(i); // getting rid of the equals sign
            let CodeWrap::CodeWrap(e_list,_,end) = elements.remove(i);

            let new_element = SyntaxElement::StatementAssign(Box::new(v_list), Box::new(e_list));
            elements.insert(i, CodeWrap::CodeWrap(new_element, start, end));

            return true;
        }
    }}

    false
}

fn can_reduce_to_statement_assign(var_list : &T, op : &T, exp_list : &T) -> bool {
    //! checks if the three given items match what they need to be in order
    //! to be a assignment statement

    match op {
        CodeWrap::CodeWrap(SyntaxElement::Token(Token::Equal),_,_) => {
            // if the middle item is an equal sign, then its worth checking the rest of the tokens.
            if var_list.item().is_var_list() && exp_list.item().is_exp_list() { return true; }
        },
        _ => { }
    }

    false
}