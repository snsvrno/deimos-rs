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

impl std::fmt::Display for Chunk {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        for statement in self.statements.iter() {
            write!(f,"{}\n",statement)?;
        }

        Ok(())
    }
}
