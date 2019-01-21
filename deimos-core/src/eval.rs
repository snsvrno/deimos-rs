use failure::Error;

use crate::parser::Parser;

use crate::elements::{Statement, Scope};

pub struct Eval<'a> {
    raw_code : &'a str,
    variables : Scope,
}

impl<'a> Eval<'a> {
    pub fn from_parser(parser : Parser<'a>) -> Result<Eval,Error> {
        let mut variables = Scope::new();
        let (code,chunks) = parser.disassemble();

        for chunk in chunks.iter() {
            let result = chunk.eval(&mut variables)?;
        }

        Ok(Eval{
            raw_code : code,
            variables
        })
    }

    pub fn get_value<'b>(&'b self, var_name : &str) -> Option<&'b Statement> {
        self.variables.get_value(var_name)
    }
}

#[cfg(test)]
mod tests {

    use crate::test_crate::*;

    #[test]
    fn assignments() {
        let code = include_str!("../../lua/basic/assignments.lua");
        let eval = setup_eval!(&code);
        
        assert_eq!(&1_f32, eval.get_value("a").unwrap().as_number());
        assert_eq!(&2_f32, eval.get_value("b").unwrap().as_number());
        assert_eq!(&10_f32, eval.get_value("c").unwrap().as_number());
        assert_eq!(&3_f32, eval.get_value("d").unwrap().as_number());
        
    }
}