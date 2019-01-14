use crate::elements::Token;
use crate::elements::TokenType;

#[derive(PartialEq,Debug)]
pub enum Statement {
    Token(Token),
    Unary(Token,Box<Statement>), // unop, expr
    Binary(Box<Statement>,Token,Box<Statement>), // expr, binop, expr
}

impl Statement {
        pub fn tokens_to_statements(tokens : Vec<Token>) -> Vec<Statement> {
        /// convinence function that converts a vec<token> to a vec<statement>
        /// by wrapping each token with a statement.
        
        let mut statements : Vec<Statement> = Vec::new();

        for t in tokens {
            statements.push(Statement::Token(t));
        }

        statements
    }

    pub fn as_token_type<'a>(&'a self) -> &'a TokenType {
        match self {
            Statement::Token(ref token) => {
                &token.get_type()
            },
            _ => {
                panic!("Cannot unwrap {:?} as a Token",self)
            }
        }
    }

    pub fn is_unop(&self) -> bool {
        match self {
            Statement::Token(token) => match token.get_type() {
                TokenType::Minus | 
                TokenType::Not |
                TokenType::Pound => true,
                _ => false,
            }
            _ => false,
        }
    }

    pub fn is_expr(&self) -> bool {
        match self {
            Statement::Token(token) => match token.get_type() {
                TokenType::Nil | 
                TokenType::False | 
                TokenType::True |
                TokenType::Number(_) | 
                TokenType::String(_) => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn into_unary(self,expr : Statement) -> Statement {
        if !self.is_unop() { panic!("Cannot make {:?} into unary, not an operator.",self); }
        if !expr.is_expr() { panic!("Cannot make unary, {:?} isn't an expression.",expr); }

        match self {
            Statement::Token(token) => Statement::Unary(token,Box::new(expr)),
            _ => panic!("Cannot make {:?} into unary, not an operator.",self),
        }
        
    }
}