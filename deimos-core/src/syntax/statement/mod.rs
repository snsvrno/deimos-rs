mod assignment;
mod doend;
mod localassignment;

use crate::syntax::{SyntaxResult, SyntaxElement};
use crate::codewrap::CodeWrap;
use crate::token::Token;

pub fn process(phrase : &mut Vec<CodeWrap<SyntaxElement>>) -> SyntaxResult {
    //! works through thelements and checks if any of the following 
    //! are matched, returns true iof it reduces something to a statement
    //! 
    //! [x] varlist `=´ explist | 
    //! [ ] functioncall | 
    //! [x] do block end | 
    //! [ ] while exp do block end | 
    //! [ ] repeat block until exp | 
    //! [ ] if exp then block {elseif exp then block} [else block] end | 
    //! [ ] for Name `=´ exp `,´ exp [`,´ exp] do block end | 
    //! [ ] for namelist in explist do block end | 
    //! [ ] function funcname funcbody | 
    //! [ ] local function Name funcbody | 
    //! [x] local namelist [`=´ explist] 
    
    let mut count = 0;
    loop {

        count += 1;

        // varlist `=´ explist
        if assignment::process(phrase) { continue; }
        // functioncall | 
        // do block end | 
        match doend::process(phrase) {
            SyntaxResult::Done => continue,
            SyntaxResult::Wrap(wrapped) => return SyntaxResult::Wrap(wrapped),
            _ => { },
        }
        // while exp do block end | 
        // repeat block until exp | 
        // if exp then block {elseif exp then block} [else block] end | 
        // for Name `=´ exp `,´ exp [`,´ exp] do block end | 
        // for namelist in explist do block end | 
        // function funcname funcbody |

        // local function Name funcbody 
        if localassignment::process(phrase) { continue; }
        
        // local namelist [`=´ explist]
        
        break;
    }

    if count > 1 { SyntaxResult::Done } 
    else { SyntaxResult::None }
}