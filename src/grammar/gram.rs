pub use grammar::literal::Literal;
pub use grammar::expression::Expression;
pub use grammar::binary::Binary;
pub use grammar::unary::Unary;
pub use grammar::grouping::Grouping;
pub use token::Token;

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
    pub fn to_literal(mut self) -> Gram {
        if let Gram::Token(token) = self {
            if let Some(lit) = Literal::create_from(token) {
                return lit;
            }
        }

        panic!("Can't convert a Gram to an Literal unless its a valid Literal");
    }

    pub fn to_expr(mut self) -> Gram {
        match Expression::create_from(self) {
            None => panic!("Can't convert a Gram to an Expression unless its a valid Expression"),
            Some(expr) => Gram::Expression(Box::new(expr))
        }
    }
}