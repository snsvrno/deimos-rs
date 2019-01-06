use grammar::literal::Literal;
use grammar::expression::Expression;
use grammar::binary::Binary;
use grammar::unary::Unary;
use grammar::grouping::Grouping;
use token::Token;

use failure::Error;

#[derive(PartialEq,Clone,Debug)]
pub enum Gram {
    Literal(Box<Literal>),
    Unary(Box<Unary>),
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Expression(Box<Expression>),
    Token(Token),
}

impl Gram {
    pub fn create(token : Token) -> Gram {
        match Literal::create_from(token.clone()) {
            Some(lit) => match Expression::create_into_gram(lit) {
                Some(expr) => expr,
                None => panic!("can't happen"),
            },
            None => Gram::Token(token),
        }
    }

    pub fn to_literal(mut self) -> Result<Gram,Error> {

        if let Gram::Token(token) = self {
            if let Some(lit) = Literal::create_from(token) {
                return Ok(lit);
            }
        }

        Err(format_err!("Can't convert to an Literal unless its a valid Literal"))
    }

    pub fn to_expr(mut self) -> Result<Gram,Error> {
        match Expression::create_from(self) {
            None => Err(format_err!("Can't convert to an Expression unless its a valid Expression")),
            Some(expr) => Ok(Gram::Expression(Box::new(expr)))
        }
    }


    pub fn unwrap_expr(mut self) -> Result<Expression,Error> {
        match self {
            Gram::Expression(expr) => Ok(*expr),
            _ => Err(format_err!("Can't unwrap '{:?}' as an Expression.",self)),
        }
    }

    pub fn unwrap_token(mut self) -> Result<Token,Error> {
        match self {
            Gram::Token(token) => Ok(token),
            _ => Err(format_err!("Can't unwrap '{:?}' as a Token.",self)),
        }
    }




    pub fn is_literal(&self) -> bool {
        match self {
            Gram::Literal(_) => true,
            _ => false,
        }
    }
    pub fn is_unary(&self) -> bool {
        match self {
            Gram::Unary(_) => true,
            _ => false,
        }
    }
    pub fn is_binary(&self) -> bool {
        match self {
            Gram::Binary(_) => true,
            _ => false,
        }
    }
    pub fn is_grouping(&self) -> bool {
        match self {
            Gram::Grouping(_) => true,
            _ => false,
        }
    }
    pub fn is_expression(&self) -> bool {
        match self {
            Gram::Expression(_) => true,
            _ => false,
        }
    }
    pub fn is_token(&self) -> bool {
        match self {
            Gram::Token(_) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Gram {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Gram::Literal(lit) => write!(f, "{}",lit),
            Gram::Unary(unary) => write!(f, "{}",unary),
            Gram::Binary(binary) => write!(f, "{}",binary),
            Gram::Grouping(grouping) => write!(f, "{}",grouping),
            Gram::Expression(expr) => write!(f, "{}",expr),
            Gram::Token(token) => write!(f, "{}",token),
        }
    }
}