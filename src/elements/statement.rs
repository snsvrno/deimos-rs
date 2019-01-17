use crate::elements::Token;
use crate::elements::TokenType;

#[derive(PartialEq,Debug)]
pub enum Statement {
    Token(Token),
    Unary(Token,Box<Statement>), // unop, expr
    Binary(Token,Box<Statement>,Box<Statement>), // binop, expr1, expr2
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

    pub fn is_binop(&self) -> bool {
        match self {
            Statement::Token(token) => match token.get_type() {
                TokenType::Plus |
                TokenType::Minus |
                TokenType::Star |
                TokenType::Slash |
                TokenType::Carrot |
                TokenType::Percent |
                TokenType::DoublePeriod |
                TokenType::GreaterThan |
                TokenType::LessThan |
                TokenType::GreaterEqual |
                TokenType::LessEqual |
                TokenType::EqualEqual |
                TokenType::NotEqual |
                TokenType::And |
                TokenType::Or => true,
                _ => false,
           },
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
            Statement::Binary(_,_,_) |
            Statement::Unary(_,_) => true,
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

    pub fn into_binary(self,expr1 : Statement, expr2 : Statement) -> Statement {
        if !self.is_binop()  { panic!("Cannot make {:?} into binary, not an operator.",self); }
        if !expr1.is_expr() { panic!("Cannot make binary, expr1 {:?} isn't an expression.", expr1); }
        if !expr2.is_expr() { panic!("Cannot make binary, expr2 {:?} isn't an expression.", expr2); }

        match self {
            Statement::Token(token) => { 
                // need to check if we have another binary that resolved too soon,
                // basically we are checking the order of operation here.
                
                if let Statement::Binary(ref op,_,_) = expr1 {
                    if TokenType::oop_binary(&token.get_type(),&op.get_type()) {
                        let (op,n_expr1,n_expr2) = expr1.explode_binary();
                        let inner = Statement::Binary(token,Box::new(n_expr2),Box::new(expr2));
                        let outer = Statement::Binary(op,Box::new(n_expr1),Box::new(inner));
                        return outer;
                    }
                }
                
                Statement::Binary(token, Box::new(expr1), Box::new(expr2))
            },
            _ => panic!("Cannot make {:?} into binary, not an operator.", self),
        }

    }

    pub fn explode_binary(self) -> (Token,Statement,Statement) {
        match self {
            Statement::Binary(op,ex1,ex2) => {
                return (op,*ex1,*ex2);
            },
            _ => panic!("Exploding {:?} as a binary isn't allowed, not a binary.",self),

        }
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Statement::Token(token) => write!(f,"{}",token),
            Statement::Unary(op,expr) => write!(f,"({} {})",op,expr),
            Statement::Binary(op,e1,e2) => write!(f,"({} {} {})",op,e1,e2),
        }
    }
}
