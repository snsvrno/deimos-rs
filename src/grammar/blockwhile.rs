use crate::chunk::Chunk;
use crate::tokentype::TokenType;

use crate::grammar::gram::Gram;

use failure::{Error,format_err};

#[derive(PartialEq,Clone,Debug)]
pub struct BlockWhile {
    expression : Chunk,
    chunks : Vec<Chunk>
}

impl BlockWhile {
    pub fn create_chunk(expression : Chunk, chunks : Vec<Chunk>) -> Chunk {
        let do_block = BlockWhile { expression, chunks };
        let gram  = Gram::BlockWhile(Box::new(do_block));
        Chunk::wrap(gram)
    }

    pub fn process(raw_chunks : &mut Vec<Chunk>) -> Result<(),Error> {
        
        let mut start : Option<usize> = None;
        let mut do_i : Option<usize> = None;
        let mut range : Option<(usize,usize,usize)> = None;

        let mut i = 0;
        
        loop {
            if i == raw_chunks.len() { break; }

            for j in 0 .. raw_chunks[i].len() {
                if let Gram::Token(ref token) = raw_chunks[i].at(j) {
                    match token.get_type() {
                        TokenType::While => { start = Some(i); }
                        TokenType::Do => { do_i = Some(i); },
                        TokenType::End => {
                            if let Some(si) = start {
                                if let Some(di) = do_i {
                                    range = Some((si,di,i));
                                    start = None;
                                    break;
                                }
                            }
                        },
                        _ => (),
                    }
                }
            }

            match range {
                Some((start,do_pos,end)) => {
                    let mut toks_block : Vec<Chunk> = raw_chunks.drain(do_pos + 1 .. end + 1).collect();
                    let mut toks_expr : Vec<Chunk> = raw_chunks.drain(start .. do_pos + 1).collect();
                    
                    if toks_block.len() < 2 {
                        return Err(format_err!("Malformed While - Do - End statement : "));
                    }

                    if toks_expr.len() < 3 {
                        return Err(format_err!("Can only have 1 expression in a While - Do - End : "));
                    }

                    // remove the first and last line
                    // this is ok because `while`,`do` and 'end' are forced on their own lines, 
                    // and if they matched then they truely on at the first and last
                    // lines of this collected drain.
                    toks_expr.remove(0);
                    toks_expr.pop();
                    toks_block.pop();

                    let while_block = BlockWhile::create_chunk(toks_expr.remove(0),toks_block);
                    raw_chunks.insert(start,while_block);

                    i = 0;
                    range = None;
                },
                None => { i = i + 1; }
            }

        }

        Ok(())
    } 
}


impl std::fmt::Display for BlockWhile {
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
        write!(f,"(while {} do {} end)",self.expression,statement)
    }
}
