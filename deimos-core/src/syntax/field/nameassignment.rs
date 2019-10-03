pub type T = CodeWrap<SyntaxElement>;

use crate::token::Token;
use crate::codewrap::CodeWrap;
use crate::syntax::SyntaxElement;

pub fn process(elements : &mut Vec<T>) -> bool {
    //! checks for an expression assignment and wraps it with a field.
    //! 
    //! Name `=´ exp 
    
    if elements.len() > 2 { for i in 0 .. elements.len() - 2 {
        if is_exp_assignment(&elements[i], &elements[i+1], &elements[i+2]) {
            let CodeWrap::CodeWrap(name, code_start, _) = elements.remove(i); // 'exp'
            elements.remove(i); // '='
            let CodeWrap::CodeWrap(exp, _, code_end) = elements.remove(i); // 'exp'

            let field = SyntaxElement::FieldAssignment(Box::new(name), Box::new(exp));
            let element = CodeWrap::CodeWrap(field, code_start, code_end);
            elements.insert(i, element);
            return true;
        }
    }}

    // didn't find anything, so we return false
    false
}

fn is_exp_assignment(name : &T, eq : &T, exp : &T) -> bool {
    //! checks the form to see if it fits the expression format.
    match eq.item() {
        SyntaxElement::Token(Token::Equal)=> {
            // need to be able to nest inside of things to see if they are a name at heart
            // the parser will process vars and exprs first before we get here, so we need
            // to undo some of the work that has been done before.
            let is_name_a_name : bool = {
                match name.item().is_name() {
                    true => true,
                    false => {
                        let mut we_found_the_var : bool = false;

                        // now to check if there is a var nested in there somewhere.
                        let CodeWrap::CodeWrap(ref name_insides, _, _) = name;
                        let mut insides_ref : &SyntaxElement = name_insides;
                        loop {
                            match insides_ref.insides_single_only() {
                                None => break,
                                Some(other_insides) => {
                                    if let SyntaxElement::Var(_) = other_insides {
                                        we_found_the_var = true;
                                        break;
                                    } else {
                                        insides_ref = other_insides;
                                        continue;
                                    }
                                }
                            }
                        }

                        we_found_the_var
                    }
                }
            };

            return is_name_a_name && exp.item().is_exp();
        },
        _ => { },
    }
    false
}