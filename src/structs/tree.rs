use failure::Error;

use structs::branch::Branch;
use structs::env::Env;

use enums::value::Value;
use enums::eresult::EResult;

#[derive(Debug)]
pub struct Tree {
    code_start : usize,
    code_end : usize,
    tree : Vec<Branch>,
    env : Env,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            code_start : 0,
            code_end : 0,
            tree : Vec::new(),
            env : Env::new(),
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

    pub fn eval(&mut self, parent_env : &Env) -> Result<(EResult,Vec<EResult>),Error> {
        self.env = Env::from_upstream(parent_env);
        
        for ref branch in self.tree.iter() {            
            match branch.eval(&self.env)? {
                EResult::Assignment(var_name,value,is_local) => {
                    match is_local {
                        true => self.env.set_var_local(var_name,value),
                        false => self.env.set_var(var_name,value),
                    }
                }, 
                _ => (),
            }
        }

        Ok((
            EResult::Value(Value::Bool(true)),
            self.env.transfer_upstream_actions()
        ))
    }
    
}