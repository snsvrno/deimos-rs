use std::collections::HashMap;

use enums::value::Value;

pub struct Env<'a> {
    pub variable_tree : Vec<&'a mut HashMap<String,Value>>,
}

impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env {
            variable_tree : Vec::new(),
        }
    }

    pub fn borrow_from(other_env :&'a mut Env) -> Env<'a> {
        let mut env = Env::new();

        for var in other_env.variable_tree.iter() {
            env.add(*var);
        }

        env
    }

    pub fn add(&mut self, variables : &'a mut HashMap<String,Value>) {
        self.variable_tree.push(variables);
    }

    pub fn value_of(&'a self, var_name : &str) -> Option<&'a Value> {
        let no_of_v_groups = self.variable_tree.len();

        for i in (0 .. no_of_v_groups).rev() {
            if let Some(ref value) = self.variable_tree[i].get(var_name) {
                return Some(value);
            }
        }

        None
    }

    pub fn insert(&mut self, var_name : String, value : Value) {
        let no_of_v_groups = self.variable_tree.len();

        let mut added = false;

        for i in (0 .. no_of_v_groups).rev() {
            if let Some(_) = self.variable_tree[i].get(&var_name) {
                if !added {
                    self.variable_tree[i].insert(var_name.clone(),value.clone());
                    added = true;
                }
            }
        }

        if !added {
            self.variable_tree[no_of_v_groups].insert(var_name.clone(),value.clone());
        }
    }
}