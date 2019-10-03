pub type T = CodeWrap<SyntaxElement>;

mod expression;
mod expressionassignment;
mod nameassignment;

use crate::syntax::SyntaxElement;
use crate::codewrap::CodeWrap;

pub fn process(elements : &mut Vec<T>) -> bool {
    //! works through the elements and checks if we are working with a field.
    //!
    //! [x] `[´ exp `]´ `=´ exp 
    //! [x] Name `=´ exp 
    //! [x] exp
    
    let mut count = 0;
    loop {
        count += 1;

        // looks for expression assignment
        if expressionassignment::process(elements) { continue; }

        // looks for a name assignment
        if nameassignment::process(elements) { continue; }

        // looks for exp
        if expression::process(elements) { continue; }

        break;
    }

    if count > 1 { true } 
    else { false }
}
