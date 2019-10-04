pub type T = CodeWrap<SyntaxElement>;

use crate::token::Token;
use crate::codewrap::CodeWrap;
use crate::syntax::{SyntaxResult, SyntaxElement};

pub fn process(elements : &mut Vec<T>) -> SyntaxResult {

    if elements.len() > 0 { 

        if let CodeWrap::CodeWrap(SyntaxElement::Token(Token::Do), start, end) = elements[0] {
            // we found the do block at the first token of the phrase, which means we are in
            // a do ... end block
            let element = SyntaxElement::StatementDoEnd(Box::new(SyntaxElement::Token(Token::Do))) ;

            // we are going to return this wrap to add to the block_stack and make all
            // subsequent stuff added to this element in the stack. we have to then check for 
            // the ending of this loop every phrase we check to make sure we correctly close it.
            return SyntaxResult::Wrap(CodeWrap::CodeWrap(element, start, end));
        }
    }

    SyntaxResult::None
}