use crate::syntax::{SyntaxResult, SyntaxElement};
use crate::codewrap::CodeWrap;
use crate::token::Token;

pub fn process(elements : &mut Vec<CodeWrap<SyntaxElement>>) -> SyntaxResult {
    //! works through the elements and checks if we can make an expression
    //! list, `{ }` below means 0+ occurance `[ ]` means optional (no repeatition)
    //! an expression == an expression list, an expression list != an expression
    //! 
    //! [x] fieldlist ::= field {fieldsep field} [fieldsep]

    // the intent here is to start looking for an expression, when we find one, we will
    // see how many are chained by `,` we can find and make that into a list.
    let mut i : usize = 0;
    let mut start : Option<usize> = None;
    loop {
        // check and make sure we don't access outside the array.
        if i >= elements.len() { break; }

        if start.is_none() {
            // if we haven't found the first expression, then lets 
            // check for an expression and mark it as the first one.
            if elements[i].item().is_field() {
                start = Some(i);
            }
        } else {
            // we are going to alternate tokens between ',' and 'exp'
            // so we can use %2 to tell if we should see a ',' or an 'exp'
            let factor = if let Some(start) = start { i - start } 
                         else { return SyntaxResult::Error(elements[i].start(), elements[i].end(),
                            "field list, start isn't defined".to_string()); };

            match factor % 2 {
                // these are all the odd ones, starting with the one right after the exp
                1 => if elements[i].item().does_match_token(Token::Comma) || elements[i].item().does_match_token(Token::SemiColon) { }
                     else { break; }

                // these are the even ones, should all be expressions
                0 => if elements[i].item().is_field() { } 
                     else { 
                        return SyntaxResult::Error(elements[i].start(), elements[i].end(),
                            "expecting a field after the comma/semicolon".to_string()); 
                     },

                _=> return SyntaxResult::Error(elements[i].start(), elements[i].end(),
                    "fieldlist, mod is not 1 or 0...".to_string()),
            }
        }

        i += 1;
    }
    // now we check if we have a start, if we actually have a list

    if let Some(start) = start {
        // removing the items that are part of the list
        let removed_items = elements.drain(start .. i);
        let mut cc = 0;

        // we still need to keep track of what the original source
        // code is
        let mut code_start : Option<usize> = None;
        let mut code_end : usize = 0;

        let mut new_list : Vec<Box<SyntaxElement>> = Vec::new();

        // we are dealing with `exp` and `comma` again, so we need to make sure
        // we are grabbing the right parts, that is why we are using the `cc%2`
        // so we can choose every other item. 
        for item in removed_items {
            if cc % 2 == 0 {
                let CodeWrap::CodeWrap(inside, s, e) = item;
                if code_start.is_none() { code_start = Some(s); }
                else { code_end = e; }

                new_list.push(Box::new(inside));
            }
            cc += 1;
        }

        if let Some(code_start) = code_start {
            elements.insert(start,CodeWrap::CodeWrap(SyntaxElement::FieldList(new_list),
                code_start, code_end));

            return SyntaxResult::Done;         
        }
    }

    SyntaxResult::None
}

#[cfg(test)]
mod tests {

    use crate::syntax::explist::process;
    use crate::syntax::SyntaxElement;
    use crate::codewrap::CodeWrap;

    // contains all the test macros, to make the construction of tests look
    // simpler, and easier to understand the nesting.
    use crate::{
        identifier, token, prefixexp, exp, var, number,
        test_process, binop, unop, remove_codewrap,
    };

    #[test]
    pub fn explist() {
        let mut input_tokens : Vec<crate::codewrap::CodeWrap<SyntaxElement>> = vec![
            exp!(number!(1.5)), token!(","),
            exp!(binop!(12.0,"+",34.0)), token!(","), 
            exp!(unop!("-",5.0)),
        ];

        // it should catch all of them the first time
        test_process!(process(&mut input_tokens), true);

        // there shouldn't be any other matches
        test_process!(process(&mut input_tokens), false);
    }

}