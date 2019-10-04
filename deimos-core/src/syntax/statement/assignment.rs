pub type T = CodeWrap<SyntaxElement>;

use crate::token::Token;
use crate::codewrap::CodeWrap;
use crate::syntax::SyntaxElement;

pub fn process(elements : &mut Vec<T>) -> bool {
    if elements.len() == 3 { 

        // if the left is an varlist
        if can_reduce_to_statement_assign(&elements[0], &elements[1], &elements[2]) {
            let CodeWrap::CodeWrap(v_list,start,_) = elements.remove(0);
            elements.remove(0); // getting rid of the equals sign
            let CodeWrap::CodeWrap(e_list,_,end) = elements.remove(0);

            let new_element = SyntaxElement::StatementAssign(Box::new(v_list), Box::new(e_list));
            elements.push(CodeWrap::CodeWrap(new_element, start, end));

            return true;
        }


        // if the left is a namelist, we need to convert the namelist to an varlist
        if can_reduce_to_statement_assign_name_list(&elements[0], &elements[1], &elements[2]) {
            let CodeWrap::CodeWrap(n_list,start,_) = elements.remove(0);
            elements.remove(0); // getting rid of the equals sign
            let CodeWrap::CodeWrap(e_list,_,end) = elements.remove(0);

            // converts the name_list
            let v_list = if let SyntaxElement::NameList(mut list) = n_list {
                let mut new_list : Vec<Box<SyntaxElement>> = Vec::new();
                for i in list.drain(..) {
                    match i.convert_to_var() {
                        Ok(var) => new_list.push(Box::new(var)),
                        Err(_) => unimplemented!(),
                    }
                }

                SyntaxElement::ExpList(new_list)
            } else { unimplemented!(); };

            let new_element = SyntaxElement::StatementAssign(Box::new(v_list), Box::new(e_list));
            elements.push(CodeWrap::CodeWrap(new_element, start, end));

            return true;
        }
    }

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

fn can_reduce_to_statement_assign_name_list(name_list : &T, op : &T, exp_list : &T) -> bool {
    //! checks if the three given items match what they need to be in order
    //! to be a assignment statement

    match op {
        CodeWrap::CodeWrap(SyntaxElement::Token(Token::Equal),_,_) => {
            // if the middle item is an equal sign, then its worth checking the rest of the tokens.
            if name_list.item().is_name_list() && exp_list.item().is_exp_list() { return true; }
        },
        _ => { }
    }

    false
}