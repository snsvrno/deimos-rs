
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum Operator {
    Plus,
    Minus,
    Equals,
}

impl Operator {
    pub fn type_is(&self) -> &'static str {
        match self {
            Operator::Plus => "plus",
            Operator::Minus => "minus",
            Operator::Equals => "equals",
        }
    }
}