use crate::elements::Statement;

#[derive(PartialEq,Debug)]
pub struct Chunk {
    statements : Vec<Statement>,
}

impl Chunk {
    pub fn new(statements : Vec<Statement>) -> Chunk {
        Chunk { statements }
    }
}