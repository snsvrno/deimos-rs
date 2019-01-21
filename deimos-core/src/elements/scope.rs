use std::collections::HashMap;

use failure::Error;

use crate::elements::Statement;

pub struct Scope {
    vars : HashMap<String,Statement>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            vars : HashMap::new(),
        }
    }

    pub fn assign(&mut self, var_name : &str, value : Statement) -> Result<(),Error> {
        self.vars.insert(var_name.to_string(),value);
        Ok(())
    }

    pub fn get_value<'a>(&'a self, var_name : &str) -> Option<&'a Statement> {
        self.vars.get(var_name)
    } 
}