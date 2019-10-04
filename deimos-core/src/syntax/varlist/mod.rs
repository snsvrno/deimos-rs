type T = CodeWrap<SyntaxElement>;

use crate::syntax::{SyntaxResult, SyntaxElement};
use crate::codewrap::CodeWrap;
use crate::token::Token;

pub fn process(elements : &mut Vec<T>) -> SyntaxResult {
    //! works through the elements and checks if we can make a var list.
    //!
    //! [x] var {`,´ var}

    let mut i : usize = 0;
    let mut start : Option<usize> = None;
    loop {
        // check and make sure we don't access outside the array.
        if i >= elements.len() { break; }
        // also check if we are at an equals sign, because we can only do var_lists on the left side
        // of an equals sign.
        if let CodeWrap::CodeWrap(SyntaxElement::Token(Token::Equal), _, _) = elements[i] { break; }

        if start.is_none() {
            // if we haven't found the first expression, then lets 
            // check for an expression and mark it as the first one.
            if let CodeWrap::CodeWrap(SyntaxElement::Var(_),_,_) = elements[i] {
                start = Some(i);
            }
        } else {
            // we are going to alternate tokens between ',' and 'exp'
            // so we can use %2 to tell if we should see a ',' or an 'exp'
            let factor = if let Some(start) = start { i - start } 
                         else { return SyntaxResult::Error(elements[i].start(), elements[i].end(),
                            "var list, start isn't defined(varlist)".to_string()); };

            match factor % 2 {
                // these are all the odd ones, starting with the one right after the exp
                1 => if let CodeWrap::CodeWrap(SyntaxElement::Token(Token::Comma),_,_) = elements[i] { } else { break; }

                // these are the even ones, should all be expressions
                0 => if let CodeWrap::CodeWrap(SyntaxElement::Var(_),_,_) = elements[i] { } 
                     else { 
                        return SyntaxResult::None; 
                     },

                _=> return SyntaxResult::Error(elements[i].start(), elements[i].end(),
                    "mod is not 1 or 0...(varlist)".to_string()),
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
            elements.insert(start,CodeWrap::CodeWrap(SyntaxElement::VarList(new_list),
                code_start, code_end));

            return SyntaxResult::Done;         
        }
    }

    SyntaxResult::None
}

#[cfg(test)]
mod tests {

    use crate::syntax::varlist::process;
    use crate::syntax::SyntaxElement;
    use crate::codewrap::CodeWrap;

    // contains all the test macros, to make the construction of tests look
    // simpler, and easier to understand the nesting.
    use crate::{
        identifier, token, prefixexp, exp, var, number,
        test_process,
    };

    #[test]
    pub fn varlist() {
        let mut input_tokens : Vec<crate::codewrap::CodeWrap<SyntaxElement>> = vec![
            var!(identifier!("bob")), token!(","),
            var!(identifier!("bob")), token!(","), 
            var!(identifier!("bob")),
        ];

        // it should catch all of them the first time
        test_process!(process(&mut input_tokens), true);

        // there shouldn't be any other matches
        test_process!(process(&mut input_tokens), false);
    }

    #[test]
    pub fn varlist_failed() {
        let mut input_tokens : Vec<crate::codewrap::CodeWrap<SyntaxElement>> = vec![
            var!(identifier!("bob")),
            var!(identifier!("bob")),
            var!(identifier!("bob")),
        ];

        // it should fail, so it still have 3 tokens, 
        // but the process DOES work, because it changes
        // var to varlist but they are varlist of len() = 1
        test_process!(process(&mut input_tokens), true);
        assert_eq!(input_tokens.len(),3);
    }

}