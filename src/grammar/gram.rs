use crate::grammar::literal::Literal;
use crate::grammar::expression::Expression;
use crate::grammar::binary::Binary;
use crate::grammar::unary::Unary;
use crate::grammar::grouping::Grouping;

use crate::grammar::blockdo::BlockDo;
use crate::grammar::blockrepeat::BlockRepeat;
use crate::grammar::blockwhile::BlockWhile;
use crate::grammar::blockif::BlockIf;

use crate::token::Token;

use failure::{Error,format_err};

#[derive(PartialEq,Clone)]
pub enum Gram {
    Literal(Box<Literal>),
    Unary(Box<Unary>),
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Expression(Box<Expression>),
    BlockDo(Box<BlockDo>),
    BlockWhile(Box<BlockWhile>),
    BlockRepeat(Box<BlockRepeat>),
    BlockIf(Box<BlockIf>),
    Token(Token),
}

impl Gram {
    pub fn create(token : Token) -> Gram {
        match Literal::create_into_gram(token.clone()) {
            Some(lit) => lit,
            None => Gram::Token(token),
        }
    }

    pub fn unwrap_expr(self) -> Result<Expression,Error> {
        match self {
            Gram::Expression(expr) => Ok(*expr),
            _ => Err(format_err!("Can't unwrap '{:?}' as an Expression.",self)),
        }
    }

    pub fn unwrap_token(self) -> Result<Token,Error> {
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

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! token {
    ($tokentype:expr) => {
        $crate::grammar::gram::Gram::Token(
            $crate::token::Token::simple($tokentype)
        )
    };
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! gram {
    ($tokentype:expr) => {
        $crate::grammar::gram::Gram::create(
            $crate::token::Token::simple($tokentype)
        )
    };
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
            Gram::BlockDo(block) => write!(f,"{}",block),
            Gram::BlockWhile(block) => write!(f,"{}",block),
            Gram::BlockRepeat(block) => write!(f,"{}",block),
            Gram::BlockIf(block) => write!(f,"{}",block),
        }
    }
}

impl std::fmt::Debug for Gram {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Gram::Literal(lit) => write!(f, "GL<{:?}>",lit),
            Gram::Unary(unary) => write!(f, "GU<{:?}>",unary),
            Gram::Binary(binary) => write!(f, "GB<{:?}>",binary),
            Gram::Grouping(grouping) => write!(f, "GG<{:?}>",grouping),
            Gram::Expression(expr) => write!(f, "GE<{:?}>",expr),
            Gram::Token(token) => write!(f, "GT<{:?}>",token),
            Gram::BlockDo(block) => write!(f,"GBD<{:?}>",block),
            Gram::BlockWhile(block) => write!(f,"GBW<{:?}>",block),
            Gram::BlockRepeat(block) => write!(f,"GBR<{:?}>",block),
            Gram::BlockIf(block) => write!(f,"GBI<{:?}>",block),
        }
    }
}