use crate::chunk::Chunk;
use crate::tokentype::TokenType;

use crate::grammar::gram::Gram;

use failure::{Error,format_err};

#[derive(PartialEq,Clone)]
pub struct BlockDo {
    chunks : Vec<Chunk>
}

impl BlockDo {
    pub fn create_chunk(chunks : Vec<Chunk>) -> Chunk {
        let do_block = BlockDo { chunks };
        let gram  = Gram::BlockDo(Box::new(do_block));
        Chunk::wrap(gram)
    }

    pub fn create_into_gram(chunks : Vec<Chunk>) -> Gram {
        let do_block = BlockDo { chunks };
        Gram::BlockDo(Box::new(do_block))
    }

    pub fn process(raw_chunks : &mut Vec<Chunk>) -> Result<(),Error> {
        
        let mut start : Option<usize> = None;
        let mut range : Option<(usize,usize)> = None;

        let mut i = 0;
        
        loop {
            if i == raw_chunks.len() { break; }

            for j in 0 .. raw_chunks[i].len() {
                if let Gram::Token(ref token) = raw_chunks[i].at(j) {
                    match token.get_type() {
                        TokenType::Do => { start = Some(i); },
                        TokenType::End => {
                            if let Some(si) = start {
                                range = Some((si,i));
                                start = None;
                                break;
                            }
                        },
                        _ => (),
                    }
                }
            }

            match range {
                Some((start,end)) => {
                    let mut toks : Vec<Chunk> = raw_chunks.drain(start .. end + 1).collect();
                    
                    if toks.len() < 3 {
                        return Err(format_err!("Malformed Do - End statement : "));
                    }
                    
                    // remove the first and last line
                    // this is ok because `do` and `end` are forced on their own lines, 
                    // and if they matched then they truely on at the first and last
                    // lines of this collected drain.
                    toks.remove(0);
                    toks.pop();

                    let do_block = BlockDo::create_chunk(toks);
                    raw_chunks.insert(start,do_block);

                    i = 0;
                    range = None;
                },
                None => { i = i + 1; }
            }

        }

        Ok(())
    } 
}


impl std::fmt::Display for BlockDo {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        let mut statement : String = String::new();

        if self.chunks.len() == 1 {
            statement = format!("{}",self.chunks[0]);
        } else {
            for c in 0 .. self.chunks.len() {
                if c == 0 {
                    statement = format!("\n  {}\n",self.chunks[c]);
                } else {
                    statement = format!("  {}  {}\n",statement,self.chunks[c]);
                }
            }
        }
        write!(f,"(do {} end)",statement)
    }
}

impl std::fmt::Debug for BlockDo {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut elements = String::new();
        for i in 0 .. self.chunks.len() {
            if i == 0 {
                elements = format!("{:?}",self.chunks[i]);
            } else {
                elements = format!("{},\n{:?}",elements,self.chunks[i]);
            }
        }
        write!(f,"BD<{}>",elements)
    }
}