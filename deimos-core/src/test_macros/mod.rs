
#[macro_export]
macro_rules! identifier {
    ($str : expr) => {
        {   
            crate::codewrap::CodeWrap::CodeWrap(
                crate::syntax::SyntaxElement::Token(
                    crate::token::Token::Identifier(
                        String::from($str)
                    )
                ), 
            0, 0)
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
                "," => Token::Comma,
                "+" => Token::Plus,
                "-" => Token::Minus,
                _ => unimplemented!(),
            };

            CodeWrap::CodeWrap(SyntaxElement::Token(token), 0, 0)
        }
    };
}

#[macro_export]
macro_rules! remove_codewrap {
    ($exp : expr) => {
        {
            use crate::codewrap::CodeWrap;

            let CodeWrap::CodeWrap(item, _, _) = $exp;
            item
        }
    };
}

#[macro_export]
macro_rules! unop {
    ($op : expr, $exp : expr) => {
        {   
            use crate::syntax::SyntaxElement;
            use crate::token::Token;
            use crate::codewrap::CodeWrap;

            CodeWrap::CodeWrap(SyntaxElement::Unop(
                Box::new(SyntaxElement::Exp(Box::new(remove_codewrap!(token!($op))))),
                Box::new(SyntaxElement::Exp(Box::new(remove_codewrap!(number!($exp))))),
            ), 0, 0)
        }
    };
}

#[macro_export]
macro_rules! binop {
    ($left : expr, $op : expr, $right : expr) => {
        {   
            use crate::syntax::SyntaxElement;
            use crate::token::Token;
            use crate::codewrap::CodeWrap;

            CodeWrap::CodeWrap(SyntaxElement::Binop(
                Box::new(SyntaxElement::Exp(Box::new(remove_codewrap!(number!($left))))),
                Box::new(SyntaxElement::Exp(Box::new(remove_codewrap!(token!($op))))),
                Box::new(SyntaxElement::Exp(Box::new(remove_codewrap!(number!($right))))),
            ), 0, 0)
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

