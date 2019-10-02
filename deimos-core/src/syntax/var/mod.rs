type T = CodeWrap<SyntaxElement>;

mod name;
mod prefixname;
mod prefixexp;

use crate::syntax::{SyntaxResult, SyntaxElement};
use crate::codewrap::CodeWrap;

pub fn process(elements : &mut Vec<T>) -> SyntaxResult {
    //! works through the elements and checks if we are working with a var.
    //!
    //! [x] Name
    //! [x] prefixexp `[´ exp `]´
    //! [x] prefixexp `.´ Name
    
    let mut count = 0;
    loop {
        count += 1;

        // looks for prefixexp [exp]
        if prefixexp::process(elements) { continue; }

        // looks for prefixexp.Name
        if prefixname::process(elements) { continue; }

        // looks for Name
        if name::process(elements) { continue; }

        break;
    }

    if count > 1 { SyntaxResult::Done } 
    else { SyntaxResult::None }
}


#[cfg(test)]
mod tests {

    use crate::syntax::var::process;
    use crate::syntax::SyntaxElement;
    use crate::codewrap::CodeWrap;

    // contains all the test macros, to make the construction of tests look
    // simpler, and easier to understand the nesting.
    use crate::{
        identifier, token, prefixexp, exp, var, number,
        test_process,
    };

    #[test]
    pub fn name() {
        let mut input_tokens : Vec<crate::codewrap::CodeWrap<SyntaxElement>> = vec![
            identifier!("bob"), identifier!("bob"), identifier!("bob"),
        ];

        // it should catch all of them the first time
        test_process!(process(&mut input_tokens), true);

        // there shouldn't be any other matches
        test_process!(process(&mut input_tokens), false);
    }

    #[test]
    pub fn prefixexp_exp() {
        let mut input_tokens : Vec<crate::codewrap::CodeWrap<SyntaxElement>> = vec![
            prefixexp!(var!(identifier!("bob"))),
            token!("["),
            exp!(number!(1.0)),
            token!("]"),
        ];

        // it should catch all of them the first time
        test_process!(process(&mut input_tokens), true);

        // it should reduce it down to 1 element
        assert_eq!(input_tokens.len(),1);

        // there shouldn't be any other matches
        test_process!(process(&mut input_tokens), false);
    }

    #[test]
    pub fn prefixexp_dot_name() {
        let mut input_tokens : Vec<crate::codewrap::CodeWrap<SyntaxElement>> = vec![
            prefixexp!(var!(identifier!("bob"))),
            token!("."),
            identifier!("bob"),
        ];

        // it should catch all of them the first time
        test_process!(process(&mut input_tokens), true);

        // it should reduce it down to 1 element
        assert_eq!(input_tokens.len(),1);

        // there shouldn't be any other matches
        test_process!(process(&mut input_tokens), false);
    }

}