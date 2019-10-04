pub type T = CodeWrap<SyntaxElement>;

use crate::token::Token;
use crate::codewrap::CodeWrap;
use crate::syntax::{SyntaxResult, SyntaxElement, final_compress};

pub fn process(elements : &mut Vec<T>) -> SyntaxResult {

    if elements.len() > 0 { 

        if let CodeWrap::CodeWrap(SyntaxElement::Token(Token::Do), start, end) = elements[0] {
            // we found the do block at the first token of the phrase, which means we are in
            // a do ... end block
            let element = SyntaxElement::StatementDoEnd(Box::new(SyntaxElement::Empty)) ;

            // we are going to return this wrap to add to the block_stack and make all
            // subsequent stuff added to this element in the stack. we have to then check for 
            // the ending of this loop every phrase we check to make sure we correctly close it.
            return SyntaxResult::Wrap(CodeWrap::CodeWrap(element, start, end));
        }
    }

    SyntaxResult::None
}

pub fn finalize(stack : &mut Vec<CodeWrap<SyntaxElement>>) -> SyntaxResult {
    
    // makes a block out of all the inside pieces
    let inner_block = match final_compress(stack) {
        SyntaxResult::Error(error_start, error_end, description) => return SyntaxResult::Error(error_start, error_end, description),
        SyntaxResult::Wrap(CodeWrap::CodeWrap(inner_block, _, _)) => inner_block,
        _ => unimplemented!(),
    };

    // checks if the insides are a block, because they need to be a block
    if !inner_block.is_block() {
        return SyntaxResult::Error(0,0,"must be able to reduce down to a block".to_string());
    }

    // creates the piece we will inject upwards.
    SyntaxResult::Ok(SyntaxElement::StatementDoEnd(Box::new(inner_block)))
}