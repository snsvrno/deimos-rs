
use crate::syntax::{SyntaxElement,SyntaxResult, fieldlist, field};
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

pub fn finalize(stack : &mut Vec<CodeWrap<SyntaxElement>>) -> SyntaxResult {

    // so this will probably be expression lists here?
    // TODO : make this not as hacky?
    // we are going to explode the expression lists back into
    // expressions and then reprocess them as fields and fieldlist

    let mut cursor : usize  = 0;
    loop {
        if cursor >= stack.len() { break; }

        if stack[cursor].item().is_exp_list()  {
            // we found an expression list
            if let CodeWrap::CodeWrap(SyntaxElement::ExpList(mut list),s,e) = stack.remove(cursor) {
                let mut first : bool = true;
                loop {
                    // we remove the last item, and add it to the stack
                    if list.len() == 0 { break; }
                    let item_pos = list.len()-1;
                    let item = list.remove(item_pos);
                    stack.insert(cursor, CodeWrap::CodeWrap(*item, s,e));

                    // and we also need to add a comma, because it was removed
                    // when we created the explist earlier in `parse()`
                    if !first {
                        stack.insert(cursor+1, CodeWrap::CodeWrap(SyntaxElement::Token(Token::Comma),s,e));
                    } else { first = false; }
                }
            }
        }

        cursor += 1;
    }

    loop {

        // checks for fieldlist
        match fieldlist::process(stack) { 
            SyntaxResult::Done => continue,
            // TODO : need to implement the error here
            _ => { },
        }

        // checks for fields
        if field::process(stack) { continue; }

        break;
    }

    let new_code_wrapped_item = match stack.len() {
        1 => {
            let CodeWrap::CodeWrap(list, _, _) = stack.remove(0);

            SyntaxElement::TableConstructor(Box::new(list))
        },
        _ => {
            // if it is any other length than 1, something didn't work
            // it is suppose to be a fieldlist, so a single item
            // that we can put inside the table constructor.
            return SyntaxResult::Error(0,0, "table definition must be a field list!".to_string());
        }
    };

    SyntaxResult::Ok(new_code_wrapped_item)
}