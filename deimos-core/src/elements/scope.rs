use std::collections::HashMap;

use failure::{Error,format_err};

use crate::elements::Statement;
use crate::elements::statement::TableIndex;
use crate::stdlib;

pub struct Scope {
    vars : HashMap<String,Statement>,
    local : Vec<Box<Scope>>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            vars : HashMap::new(),
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
        let mut sliced = Scope::split_lookup_name(var_name);
        
        // gets the base value, and then looks inside if we are trying
        // too index it like an array or a table
        match self.get_value_internal(sliced.remove(0)) {
            None => None,
            Some(ref base) => {
                let mut working : &Statement = base;
                loop {
                   if sliced.len() <= 0 { break; } 
                   if let Statement::Table(ref table) = working {
                        let index = sliced.remove(0);
                        match table.get(&TableIndex::create(&index)) {
                            None => return None, // panic!("Object doesn't contain index {}",index),
                            Some(ref subtable) => working = subtable,
                        }
                   } else {
                        panic!("Object {} is not a table.",working);
                   }
                }
                Some(&working)
            }
        }
    }

    fn get_value_internal<'a>(&'a self, var_name : &str) -> Option<&'a Statement> {
        //! an private internal function, doesn't mean it gets the 
        //! internal value or something

        // first checks if there are local scopes, and checks if that stuff is in there
        for i in (0 .. self.local.len()).rev() {
            if let Some(ref value) = self.local[i].get_value(var_name) {
                return Some(&value);
            }
        }

        // then checks the global list
        self.vars.get(var_name)
    }

    fn split_lookup_name<'b>(name : &'b str) -> Vec<&'b str> {
        //! suppose to split a string into sections based
        //! on lua rules
        //!
        //! bob[1][2][3]
        //! bob[1].left[3]
        //! bob.x
        //!
        
        let mut splits : Vec<&str> = Vec::new();
        let mut c = 0;

        for i in 0 .. name.len() {
            let char = &name[i .. i+1];
            match char {
                "[" | 
                "]" |
                "." => {
                    splits.push(&name[c .. i]); c = i + 1; 
                },
                _ => (),
            } 
        }

        if c < name.len() {
            splits.push(&name[c .. ]);
        }

        splits
    }

    /*
    pub fn register_function(&mut self,name : &str, function : Statement) {
        self.funcs.insert(name.to_string(),function);
    }*/

    pub fn eval_stdlib_function(&mut self, name : &str, args : &Statement) -> Option<Result<Statement,Error>> {
        match name {
            "print" => Some(stdlib::core::print(self,args)), 
            "assert" => Some(stdlib::core::assert(self,args)),
            _ => None,
        }
    }

    pub fn eval_function(&mut self, name : &str, args : &Statement) -> Result<Statement,Error> {
       match self.get_function(name){
           Ok(func) => func.eval_as_function(self,args),
           Err(error) => {
                // going to try if global
                match self.eval_stdlib_function(name,args) {
                    Some(result) => result,
                    None => Err(error),
                }
           }
       }
    }

    pub fn get_function(&self, name : &str) -> Result<Statement,Error> {
        match self.vars.get(name) {
            None => Err(format_err!("Function {} isn't defined.",name)),
            Some(func) => match func.is_function() {
                true => Ok(func.clone()),
                false => Err(format_err!("Var {} isn't a function defined.",name))
            },
        }
    }
}
