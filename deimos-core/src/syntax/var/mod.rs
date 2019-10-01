type T = CodeWrap<SyntaxElement>;

mod name;
mod prefixname;
mod prefixexp;

use crate::syntax::{SyntaxResult, SyntaxElement};
use crate::codewrap::CodeWrap;

pub fn process(elements : &mut Vec<T>) -> SyntaxResult {
    //! works through the elements and checks if we are working with a var.
    //!
    //! [x] Name
    //! [x] prefixexp `[´ exp `]´
    //! [x] prefixexp `.´ Name
    
    let mut count = 0;
    loop {
        count += 1;

        // looks for prefixexp [exp]
        if prefixexp::process(elements) { continue; }

        // looks for prefixexp.Name
        if prefixname::process(elements) { continue; }

        // looks for Name
        if name::process(elements) { continue; }

        break;
    }

    if count > 1 { SyntaxResult::Done } 
    else { SyntaxResult::None }
}


#[cfg(test)]
mod tests {

    use crate::syntax::{ SyntaxResult, SyntaxElement };
    use crate::codewrap::CodeWrap::CodeWrap;
    use crate::token::Token;

    use crate::syntax::var::process;

    #[test]
    pub fn name() {
        let mut input_tokens : Vec<crate::codewrap::CodeWrap<SyntaxElement>> = vec![
            CodeWrap(SyntaxElement::Token(Token::Identifier(String::from("bob"))), 0, 0),
            CodeWrap(SyntaxElement::Token(Token::Identifier(String::from("bob"))), 0, 0),
            CodeWrap(SyntaxElement::Token(Token::Identifier(String::from("bob"))), 0, 0),
        ];

        // it should catch all of them the first time
        let result = match process(&mut input_tokens) {
            SyntaxResult::Done => true,
            _ => false,
        };
        assert!(result);

        // there shouldn't be any other matches
        let result = match process(&mut input_tokens) {
            SyntaxResult::Done => true,
            _ => false,
        };
        assert!(!result);
    }

    #[test]
    pub fn prefixexp_exp() {
        let mut input_tokens : Vec<crate::codewrap::CodeWrap<SyntaxElement>> = vec![
            CodeWrap(SyntaxElement::PrefixExp(Box::new(SyntaxElement::Var(Box::new(SyntaxElement::Token(Token::Identifier(String::from("bob"))))))), 0, 0),
            CodeWrap(SyntaxElement::Token(Token::LeftBracket), 0, 0),
            CodeWrap(SyntaxElement::Exp(Box::new(SyntaxElement::Token(Token::Number(1.0)))), 0, 0),
            CodeWrap(SyntaxElement::Token(Token::RightBracket), 0, 0),
        ];

        // it should catch all of them the first time
        let result = match process(&mut input_tokens) {
            SyntaxResult::Done => true,
            _ => false,
        };
        assert!(result);

        // it should reduce it down to 1 element
        assert_eq!(input_tokens.len(),1);


        // there shouldn't be any other matches
        let result = match process(&mut input_tokens) {
            SyntaxResult::Done => true,
            _ => false,
        };
        assert!(!result);
    }

    #[test]
    pub fn prefixexp_dot_name() {
        let mut input_tokens : Vec<crate::codewrap::CodeWrap<SyntaxElement>> = vec![
            CodeWrap(SyntaxElement::PrefixExp(Box::new(SyntaxElement::Var(Box::new(SyntaxElement::Token(Token::Identifier(String::from("bob"))))))), 0, 0),
            CodeWrap(SyntaxElement::Token(Token::Period), 0, 0),
            CodeWrap(SyntaxElement::Token(Token::Identifier(String::from("bob"))), 0, 0),
        ];

        // it should catch all of them the first time
        let result = match process(&mut input_tokens) {
            SyntaxResult::Done => true,
            _ => false,
        };
        assert!(result);

        // it should reduce it down to 1 element
        assert_eq!(input_tokens.len(),1);

        
        // there shouldn't be any other matches
        let result = match process(&mut input_tokens) {
            SyntaxResult::Done => true,
            _ => false,
        };
        assert!(!result);
    }

}