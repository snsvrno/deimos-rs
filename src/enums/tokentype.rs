#[derive(Eq,PartialEq,Debug)]
pub enum TokenType {
    Int,
    Operator,
    WhiteSpace,
    String,
    Word,
    Other,
}