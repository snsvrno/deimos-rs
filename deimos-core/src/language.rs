use crate::token::Token;
use crate::codewrap::{CodeWrap, CodeWrappable};

type T = CodeWrap<SyntaxElement>;

pub enum SyntaxElement {
    Token(Token),               // a simple convert

    Binop(Box<SyntaxElement>, Box<SyntaxElement>, Box<SyntaxElement>), // exp1, op, exp2

    // the final few
    //Chunk(SyntaxElement),       
    //Block(SyntaxElement),       // the final form!
}

impl CodeWrappable for SyntaxElement { }

impl std::fmt::Display for SyntaxElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxElement::Token(token) => write!(f, "{}", token),
            SyntaxElement::Binop(left, op, right) => write!(f, "({} {} {})",
                op, left, right),

            _ => write!(f, "SyntaxElement not defined!!")
        }
    }
}

impl SyntaxElement {
    pub fn reduce(elements : &mut Vec<CodeWrap<SyntaxElement>>) -> bool {
        //! will attempt to 'reduce' the list of tokens (or syntaxelement)
        //! to some kind of defined syntaxelement. It will consume edit
        //! the sent Vec<> and return a bool to whether it performed any
        //! changes or not.or
        
        // checks for binary ops
        for i in 0 .. elements.len() - 2 {
            if SyntaxElement::can_reduce_to_binop(&elements[i], &elements[i+1], &elements[i+2]) {
                let CodeWrap::CodeWrap(left, start, _) = elements.remove(i);
                let CodeWrap::CodeWrap(op, _, _) = elements.remove(i);
                let CodeWrap::CodeWrap(right, _, end) = elements.remove(i);

                // we make the new SyntaxElement element, and add it where 
                // we took it off
                elements.insert(i,CodeWrap::CodeWrap( 
                    SyntaxElement::Binop(
                        Box::new(left), 
                        Box::new(op), 
                        Box::new(right))
                    , start, end));

                // we leave saying that we made a change
                return true;    
            }
        }

        false
    }

    fn can_reduce_to_binop(left : &T, op : &T, right : &T) -> bool {
        //! checks if the three SyntaxElements can become a binary operation
        //! (binop)
        
        if let SyntaxElement::Token(ref token) = op.item() {
            match token {
                Token::Plus | Token::Minus | Token::Star | Token::Slash |
                Token::Carrot | Token::Percent | Token::DoublePeriod |
                Token::LessThan | Token::LessEqual | Token::GreaterThan |
                Token::GreaterEqual | Token::EqualEqual | Token::NotEqual |
                Token::And | Token::Or => {
                    // TODO : need to check if the other two are expressions
                    return true;
                },
                _ => return false,
            }
        }
        false
    }
}