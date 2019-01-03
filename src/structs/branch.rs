use enums::token::Token;
use enums::operator::Operator;
use enums::value::Value;
use enums::eresult::EResult;

use structs::env::Env;

use failure::Error;

#[derive(Debug)]
pub struct Branch {
    child1 : Option<Box<Branch>>,
    child2 : Option<Box<Branch>>,
    token : Token,
    variable : Option<String>,
}

impl Branch {
 
    pub fn new(token : Token) -> Branch {
        Branch {
            token : token,
            child1 : None,
            child2 : None,
            variable : None,

        }
    }

    pub fn add_child(&mut self, child : Branch) {
        if self.child1.is_none() {
            self.child1 = Some(Box::new(child));
        } else if self.child2.is_none() {
            self.child2 = Some(Box::new(child));
        }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        if self.child1.is_some() { len+=1; }
        if self.child2.is_some() { len+=1; }
        len
    }

    pub fn get_parent<'a>(&'a self, part : &Branch) -> Option<&'a Branch> {
        if let Some(ref child) = self.child1 {
            if let Some(ref parent) = child.get_parent(part) {
                return Some(parent);
            }
        } else if let Some(ref child) = self.child2 {
            if let Some(ref parent) = child.get_parent(part) {
                return Some(parent);
            }
        }

        None
    }

    pub fn eval(&self, env : &Env) -> Result<EResult,Error> {
        match self.token {

            Token::Operator(Operator::Equals(is_local)) => {
                match self.child1 {
                    None => return Err(format_err!("Left of '=' operator cannot be empty")),
                    Some(ref child) => {
                        if let Some (ref c2) = self.child2 {
                            match child.token {
                                Token::Word(ref word) => {
                                    return Ok(EResult::Assignment(
                                        word.clone(),
                                        c2.eval(env)?.unwrap_value()?,
                                        is_local
                                    ));
                                },
                                _ => return Err(format_err!("Left of '=' must be a 'word'"))
                            }
                        }
                    }
                }
            },

            Token::Operator(ref op) => {
                if self.len() != 2 {
                    return Err(format_err!("Cannot evaluate unless there are two children."))
                }
                match (&self.child1, &self.child2) {
                    (Some(ref c1), Some(ref c2)) => {
                        let c1e = c1.eval(env)?.unwrap_value()?;
                        let c2e = c2.eval(env)?.unwrap_value()?;

                        match op {
                            Operator::Plus => { return Ok(EResult::Value(Value::add(&c1e,&c2e)?)); },
                            Operator::Minus => { return Ok(EResult::Value(Value::subtract(&c1e,&c2e)?)); },
                            _ => (),
                        }
                    },
                    (_,_) => (),
                }
            },

            Token::Word(ref word) => {
                match env.get_value_of(&word) {
                    Some(value) => return Ok(EResult::Value(value.clone())),
                    None => return Ok(EResult::Value(Value::Nil)),
                }
            },
            
            Token::Int(ref int) => {
                return Ok(EResult::Value(Value::Int(int.clone())));
            }

            _ => (),
        }

        Err(format_err!("Branch Eval didn't find a match for: {:?}",self.token))
    }

    pub fn is_none(&self) -> bool {
        match self.token {
            Token::None => true,
            _ => false,
        }
    }

    pub fn pretty(&self,prefix : Option<String>) {
        let pre = if let Some(prefix) = prefix { prefix } else { "".to_string() };

        println!("{}Token {:?}",pre,self.token);

        if let Some(ref child) = self.child1 {
            child.pretty(Some(format!("{}{}",pre,"--")));
        } 
        if let Some(ref child) = self.child2 {
            child.pretty(Some(format!("{}{}",pre,"--")));
        } 
    }
}