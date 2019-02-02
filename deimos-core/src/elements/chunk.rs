use failure::Error;

use crate::elements::Statement;
use crate::elements::Scope;

#[derive(PartialEq,Debug)]
pub struct Chunk {
    statements : Vec<Statement>,
}

impl Chunk {
    pub fn empty() -> Chunk {
        Chunk { statements : Vec::new() }
    }
    
    pub fn new(statements : Vec<Statement>) -> Chunk {
        Chunk { statements }
    }

    pub fn add(&mut self, statement : Statement) {
        self.statements.push(statement);
    }

    pub fn is_empty(&self) -> bool {
        self.statements.len() == 0
    }

    pub fn eval(&self, mut scope : &mut Scope) -> Result<Statement,Error> {
        let mut result = Statement::Empty;
        
        for stat in self.statements.iter() {
            result = stat.eval(&mut scope)?;
        }
        
        Ok(result)
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
