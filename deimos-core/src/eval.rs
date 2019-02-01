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
        assert_eq!("a string here", eval.get_value("e").unwrap().as_string());
    }

    #[test]
    fn unaryops() {
        let code = include_str!("../../lua/basic/unaryops.lua");
        let eval = setup_eval!(&code);

        assert_eq!(&-5_f32, eval.get_value("a").unwrap().as_number());
        assert_eq!(&-100.23,eval.get_value("b").unwrap().as_number());
        assert_eq!(&5_f32, eval.get_value("c").unwrap().as_number());
        assert_eq!(false, eval.get_value("d").unwrap().as_bool());
        assert_eq!(true, eval.get_value("e").unwrap().as_bool());
    }

    #[test]
    fn binaryops() {
        let code = include_str!("../../lua/basic/binops.lua");
        let eval = setup_eval!(&code);
        
        assert_eq!(&101_f32, eval.get_value("a").unwrap().as_number());
        assert_eq!(&2_f32, eval.get_value("b1").unwrap().as_number());
        assert_eq!(&-2_f32, eval.get_value("b2").unwrap().as_number());
        assert_eq!(&40_f32, eval.get_value("c").unwrap().as_number());
        assert_eq!(&3_f32, eval.get_value("d1").unwrap().as_number());
        assert_float!(&1.333_f32, eval.get_value("d2").unwrap().as_number());
        assert_eq!(&0_f32, eval.get_value("e1").unwrap().as_number());
        assert_eq!(&2_f32, eval.get_value("e2").unwrap().as_number());
        assert_eq!(&8_f32, eval.get_value("f").unwrap().as_number());
        assert_eq!("34", eval.get_value("g1").unwrap().as_string());
        assert_eq!("asmb", eval.get_value("g2").unwrap().as_string());
        assert_eq!(&15_f32, eval.get_value("h").unwrap().as_number());
    }

    #[test]
    fn functions() {
        let code = include_str!("../../lua/basic/functions.lua");
        let eval = setup_eval!(&code);
        
        assert_eq!(&4_f32, eval.get_value("var1").unwrap().as_number());
        assert_eq!(&9_f32, eval.get_value("var2").unwrap().as_number());
    }

    #[test]
    fn tables() {
        let code = include_str!("../../lua/basic/tables.lua");
        let eval = setup_eval!(&code);

        assert!(&10_f32 != eval.get_value("bob[2]").unwrap().as_number());
        
        assert_eq!(&10_f32, eval.get_value("bob[1]").unwrap().as_number());
        assert_eq!(&20_f32, eval.get_value("bob[2]").unwrap().as_number());
    }
}
