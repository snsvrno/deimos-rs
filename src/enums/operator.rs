
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum Operator {
    Plus,
    Minus,
    Equals(bool), // bool means is_local
}

impl Operator {
    pub fn type_is(&self) -> &'static str {
        match self {
            Operator::Plus => "plus",
            Operator::Minus => "minus",
            Operator::Equals(is_local) => match is_local {
                true => "equals, local",
                false => "equals",
            },
        }
    }
}