use crate::syntax::{SyntaxResult, SyntaxElement};
use crate::codewrap::CodeWrap;
use crate::token::Token;

pub fn process(elements : &mut Vec<CodeWrap<SyntaxElement>>) -> SyntaxResult {
    //! looks for an namelist
    //! 
    //! [x] Name {`,´ Name}

    // the intent here is to start looking for an expression, when we find one, we will
    // see how many are chained by `,` we can find and make that into a list.
    let mut i : usize = 0;
    let mut start : Option<usize> = None;

    // because of the order here, we are going to try and convert a varlist
    // to a namelist, we will check if all of the components are names
    // and then convert it.
    for i in 0 .. elements.len() {
        // checks for a varlist
        if let SyntaxElement::VarList(ref var_list) = elements[i].item() {
            // checks the items to see if they are all names

            for var in var_list.iter() {
                // if we don't find the inside is a name then 
                // we should continue checking the elements, because
                // this thing can't ever be a namelist
                if !var.ref_to_inside().is_name() { continue; }
            }

            // we are here so we must have a name list, lets explode this and 
            // remake it as a namelist.

            let CodeWrap::CodeWrap(old_list, start, end) = elements.remove(i);

            match old_list {
                SyntaxElement::VarList(mut list) => {
                    // technically we should convert the list instead of 
                    // just using it, because these are names and not vars.
                    let mut new_list : Vec<Box<SyntaxElement>> = Vec::new();

                    // converting it
                    for item in list.drain(..) {
                        match item.convert_to_name() {
                            Ok(name) => new_list.push(Box::new(name)),
                            Err(not_a_name) => {
                                return SyntaxResult::Error(start, end, 
                                    format!("attempted to convert {} to a name, but failed.", not_a_name));
                            }
                        }
                    }

                    // we survived the for loop,
                    let namelist = SyntaxElement::NameList(new_list);
                    let new_element = CodeWrap::CodeWrap(namelist, start, end);
                    elements.insert(i, new_element);
                    return SyntaxResult::Done;

                },
                // error message because technically we should never be able to get
                // to this line because if we are in this area we should have a varlist
                _ => return SyntaxResult::Error(start, end, "we shouldn't be here, this should have been a varlist".to_string())
            }
        }
    }


    SyntaxResult::None
}