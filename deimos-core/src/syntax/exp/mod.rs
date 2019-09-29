mod token;
mod unop;
mod binop;

use crate::syntax::SyntaxElement;
use crate::codewrap::CodeWrap;
use crate::token::Token;

pub fn process(phrase : &mut Vec<CodeWrap<SyntaxElement>>) -> bool {
    //! works through the elements and checks if any of the following 
    //! are matched, returns true if it reduces something
    //!
    //! [x] nil | 
    //! [x] false | 
    //! [x] true | 
    //! [x] Number | 
    //! [ ] String | 
    //! [x] `...´ | 
    //! [ ] function | 
    //! [ ] prefixexp | 
    //! [ ] tableconstructor | 
    //! [x] exp binop exp | 
    //! [x] unop exp 
    
    let mut count = 0;
    loop {
        // the intent of this loop is to work our way down the list,
        // and when we actually process one then we go back to the top
        // of the list and start over, if we go done the list and dont
        // process anyhting then we just leave
        
        count += 1;

        // checks for nil
        if token::process(phrase, Token::Nil) { continue; }
        // checks for false
        if token::process(phrase, Token::False) { continue; }
        // checks for true
        if token::process(phrase, Token::True) { continue; }
        // checks for Number
        if token::process(phrase, Token::Number(0.0)) { continue; }
        // checks for String
        // checks for `...`
        if token::process(phrase, Token::TriplePeriod) { continue; }
        // checks for function
        // checks for prefixexp
        // checks for tableconstructor   
        // checks for binop
        if binop::process(phrase) { continue; }
        // checks for unop
        if unop::process(phrase) { continue; }

        break;
    }

    if count > 1 { true } 
    else { false }
}