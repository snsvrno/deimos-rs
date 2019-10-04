pub type T = CodeWrap<SyntaxElement>;

use crate::token::Token;
use crate::codewrap::CodeWrap;
use crate::syntax::SyntaxElement;

pub fn process(elements : &mut Vec<T>) -> bool {
    //! checks for local assignments
    //!
    //! [x] local namelist
    //! [x] local namelist `=´ explist

    // first we check if we have the full assignment.
    if elements.len() == 4 {
        if checks_for_assignment(&elements[0], &elements[1], &elements[2], &elements[3]) {
            let CodeWrap::CodeWrap(_,start,_) = elements.remove(0);     // the local keyword
            let CodeWrap::CodeWrap(namelist,_,_) = elements.remove(0);    // the var name
            elements.remove(0); // getting rid of the equals sign
            let CodeWrap::CodeWrap(explist,_,end) = elements.remove(0);  // the exp

            let new_element = SyntaxElement::StatementLocalAssign(
                Box::new(namelist), 
                Some(Box::new(explist)));
            elements.push(CodeWrap::CodeWrap(new_element, start, end));

            return true;
        }
    }

    // if that doesn't match then we can check if we just have the 
    // local defined with no assignment
    if elements.len() == 2 {
        if checks_for_blank_assignment(&elements[0], &elements[1]) {
            let CodeWrap::CodeWrap(_,start,_) = elements.remove(0); // the local keyword
            let CodeWrap::CodeWrap(namelist,_,end) = elements.remove(0); // the namelist

            let new_element = SyntaxElement::StatementLocalAssign(Box::new(namelist), None);
            elements.push(CodeWrap::CodeWrap(new_element, start, end));

            return true;
        }
    }

    false
}

fn checks_for_blank_assignment(local_keyword : &T, var_name : &T) -> bool {
    //! checks if the three given items match what they need to be in order
    //! to be a assignment statement
    //! 
    //! local namelist

    match local_keyword.item() {
        SyntaxElement::Token(Token::Local) => {
            return var_name.item().is_name_list();
        },
        _ => { }
    }

    false
}

fn checks_for_assignment(local_keyword : &T, var_name : &T, eq : &T, exp : &T) -> bool {
    //! checks if the three given items match what they need to be in order
    //! to be a assignment statement
    //! 
    //! local namelist `=´ explist

    match (local_keyword.item(), eq.item()) {
        ( SyntaxElement::Token(Token::Local), SyntaxElement::Token(Token::Equal) ) => {
            return var_name.item().is_name_list() && exp.item().is_exp_list();
        },
        _ => { }
    }

    false
}