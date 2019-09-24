use crate::token::Token;
use crate::codewrap::{CodeWrap, CodeWrappable};

use failure::Error;
use crate::syntaxerror::SyntaxError;

type T = CodeWrap<SyntaxElement>;

#[derive(Debug)]
pub enum SyntaxElement {
    Token(Token),               // a simple convert
    Block(Box<SyntaxElement>),
    Chunk(Vec<Box<SyntaxElement>>),
    
    Var(Box<SyntaxElement>),
    VarList(Vec<Box<SyntaxElement>>),
    NameList(Vec<Box<SyntaxElement>>),
    Exp(Box<SyntaxElement>),
    ExpList(Vec<Box<SyntaxElement>>),
    Binop(Box<SyntaxElement>, Box<SyntaxElement>, Box<SyntaxElement>), // exp1, op, exp2
    Unop(Box<SyntaxElement>, Box<SyntaxElement>),   // op, exp

    StatementAssign(Box<SyntaxElement>, Box<SyntaxElement>),    // the left, the right
    LastStatReturn(Option<Box<SyntaxElement>>),
    LastStat

    // the final few
    //Chunk(SyntaxElement),       
    //Block(SyntaxElement),       // the final form!
}

impl CodeWrappable for SyntaxElement { }

impl std::fmt::Display for SyntaxElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxElement::Token(token) => write!(f, "{}", token),
            SyntaxElement::Binop(left, op, right) => write!(f, "({} {} {})", op, left, right),
            SyntaxElement::Unop(op, exp) => write!(f, "({} {})", op, exp),
            SyntaxElement::Var(item) => write!(f, "{}", item),
            SyntaxElement::VarList(list) => write!(f, "<Var {}>", SyntaxElement::list_to_string(list,", ")),
            SyntaxElement::NameList(list) => write!(f, "<Name {}>", SyntaxElement::list_to_string(list,", ")),
            SyntaxElement::Exp(item) => write!(f, "{}", item),
            SyntaxElement::ExpList(list) => write!(f, "<Exp {}>", SyntaxElement::list_to_string(list,", ")),
            SyntaxElement::StatementAssign(left,right) => write!(f, "(= {} {})", left, right),


            SyntaxElement::Block(chunk) => write!(f, "{}", chunk),
            SyntaxElement::Chunk(statements) => write!(f, "{}", SyntaxElement::list_to_string(statements,"\n")),

            _ => write!(f, "SyntaxElement not defined!!")
        }
    }
}

impl SyntaxElement {

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // CHUNKS /// //////////////////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn process_statements_to_chunk(elements : &mut Vec<T>) -> Result<bool,Error> {
        //! takes the list of elements (assuming its statements, but will check) and 
        //! attempts to reduce them into a chunk
        //!
        //! chunk ::= {stat [`;´]} [laststat [`;´]]

        if elements.len() > 1 { for i in 0 .. elements.len()-1 {
            if !elements[i].item().is_statement() {
                return Err(SyntaxError::general(elements[i].start(), elements[i].end(),
                    "must be a statement"));
            }
        }}

        if elements[elements.len()-1].item().is_statement() || elements[elements.len()-1].item().is_last_statement() {
            // everything checks out, here is where we actually pull out these statements
            // and makes our chunk
            let mut new_list : Vec<Box<SyntaxElement>> = Vec::new();
            let mut start : Option<usize> = None;
            let mut end : usize = 0;

            for item in elements.drain(..) {
                let CodeWrap::CodeWrap(item, s, e) = item;
                
                // gets the overall code reference
                if start.is_none() { start = Some(s); }
                else { end = e; }

                new_list.push(Box::new(item));
            }

            if let Some(start) = start {
                elements.insert(0, CodeWrap::CodeWrap(SyntaxElement::Chunk(new_list), start, end) );
                return Ok(true)
            }
            

        } else {
            let element = &elements[elements.len()-1];
            return Err(SyntaxError::general(element.start(), element.end(),
                "last line must be a statement"));
        }

