pub type T = CodeWrap<SyntaxElement>;

use crate::token::Token;
use crate::codewrap::CodeWrap;
use crate::syntax::{SyntaxResult, SyntaxElement, final_compress};

pub fn process(elements : &mut Vec<T>) -> SyntaxResult {

    if elements.len() == 3 { if is_a_while_loop(&elements[0],&elements[1],&elements[2]) { 

        if let CodeWrap::CodeWrap(SyntaxElement::Token(Token::While), start, end) = elements[0] {
            // we found the while block at the first token of the phrase, which means we are in
            // a do ... end block

            if elements.len() != 3 {
                return SyntaxResult::Error(start,end, "malformed while .. do .. end".to_string());
            }

            elements.remove(0);

            let CodeWrap::CodeWrap(exp,s,e) = elements.remove(0);
            if !exp.is_exp() {
                return SyntaxResult::Error(s,e,"the condition of a while loop must be an expression".to_string());
            }

            let element = SyntaxElement::StatementWhile(
                Box::new(exp),
                Box::new(SyntaxElement::Empty),
            ) ;

            // we remove the do so there is no confusion
            elements.remove(0);

            // we are going to return this wrap to add to the block_stack and make all
            // subsequent stuff added to this element in the stack. we have to then check for 
            // the ending of this loop every phrase we check to make sure we correctly close it.
            return SyntaxResult::Wrap(CodeWrap::CodeWrap(element, start, end));
        }
    }}

    SyntaxResult::None
}

fn is_a_while_loop(while_token : &T, exp : &T, do_token : &T) -> bool {
    match (while_token.item(), do_token.item()) {
        ( SyntaxElement::Token(Token::While), SyntaxElement::Token(Token::Do) ) => {
            exp.item().is_exp()
        },
        _ => false
    }
}

pub fn finalize(condition : Box<SyntaxElement>, stack : &mut Vec<CodeWrap<SyntaxElement>>) -> SyntaxResult {

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

    SyntaxResult::Ok(SyntaxElement::StatementWhile(condition, Box::new(inner_block)))
}