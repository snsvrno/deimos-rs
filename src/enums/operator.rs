
#[derive(Debug,Clone)]
pub enum Operator {
    Plus,
    Minus,
}

impl Operator {
    pub fn type_is(&self) -> &'static str {
        match self {
            Operator::Plus => "plus",
            Operator::Minus => "minus",
        }
    }
}