        Ok(false)
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // STATEMENTS //////////////////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn process_statement(elements : &mut Vec<T>) -> Result<bool,Error> {
        //! works through thelements and checks if any of the following 
        //! are matched, returns true iof it reduces something to a statement
        //! 
        //! [ ] varlist `=´ explist | 
        //! [ ] functioncall | 
        //! [ ] do block end | 
        //! [ ] while exp do block end | 
        //! [ ] repeat block until exp | 
        //! [ ] if exp then block {elseif exp then block} [else block] end | 
        //! [ ] for Name `=´ exp `,´ exp [`,´ exp] do block end | 
        //! [ ] for namelist in explist do block end | 
        //! [ ] function funcname funcbody | 
        //! [ ] local function Name funcbody | 
        //! [ ] local namelist [`=´ explist] 

        // varlist `=´ explist
        if elements.len() > 2 { for i in 0 .. elements.len() - 2 {
            if SyntaxElement::can_reduce_to_statement_assign(&elements[i], &elements[i+1], &elements[i+2]) {
                    let CodeWrap::CodeWrap(v_list,start,_) = elements.remove(i);
                    elements.remove(i); // getting rid of the equals sign
                    let CodeWrap::CodeWrap(e_list,_,end) = elements.remove(i);

                    let new_element = SyntaxElement::StatementAssign(Box::new(v_list), Box::new(e_list));
                    elements.insert(i, CodeWrap::CodeWrap(new_element, start, end));
            }
        }}

        // functioncall | 
        // do block end | 
        // while exp do block end | 
        // repeat block until exp | 
        // if exp then block {elseif exp then block} [else block] end | 
        // for Name `=´ exp `,´ exp [`,´ exp] do block end | 
        // for namelist in explist do block end | 
        // function funcname funcbody | 
        // local function Name funcbody | 
        // local namelist [`=´ explist] 
        
