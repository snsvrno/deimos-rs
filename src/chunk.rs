/// per 2.4.1 - Chunks of the Lu 5.1 Reference Manual
/// 
/// > The unit of execution of Lua is called a _chunk_. A chunk is simply 
/// a sequence of statements, which are executed sequentially. Each statement
/// can be optionally followed by a semicolon:
/// 
/// ```lua
/// chunk ::= { stat [';']}
/// ```

use crate::grammar::gram::Gram;

#[derive(PartialEq,Debug,Clone)]
pub struct Chunk {
    elements : Vec<Gram>,
}

impl Chunk {
    pub fn new() -> Chunk { 
        Chunk {
            elements : Vec::new(),
        }
    }

    pub fn wrap(gram : Gram) -> Chunk {
        Chunk {
            elements : vec![gram],
        }
    }

    pub fn new_from(chunk : Vec<Gram>) -> Chunk {
        Chunk {
            elements : chunk,
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn at<'a>(&'a self, index : usize) -> &'a Gram {
        &self.elements[index]
    }

    pub fn add(&mut self, gram : Gram) {
        self.elements.push(gram);
    }

    pub fn remove(&mut self, start : usize, end : usize) -> Vec<Gram> {
        self.elements.drain(start .. end).collect()
    }

    pub fn insert(&mut self, index : usize, gram : Gram) {
        self.elements.insert(index,gram);
    }

    pub fn iter(&self) -> std::slice::Iter<Gram> {
        self.elements.iter()
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        let mut statement : String = String::new();
        for c in 0 .. self.elements.len() {
            if c == 0 {
                statement = format!("{}",self.elements[c]);
            } else {
                statement = format!("{}, {}",statement,self.elements[c]);
            }
        }
        write!(f,"{}",statement)
    }
}