use crate::chunk::Chunk;
use crate::tokentype::TokenType;

use crate::grammar::gram::Gram;

use failure::{Error,format_err};

#[derive(PartialEq,Clone,Debug)]
pub struct BlockRepeat {
    expression : Chunk,
    chunks : Vec<Chunk>
}

impl BlockRepeat {
    pub fn create_chunk(expression : Chunk, chunks : Vec<Chunk>) -> Chunk {
        let do_block = BlockRepeat { expression, chunks };
        let gram  = Gram::BlockRepeat(Box::new(do_block));
        Chunk::wrap(gram)
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
                        TokenType::Repeat => { start = Some(i); },
                        TokenType::Until => {
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
                    let mut toks_expr : Vec<Chunk> = raw_chunks.drain(end + 1 .. end + 2).collect();
                    let mut toks : Vec<Chunk> = raw_chunks.drain(start .. end + 1).collect();

                    if toks_expr.len() != 1 {
                        return Err(format_err!("Repeat - Until requires exactly 1 expression: "));
                    }

                    if toks.len() < 3 {
                        return Err(format_err!("Malformed Repeat - Until statement : "));
                    }
                    
                    // remove the first and last line
                    // this is ok because `do` and `end` are forced on their own lines, 
                    // and if they matched then they truely on at the first and last
                    // lines of this collected drain.
                    toks.remove(0);
                    toks.pop();

                    let repeat_block = BlockRepeat::create_chunk(toks_expr.remove(0),toks);
                    raw_chunks.insert(start,repeat_block);

                    i = 0;
                    range = None;
                },
                None => { i = i + 1; }
            }

        }

        Ok(())
    } 
}


impl std::fmt::Display for BlockRepeat {
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
        write!(f,"(repeat {} until {})",statement,self.expression)
    }
}
