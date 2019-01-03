use std::collections::HashMap;
use enums::eresult::EResult;

use enums::value::Value;

use functions;

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Env {
    functions : HashMap<String,fn() -> EResult>,
    variables : HashMap<String,Value>,
    upstream_variables : HashMap<String,Value>,
    upstream_actions : Vec<EResult>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            functions : HashMap::new(),
            variables : HashMap::new(),
            upstream_variables : HashMap::new(),
            upstream_actions : Vec::new(),
        }
    }

    pub fn from_upstream(other_env : &Env) -> Env {
        let mut env = Env::new();
        env.upstream_variables = other_env.variables.clone();
        env
    }

    pub fn set_var(&mut self, var_name : String, value : Value) {

        // checks if the variable is already assigned locally. if it doesn't
        // exist that means we are trying to access a variable in a higher scope
        // so we make an upstream_action to assign the variable once we are finished 
        // in this scope, but we have to modify the upstream_variables too in case
        // we use that variable again in this scope.
        match self.variables.get(&var_name) {
            Some(_) => { self.variables.insert(var_name,value); },
            None => {
                // modify the variable here
                self.upstream_variables.insert(var_name.clone(),value.clone());

                // set a command to change it upstream, though first need to check 
                // if we already have an assignment command, we can then remove it
                let mut added = false;
                for action in self.upstream_actions.iter_mut() {
                    if let EResult::Assignment(ref e_var, ref mut e_value, _) = action {
                        if e_var == &var_name {
                            *e_value = value.clone();
                            added = true;
                        }
                    }
                }
                // if it isn't modified then we add it here.
                if !added {
                    self.upstream_actions.push(EResult::Assignment(
                        var_name,value,false
                    ));
                }
            }
        }
    }

    pub fn load_lua_standard_functions(&mut self) {

    }

    pub fn set_var_local(&mut self, var_name : String, value : Value) {
        self.variables.insert(var_name,value);
    }

    pub fn get_value_of<'a>(&'a self, var_name : &str) -> Option<&'a Value> {

        match self.variables.get(var_name) {
            Some(ref value) => Some(&value),
            None => self.upstream_variables.get(var_name),
        }
    }

    pub fn run_function(&self, func_name : &str) -> Option<EResult> {
        match self.functions.get(func_name) {
            None => None,
            Some(func) => {
                Some(func())
            }
        }
    }

    pub fn transfer_upstream_actions(&mut self) -> Vec<EResult> {
        let mut commands = Vec::new();
        for action in self.upstream_actions.drain(..){
            commands.push(action);
        }
        commands
    }
}