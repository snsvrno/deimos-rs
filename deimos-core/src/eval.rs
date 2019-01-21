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
    fn simple() {
        let code = r"
        bob = 5 + 3
        ";

        let eval = setup_eval!(&code);
        println!("bob is {:?}",eval.get_value("bob"));
        assert_eq!(&8_f32, eval.get_value("bob").unwrap().as_number());
        
    }
}