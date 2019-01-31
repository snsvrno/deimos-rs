use std::collections::HashMap;

use failure::{Error,format_err};

use crate::elements::Statement;
use crate::elements::TokenType;

use crate::stdlib;

pub struct Scope {
    vars : HashMap<String,Statement>,
    funcs : HashMap<String,Statement>,
    local : Vec<Box<Scope>>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            vars : HashMap::new(),
            funcs : HashMap::new(),
            local : Vec::new(),
        }
    }

    pub fn assign_local(&mut self, var_name : &str, value : Statement) -> Result<(),Error> {
        if self.local.len() == 0 {
            self.push_local();
        }
        
        let pos = self.local.len()-1;
        self.local[pos].vars.insert(var_name.to_string(),value);
        Ok(())
    }

    pub fn assign(&mut self, var_name : &str, value : Statement) -> Result<(),Error> {
        
        // needs to check if this variable exists in any of the local
        // scopes, because they should be assigned there if they are being
        // called / referencesd (they are being shadowed)
        for i in (0 .. self.local.len()).rev() {
            if self.local[i].get_value(var_name).is_some() {
                self.local[i].assign(var_name,value);
                return Ok(());
            }
        }

        self.vars.insert(var_name.to_string(),value);
        Ok(())
    }

    pub fn push_local(&mut self) {
        self.local.push(Box::new(Scope::new()));
    }

    pub fn pop_local(&mut self) -> usize {
        self.local.pop();
        self.local.len()
    }

    pub fn get_value<'a>(&'a self, var_name : &str) -> Option<&'a Statement> {
        
        // first checks if there are local scopes, and checks if that stuff is in there
        for i in (0 .. self.local.len()).rev() {
            if let Some(ref value) = self.local[i].get_value(var_name) {
                return Some(&value);
            }
        }

        // then checks the global list
        self.vars.get(var_name)
    } 

    pub fn register_function(&mut self,name : &str, function : Statement) {
        self.funcs.insert(name.to_string(),function);
    }

    pub fn eval_stdlib_function(&mut self, name : &str, args : &Statement) -> Option<Result<Statement,Error>> {
        match name {
            "print" => Some(stdlib::core::print(self,args)), 
            "assert" => Some(stdlib::core::assert(self,args)),
            _ => None,
        }
    }

    pub fn get_function<'a>(&'a self, name : &str) -> Result<&'a Statement,Error> {
        match self.funcs.get(name) {
            None => Err(format_err!("Function {} isn't defined.",name)),
            Some(ref func) => Ok(func),
        }
    }
}
