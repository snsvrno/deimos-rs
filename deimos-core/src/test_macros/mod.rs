
#[macro_export]
macro_rules! identifier {
    ($str : expr) => {
        {   
            use crate::syntax::SyntaxElement;
            use crate::token::Token;
            use crate::codewrap::CodeWrap;

            CodeWrap::CodeWrap(SyntaxElement::Token(Token::Identifier(String::from($str))), 0, 0)
        }
    };
}


#[macro_export]
macro_rules! token {
    ($str : expr) => {
        {   
            use crate::syntax::SyntaxElement;
            use crate::token::Token;
            use crate::codewrap::CodeWrap;

            let token = match $str {
                "[" => Token::LeftBracket,
                "]" => Token::RightBracket,
                "." => Token::Period,
                _ => unimplemented!(),
            };

            CodeWrap::CodeWrap(SyntaxElement::Token(token), 0, 0)
        }
    };
}

#[macro_export]
macro_rules! number {
    ($num : expr) => {
        {   
            use crate::syntax::SyntaxElement;
            use crate::token::Token;
            use crate::codewrap::CodeWrap;

            CodeWrap::CodeWrap(SyntaxElement::Exp(Box::new(SyntaxElement::Token(Token::Number($num)))), 0, 0)
        }
    };
}

#[macro_export]
macro_rules! exp {
    ($item : expr) => {
        {   
            use crate::syntax::SyntaxElement;
            use crate::token::Token;
            use crate::codewrap::CodeWrap;

            if let CodeWrap::CodeWrap(element, _, _) = $item {
                CodeWrap::CodeWrap(SyntaxElement::Exp(Box::new(element)), 0, 0)
            } else {
                unimplemented!();
            }            
        }
    };
}

#[macro_export]
macro_rules! prefixexp {
    ($item : expr) => {
        {   
            use crate::syntax::SyntaxElement;
            use crate::token::Token;
            use crate::codewrap::CodeWrap;

            if let CodeWrap::CodeWrap(element, _, _) = $item {
                CodeWrap::CodeWrap(SyntaxElement::PrefixExp(Box::new(element)), 0, 0)
            } else {
                unimplemented!();
            }            
        }
    };
}

#[macro_export]
macro_rules! var {
    ($item : expr) => {
        {   
            use crate::syntax::SyntaxElement;
            use crate::token::Token;
            use crate::codewrap::CodeWrap;

            if let CodeWrap::CodeWrap(element, _, _) = $item {
                CodeWrap::CodeWrap(SyntaxElement::Var(Box::new(element)), 0, 0)
            } else {
                unimplemented!();
            }            
        }
    };
}

#[macro_export]
macro_rules! test_process {
    ($item : expr, $boolean : expr) => {
        {   

            use crate::syntax::SyntaxResult;

            let result = match $item {
                SyntaxResult::Done => true,
                _ => false,
            };

            if $boolean {
                assert!(result);  
            } else {
                assert!(!result);  
            }
        }
    };
}

