use std::collections::HashMap;

use enums::value::Value;

pub struct Env<'a> {
    pub variable_tree : Vec<&'a HashMap<String,Value>>,
}

impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env {
            variable_tree : Vec::new(),
        }
    }

    pub fn from(variables :&'a HashMap<String,Value>) -> Env<'a> {
        Env {
            variable_tree : vec![variables],
        }
    }

    pub fn make_from(other_env : &'a Env) -> Env<'a> {
        let mut vec : Vec<&HashMap<String,Value>> = Vec::new();
        for v in other_env.variable_tree.iter() {
            vec.push(v);
        }

        Env {
            variable_tree : vec,
        }
    }

    pub fn add(&mut self, variables : &'a HashMap<String,Value>) {
        self.variable_tree.push(variables);
    }

    pub fn value_of(&self, var_name : &str) -> Option<&'a Value> {
        let no_of_v_groups = self.variable_tree.len();

        for i in (0 .. no_of_v_groups).rev() {
            if let Some(ref value) = self.variable_tree[i].get(var_name) {
                return Some(value);
            }
        }

        None
    }
}