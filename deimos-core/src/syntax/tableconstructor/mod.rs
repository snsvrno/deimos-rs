
use crate::syntax::{SyntaxElement,SyntaxResult};
use crate::codewrap::CodeWrap;
use crate::token::Token;

pub fn process(elements : &mut Vec<CodeWrap<SyntaxElement>>) -> SyntaxResult {
    //! works through the elements and checks if we are trying to make
    //! a table
    //!
    //! [ ] tableconstructor ::= `{´ [fieldlist] `}´
    //! [ ] fieldlist ::= field {fieldsep field} [fieldsep]
    
    for i in 0 .. elements.len() {
        if let CodeWrap::CodeWrap(SyntaxElement::Token(Token::LeftMoustache),_,_) = elements[i] {
            return SyntaxResult::TableConst(i);
        } 
    }   

    SyntaxResult::None
}

/*
pub fn process(elements : &mut Vec<CodeWrap<SyntaxElement>>) -> SyntaxResult {
    //! works through the elements and checks if we are trying to make
    //! a table
    //!
    //! [ ] tableconstructor ::= `{´ [fieldlist] `}´
    //! [ ] fieldlist ::= field {fieldsep field} [fieldsep]
    //! [ ] field ::= `[´ exp `]´ `=´ exp 
    //! [ ]           Name `=´ exp 
    //! [ ]           exp
    
    let mut cursor : usize = 0;
    // contains where the first '{' is and where the '}' is
    let mut instructions : Option<(usize, usize)>  = None;

    loop {
        // leaves if we reach the end
        if cursor >= elements.len() { break; }

        if let CodeWrap::CodeWrap(SyntaxElement::Token(Token::LeftMoustache),_,_) = elements[cursor] {

            // we mark that we found the beginning.
            let cursor_starting_point : usize = cursor;
            // a nesting counter, so we don't just stop at the first '}' we see
            // set at 0 because we are using the same token to start the next 
            // inner loop
            let mut nesting_counter : usize = 0;
            loop {
                // we are going to try and find the ending token, 
                // if we close it and find both the start
                // and the end then we continue. if we don't then
                // we will send out of the function asking for more tokens.
                
                // leaves if we reach the end
                if cursor >= elements.len() { break; }

                match elements[cursor] {
                    CodeWrap::CodeWrap(SyntaxElement::Token(Token::LeftMoustache),_,_) => nesting_counter += 1,
                    CodeWrap::CodeWrap(SyntaxElement::Token(Token::RightMoustache),_,_) => { 
                        nesting_counter -= 1;

                        if nesting_counter == 0 {
                            // we found the end, we need to do something here.
                            instructions = Some((cursor_starting_point, cursor));
                            cursor = elements.len(); // so we leave all these loops.
                        }
                    },
                    _ => { },
                }
                
                cursor += 1;
            }

            // if we are here we found the start but never found the end
            // so we need to ask the parser for more tokens
            if instructions.is_none() { return SyntaxResult::More; }

            break;
        }

        cursor += 1;
    }

    println!("{:?}",instructions);

    if let Some((start, end)) = instructions {
        let mut inner_tokens : Vec<CodeWrap<SyntaxElement>> = elements.drain(start .. end).collect();
        let CodeWrap::CodeWrap(_, _, code_end) = inner_tokens.remove(inner_tokens.len()-1);
        let CodeWrap::CodeWrap(_, code_start, _) = inner_tokens.remove(0);

        println!("inside of the table");
        for i in inner_tokens.iter() {
            println!("     {}",i.item());
        }
    }


    SyntaxResult::None
}*/