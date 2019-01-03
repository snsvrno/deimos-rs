use failure::Error;

use structs::branch::Branch;
use structs::env::Env;

use enums::value::Value;
use enums::eresult::EResult;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Tree {
    code_start : usize,
    code_end : usize,
    tree : Vec<Branch>,
    variables : HashMap<String,Value>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            code_start : 0,
            code_end : 0,
            tree : Vec::new(),
            variables : HashMap::new(),
        }
    }

    pub fn set_range(&mut self, start : usize, end : usize) {
        self.code_start = start;
        self.code_end = end;
    }

    pub fn add_branch(&mut self, branch : Branch) {
        self.tree.push(branch);
    }

    pub fn get_raw_code<'a>(&self, entire_code : &'a str) -> Result<&'a str,Error> {
        if self.code_start == self.code_end || self.code_start > self.code_end {
            return Err(format_err!("Code Range for Tree doesn't seem initalized properly: {} to {}",self.code_start,self.code_end));
        }

        Ok(&entire_code[self.code_start .. self.code_end])
    }

    pub fn eval(&mut self, parent_env : &mut Env) -> Result<EResult,Error> {
        for ref branch in self.tree.iter() {

            // builds the environment
            let mut env = Env::borrow_from(parent_env);
            env.add(&mut self.variables);
            // parent_env.add(&mut self.variables);
             
            match branch.eval(&env)? {
                EResult::Assignment(var_name,value) => { 
                    env.insert(var_name,value);
                }, 
                _ => (),
            }
        }

        Ok(EResult::Value(Value::Bool(true)))
    }
    
}