use failure::Error;

use crate::parser::Parser;

use crate::elements::{Statement, Scope};

pub struct Eval<'a> {
    raw_code : &'a str,
    variables : Scope,
    result : Statement,
}

impl<'a> Eval<'a> {
    pub fn from_parser(parser : Parser<'a>) -> Result<Eval,Error> {
        let mut variables = Scope::new();
        let (code,chunk) = parser.disassemble();

        let result : Statement = match chunk.eval(&mut variables)? {
            Statement::Return(chunk_result) => *chunk_result,
            _ => Statement::Empty,
        };
        
        Ok(Eval{
            raw_code : code,
            variables,
            result
        })
    }

    pub fn get_value<'b>(&'b self, var_name : &str) -> Option<&'b Statement> {
        self.variables.get_value(var_name)
    }

    pub fn get_result(&self) -> &Statement {
        &self.result
    }
}