        Ok(false)
    }

    fn can_reduce_to_statement_assign(var_list : &T, op : &T, exp_list : &T) -> bool {
        //! checks if the three given items match what they need to be in order
        //! to be a assignment statement

        match op {
            CodeWrap::CodeWrap(SyntaxElement::Token(Token::Equal),_,_) => {
                // if the middle item is an equal sign, then its worth checking the rest of the tokens.
                if var_list.item().is_var_list() && exp_list.item().is_exp_list() { return true; }
            },
            _ => { }
        }

        false
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // VARLIST /////////////////////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn process_var_list(elements : &mut Vec<T>) -> Result<bool,Error> {
        //! works through the elements and checks if we can make a var list.
        //!
        //! [x] var {`,´ var}

        let mut i : usize = 0;
        let mut start : Option<usize> = None;
        loop {
            // check and make sure we don't access outside the array.
            if i >= elements.len() { break; }

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
                             else { return Err(SyntaxError::general(elements[i].start(), elements[i].end(),
                                "var list, start isn't defined")); };

                match factor % 2 {
                    // these are all the odd ones, starting with the one right after the exp
                    1 => if let CodeWrap::CodeWrap(SyntaxElement::Token(Token::Comma),_,_) = elements[i] { } else { break; }

                    // these are the even ones, should all be expressions
                    0 => if let CodeWrap::CodeWrap(SyntaxElement::Var(_),_,_) = elements[i] { } 
                         else { 
                            return Err(SyntaxError::general(elements[i].start(), elements[i].end(),
                                "var an expression after the comma")); 
                         },

                    _=> return Err(SyntaxError::general(elements[i].start(), elements[i].end(),
                        "var list, mod is not 1 or 0...")),
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

                return Ok(true);         
            }
        }

        Ok(false)
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // VAR /////////////////////////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn process_var(elements : &mut Vec<T>) -> Result<bool,Error> {
        //! works through the elements and checks if we are working with a var.
        //!
        //! [x] Name
        //! [ ] prefixexp `[´ exp `]´
        //! [ ] prefixexp `.´ Name

        // looks for Name
        // Name is another name for Identifier
        for i in 0 .. elements.len() {
            let found : bool = if let CodeWrap::CodeWrap(SyntaxElement::Token(Token::Identifier(_)),_,_) = elements[i] { true }
                               else { false };

            if found {
                let CodeWrap::CodeWrap(token, start, end) = elements.remove(i);
                elements.insert(i, CodeWrap::CodeWrap(SyntaxElement::Var(Box::new(token)), start, end));
                return Ok(true); // we leave saying that we made a change 
            }
        }

        // looks for prefixexp [exp]
        // looks for prefixexp.Name

        Ok(false)
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // EXPRESSIONS LISTS ///////////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn process_exp_list(elements : &mut Vec<T>) -> Result<bool,Error> {
        //! works through the elements and checks if we can make an expression
        //! list, `{ }` below means 0+ occurance `[ ]` means optional (no repeatition)
        //! an expression == an expression list, an expression list != an expression
        //! 
        //! [ ] {exp `,´} exp
        
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
                if let CodeWrap::CodeWrap(SyntaxElement::Exp(_),_,_) = elements[i] {
                    start = Some(i);
                }
            } else {
                // we are going to alternate tokens between ',' and 'exp'
                // so we can use %2 to tell if we should see a ',' or an 'exp'
                let factor = if let Some(start) = start { i - start } 
                             else { return Err(SyntaxError::general(elements[i].start(), elements[i].end(),
                                "expression list, start isn't defined")); };

                match factor % 2 {
                    // these are all the odd ones, starting with the one right after the exp
                    1 => if let CodeWrap::CodeWrap(SyntaxElement::Token(Token::Comma),_,_) = elements[i] { } else { break; }

                    // these are the even ones, should all be expressions
                    0 => if let CodeWrap::CodeWrap(SyntaxElement::Exp(_),_,_) = elements[i] { } 
                         else { 
                            return Err(SyntaxError::general(elements[i].start(), elements[i].end(),
                                "expecting an expression after the comma")); 
                         },

                    _=> return Err(SyntaxError::general(elements[i].start(), elements[i].end(),
                        "expression list, mod is not 1 or 0...")),
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
                elements.insert(start,CodeWrap::CodeWrap(SyntaxElement::ExpList(new_list),
                    code_start, code_end));

                return Ok(true);         
            }
        }

        Ok(false)
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // EXPRESSIONS /////////////////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn process_exp(elements : &mut Vec<T>) -> Result<bool,Error> {
        //! works through the elements and checks if any of the following 
        //! are matched, returns true if it reduces something
        //!
        //! [x] nil | 
        //! [x] false | 
        //! [x] true | 
        //! [x] Number | 
        //! [x] String | 
        //! [x] `...´ | 
        //! [ ] function | 
        //! [ ] prefixexp | 
        //! [ ] tableconstructor | 
        //! [x] exp binop exp | 
        //! [x] unop exp 
        
        // checks for nil
        if SyntaxElement::process_exp_single_token(elements,Token::Nil) { return Ok(true); }
        // checks for false
        if SyntaxElement::process_exp_single_token(elements,Token::False) { return Ok(true); }
        // checks for true
        if SyntaxElement::process_exp_single_token(elements,Token::True) { return Ok(true); }
        // checks for Number
        if SyntaxElement::process_exp_single_token(elements,Token::Number(0.0)) { return Ok(true); }
        // checks for String
        // TODO : Combine the Token::String and Token::MultiLineString objects together since programmatically
        //        (in lua) it doesn't matter which you have when parsing. do i care if i want to reassemble the
        //        code? 
        if SyntaxElement::process_exp_single_token(elements,Token::Nil) { return Ok(true); }
        // checks for `...`
        if SyntaxElement::process_exp_single_token(elements,Token::TriplePeriod) { return Ok(true); }
        // checks for function
        // checks for prefixexp
        // checks for tableconstructor
        if SyntaxElement::process_exp_binop(elements) { return Ok(true); }
        if SyntaxElement::process_exp_unop(elements) { return Ok(true); }

        Ok(false)
    }

    fn process_exp_single_token(elements : &mut Vec<T>, check_token: Token) -> bool {
        //! a shared function that looks for the following token and processes it into
        //! an expression, used for the following cases
        //!         
        //! nil | false | true | Number | String | `...´
        
        for i in 0 .. elements.len() {
            let found : bool = if let CodeWrap::CodeWrap(SyntaxElement::Token(ref token),_,_) = elements[i] { 
                                   check_token.is_same_type(&token) 
                               } else { false };

            if found {
                let CodeWrap::CodeWrap(token, start, end) = elements.remove(i);
                elements.insert(i, CodeWrap::CodeWrap(SyntaxElement::Exp(Box::new(token)), start, end));
                return true; // we leave saying that we made a change 
            }
        }

        // didn't find anyhting, so we return false
        false
    }

    fn process_exp_binop(elements : &mut Vec<T>) -> bool {
        //! attemps to combine elements into a binary op

        // checks for binary ops
        if elements.len() > 2 { for i in 0 .. elements.len() - 2 {

            if SyntaxElement::can_reduce_to_exp_binop(&elements[i], &elements[i+1], &elements[i+2]) {
                let CodeWrap::CodeWrap(left, start, _) = elements.remove(i);
                let CodeWrap::CodeWrap(op, _, _) = elements.remove(i);
                let CodeWrap::CodeWrap(right, _, end) = elements.remove(i);

                // we make the new SyntaxElement element, and add it where 
                // we took it off
                let new_op = SyntaxElement::Binop(Box::new(left),Box::new(op),Box::new(right));
                // since a binop is an expression, and the rest of the matches are expecting this
                // we will wrap it now.
                let new_exp = SyntaxElement::Exp(Box::new(new_op));
                elements.insert(i,CodeWrap::CodeWrap(new_exp, start, end));
                
                return true; // we leave saying that we made a change 
            }
        }}

        // didn't find anyhting, so we return false
        false
    }


    fn process_exp_unop(elements : &mut Vec<CodeWrap<SyntaxElement>>) -> bool {
        //! checks for unary ops
        
        if elements.len() > 1 { for i in 0 .. elements.len() - 1 {

            if SyntaxElement::can_reduce_to_exp_unop(&elements[i], &elements[i+1]) {
                let CodeWrap::CodeWrap(op, start, _) = elements.remove(i);
                let CodeWrap::CodeWrap(right, _, end) = elements.remove(i);

                // we make the new SyntaxElement element, and add it where 
                // we took it off
                let new_op = SyntaxElement::Unop(Box::new(op),Box::new(right));
                // since a unop is an expression, and the rest of the matches are expecting this
                // we will wrap it now.
                let new_exp = SyntaxElement::Exp(Box::new(new_op));
                elements.insert(i,CodeWrap::CodeWrap(new_exp, start, end));

                return true;  // we leave saying that we made a change   
            }
        }}

        // didn't find anyhting, so we return false
        false
    }

    fn can_reduce_to_exp_binop(left : &T, op : &T, right : &T) -> bool {
        //! checks if the three SyntaxElements can become a binary operation
        //! (binop)
        
        if let SyntaxElement::Token(ref token) = op.item() {
            match token {
                Token::Plus | Token::Minus | Token::Star | Token::Slash |
                Token::Carrot | Token::Percent | Token::DoublePeriod |
                Token::LessThan | Token::LessEqual | Token::GreaterThan |
                Token::GreaterEqual | Token::EqualEqual | Token::NotEqual |
                Token::And | Token::Or => match (left.item().is_exp(), right.item().is_exp()) {
                    // checking if the two other parts are expressions
                    (true, true) => return true,
                    _ => return false,
                }
                _ => return false,
            }
        }
        false
    }

    fn can_reduce_to_exp_unop(op : &T, right : &T) -> bool {
        //! checks if the two SyntaxElements can become a unary operation
        //! (unop)
        
        if let SyntaxElement::Token(ref token) = op.item() {
            match token {
                Token::Minus | Token::Not | Token::Pound => match right.item().is_exp() {
                    // checking if the other part is an expression
                    true => return true,
                    _ => return false,
                }
                _ => return false,
            }
        }
        false
    }

    ////////////////////////////////
    ////////////////////////////////
    ////////////////////////////////

    fn is_exp(&self) -> bool {
        match self {
            SyntaxElement::Exp(_) => true,
            _ => false,
        }
    }

    fn is_exp_list(&self) -> bool {
        match self {
            SyntaxElement::ExpList(_) => true,
            _ => false,
        }
    }

    fn is_var_list(&self) -> bool {
        match self {
            SyntaxElement::VarList(_) => true,
            _ => false,
        }
    }

    fn is_statement(&self) -> bool {
        match self {
            SyntaxElement::StatementAssign(_,_) => true,
            _ => false,
        }
    }

    fn is_last_statement(&self) -> bool {
        match self {
            SyntaxElement::LastStatReturn(_) | 
            SyntaxElement::LastStat => true,
            _ => false,
        }
    }

    fn list_to_string(list : &Vec<Box<SyntaxElement>>, divider : &str) -> String {
        let mut string : String = String::new();

        for item in list {
            string = format!("{}{}{}",string, item, divider);
        }

        return string;
    }
}