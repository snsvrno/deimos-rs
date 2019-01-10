use crate::chunk::Chunk;
use crate::tokentype::TokenType;

use crate::grammar::gram::Gram;

use failure::{Error,format_err};

struct IfBuilder {
    expression : Option<Chunk>,
    chunks : Option<Vec<Chunk>>,
    else_chunks : Option<Vec<Chunk>>,
}

impl IfBuilder {
    pub fn new() -> IfBuilder {
        IfBuilder {
            expression : None,
            chunks : None,
            else_chunks : None,
        }
    }

    pub fn set_expression(&mut self, chunk : Chunk) {
        self.expression = Some(chunk);
    }

    pub fn set_chunks(&mut self, chunks : Vec<Chunk>) {
        self.chunks = Some(chunks);
    }

    pub fn set_else_chunks(&mut self, chunks : Vec<Chunk>) {
        self.else_chunks = Some(chunks);
    }

    pub fn build(mut self) -> BlockIf {

        BlockIf {
            expression : self.expression.unwrap(),
            chunks : self.chunks.unwrap(),
            else_chunks : self.else_chunks,
            else_if : None,
        }
    }
}

#[derive(PartialEq,Clone,Debug)]
pub struct BlockIf {
    expression : Chunk,
    chunks : Vec<Chunk>,
    else_chunks : Option<Vec<Chunk>>,
    else_if : Option<Box<BlockIf>>,
}

impl BlockIf {
    pub fn create_chunk(block : BlockIf) -> Chunk {
        let gram  = Gram::BlockIf(Box::new(block));
        Chunk::wrap(gram)
    }

    pub fn process(raw_chunks : &mut Vec<Chunk>) -> Result<(),Error> {
        
        let mut matches : Vec<(TokenType,usize)> = Vec::new();
        let mut found_end : bool = false;

        let mut i = 0;
        
        loop {
            if i == raw_chunks.len() { break; }

            for j in 0 .. raw_chunks[i].len() {
                if let Gram::Token(ref token) = raw_chunks[i].at(j) {
                    match token.get_type() {
                        TokenType::If |
                        TokenType::Else |
                        TokenType::Elseif |
                        TokenType::Then => { matches.push((token.get_type().clone(),i)); },
                        TokenType::End => {
                            matches.push((token.get_type().clone(),i));
                            found_end = true;
                            break;
                        },
                        _ => (),
                    }
                }
            }

            if found_end {
                for m in matches.iter() { println!("{}",m.0); }

                if !(matches[0].0 == TokenType::If)
                || !(matches[1].0 == TokenType::Then)
                || !(matches[matches.len()-1].0 == TokenType::End) {
                    return Err(format_err!("If-Then-End isn't correct: "));
                }

                let start = matches[0].1;

                let mut builder = IfBuilder::new();
                let mut toks : Vec<Chunk> = raw_chunks.drain(matches[0].1 .. matches[matches.len()-1].1 + 1).collect();

                for i in (0 .. matches.len() - 1).rev() {
                    match matches[i].0 {
                        TokenType::If => { 
                            let mut chunks : Vec<Chunk> = toks.drain(matches[i].1 - start + 1 .. matches[i+1].1 - start).collect();
                            builder.set_expression(chunks.remove(0));
                        },
                        TokenType::Then => {
                            let chunks : Vec<Chunk> = toks.drain(matches[i].1 - start + 1 .. matches[i+1].1 - start).collect();
                            builder.set_chunks(chunks);
                        },
                        TokenType::Else => {
                            let chunks : Vec<Chunk> = toks.drain(matches[i].1 - start + 1 .. matches[i+1].1 - start).collect();
                            builder.set_else_chunks(chunks);
                        },
                        _ => (),
                    }
                }

                let if_block = BlockIf::create_chunk(builder.build());
                raw_chunks.insert(start,if_block);


                i = 0;
                found_end = false;
                matches = Vec::new();

            } else {
                i = i + 1;
            }
            

        }

        Ok(())
    } 
}


impl std::fmt::Display for BlockIf {
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

        match self.else_chunks {
            Some(ref chunks) => {
                let mut else_statement : String = String::new();
                if chunks.len() == 1 {
                    else_statement = format!("{}",chunks[0]);
                } else {
                    for c in 0 .. chunks.len() {
                        if c == 0 {
                            else_statement = format!("\n  {}\n",chunks[c]);
                        } else {
                            else_statement = format!("  {}  {}\n",else_statement,chunks[c]);
                        }
                    }
                }

                write!(f,"(if {} then {}  else {} end)",self.expression,statement,else_statement)
            },
            None => write!(f,"(if {} then {} end)",self.expression,statement)
        } 
    }
}
