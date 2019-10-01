mod var;
mod exp;

use crate::syntax::SyntaxElement;
use crate::codewrap::CodeWrap;

pub fn process(phrase : &mut Vec<CodeWrap<SyntaxElement>>) -> bool {
    //! works through the elements and checks if any of the following 
    //! are matched, returns true if it reduces something
    //!
    //! [x] var
    //! [ ] functioncall
    //! [x] `(´ exp `)´
    
    let mut count = 0;
    loop {
        // the intent of this loop is to work our way down the list,
        // and when we actually process one then we go back to the top
        // of the list and start over, if we go done the list and dont
        // process anyhting then we just leave
        
        count += 1;

        // var
        if var::process(phrase) { continue; }

        // functioncall

        // `(´ exp `)´
        if exp::process(phrase) { continue; }

        break;
    }

    if count > 1 { true } 
    else { false }
}