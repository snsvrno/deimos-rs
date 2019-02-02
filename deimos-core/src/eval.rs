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
        let (code,chunks) = parser.disassemble();

        let mut result = Statement::Empty;

        for chunk in chunks.iter() {
            if let Statement::Return(chunk_result) = chunk.eval(&mut variables)?{
                result = *chunk_result;   
            }
        }

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
