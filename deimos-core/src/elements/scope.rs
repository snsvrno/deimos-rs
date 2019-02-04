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

    fn assign_table_value(&mut self, var_address : &Vec<&str>, value : Statement) -> Result<(),Error> {
        let mut scope : Option<usize> = None;
        let hash_table : HashMap<TableIndex,Statement> = {
            let mut table : Option<HashMap<TableIndex,Statement>> = None;
            // looks through the scopes for the variable
            for i in (0 .. self.local.len()).rev() {
                if table.is_none() {
                    if let Some(ref table_ref) = self.local[i].get_value(var_address[0])? {
                        match table_ref {
                            Statement::Table(hashmap) => { 
                                table = Some(hashmap.clone());
                                scope = Some(i);
                            },
                            _ => return Err(format_err!("{} is not a table : {}.",var_address[0],table_ref)),
                        }
                    }
                }
            }

            match table {
                Some(table) => { table },
                None => {
                    match self.vars.get(var_address[0]) {
                        None => HashMap::new(),
                        Some(ref table_ref) => {
                            match table_ref {
                                Statement::Table(hashmap) => hashmap.clone(),
                                _ => return Err(format_err!("{} is not a table : {}.",var_address[0],table_ref)),
                            }
                        },
                    }
                },
            }
        };

        let mut list_o_hash : Vec<HashMap<TableIndex,Statement>> = Vec::new();
        list_o_hash.push(hash_table);
        for j in 1 .. var_address.len() - 1 {
            let hash_depth = list_o_hash.len() - 1;

            match list_o_hash[hash_depth].get(&TableIndex::create(var_address[j])) {
                None => return Err(format_err!("{} is not defined.",var_address[j])),
                Some(ref subtable) => {
                    match subtable {
                        Statement::Table(ref subhash) => {
                            list_o_hash.push(subhash.clone());
                        },
                        _ => return Err(format_err!("{} is not a table.",var_address[j]))
                    }
                }
            }
        }

        // do the final assignment
        let final_hash = list_o_hash.len()-1;
        list_o_hash[final_hash].insert(TableIndex::create(var_address[var_address.len()-1]),value);

        // rebuild the hash
        loop {
            if list_o_hash.len() == 1 { break; }
            let i = list_o_hash.len()-1;
            let subtable = list_o_hash.remove(i);
            list_o_hash[i-1].insert(
                TableIndex::create(var_address[i]),
                Statement::Table(subtable)
            );
        }
        
        match scope {
            Some(i) => { self.local[i].vars.insert(var_address[0].to_string(),Statement::Table(list_o_hash.remove(0))); },
            None => { self.vars.insert(var_address[0].to_string(),Statement::Table(list_o_hash.remove(0))); },
        }
        
        Ok(())
        
    }

    pub fn assign(&mut self, var_name : &str, value : Statement) -> Result<(),Error> {
        let sliced = Scope::split_lookup_name(var_name);

        if sliced.len() > 1 {
            // we are trying to index a table
            
            self.assign_table_value(&sliced,value)
        } else {
            // this is a normal variable

            // needs to check if this variable exists in any of the local
            // scopes, because they should be assigned there if they are being
            // called / referencesd (they are being shadowed)
            for i in (0 .. self.local.len()).rev() {
                if self.local[i].get_value(var_name)?.is_some() {
                    self.local[i].assign(var_name,value);
                    return Ok(());
                }
            }

            self.vars.insert(var_name.to_string(),value);
            Ok(())
        }

    }

    pub fn push_local(&mut self) {
        self.local.push(Box::new(Scope::new()));
    }

    pub fn pop_local(&mut self) -> usize {
        self.local.pop();
        self.local.len()
    }

    pub fn get_value<'a>(&'a self, var_name : &str) -> Result<Option<&'a Statement>,Error> {
        let mut sliced = Scope::split_lookup_name(var_name);
        
        // gets the base value, and then looks inside if we are trying
        // too index it like an array or a table
        match self.get_value_internal(sliced.remove(0))? {
            None => Ok(None),
            Some(ref base) => {
                let mut working : &Statement = base;
                loop {
                   if sliced.len() <= 0 { break; } 
                   if let Statement::Table(ref table) = working {
                        let index = sliced.remove(0);
                        match table.get(&TableIndex::create(&index)) {
                            None => return Ok(None), // panic!("Object doesn't contain index {}",index),
                            Some(ref subtable) => working = subtable,
                        }
                   } else {
                        return Err(format_err!("Object {} is not a table.",working));
                   }
                }
                Ok(Some(&working))
            }
        }
    }

    fn get_value_internal<'a>(&'a self, var_name : &str) -> Result<Option<&'a Statement>,Error> {
        //! an private internal function, doesn't mean it gets the 
        //! internal value or something

        // first checks if there are local scopes, and checks if that stuff is in there
        for i in (0 .. self.local.len()).rev() {
            if let Some(ref value) = self.local[i].get_value(var_name)? {
                return Ok(Some(&value));
            }
        }

        // then checks the global list
        Ok(self.vars.get(var_name))
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
