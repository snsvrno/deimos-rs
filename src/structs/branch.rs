use enums::token::Token;
use enums::operator::Operator;

use failure::Error;

#[derive(Debug)]
pub struct Branch {
    child1 : Option<Box<Branch>>,
    child2 : Option<Box<Branch>>,
    token : Token,
}

impl Branch {
 
    pub fn new(token : Token) -> Branch {
        Branch {
            token : token,
            child1 : None,
            child2 : None,

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

    pub fn eval(&self) -> Result<i32,Error> {
        match self.token {
            Token::Operator(ref op) => {
                if self.len() != 2 {
                    return Err(format_err!("Cannot evaluate unless there are two children."))
                }
                let mut value = 0_i32;

                if let Some(ref c1) = self.child1 {
                    value = c1.eval()?;
                }

                if let Some(ref c2) = self.child2 {
                    match op {
                        Operator::Plus => {
                            value += c2.eval()?;
                        },
                        Operator::Minus => {
                            value -= c2.eval()?
                        }
                    }
                }

                return Ok(value);
            },
            
            Token::Int(ref int) => {
                return Ok(int.clone())
            }

            _ => (),
        }

        Err(format_err!("Unimplmented"))
    }

    pub fn is_none(&self) -> bool {
        match self.token {
            Token::None => true,
            _ => false,
        }
    }
}