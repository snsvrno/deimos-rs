use crate::chunk::Chunk;
use crate::tokentype::TokenType;

use crate::grammar::gram::Gram;

use failure::{Error,format_err};

struct IfBuilder {
    expression : Option<Chunk>,
    chunks : Option<Vec<Chunk>>,
    else_chunks : Option<Vec<Chunk>>,
    elseif : Vec<BlockIf>,
}

impl IfBuilder {
    pub fn new() -> IfBuilder {
        IfBuilder {
            expression : None,
            chunks : None,
            else_chunks : None,
            elseif : Vec::new(),
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

    pub fn add_elseif(&mut self, block : BlockIf) {
        self.elseif.push(block);
    }

    pub fn insert_elseif(&mut self, pos : usize, block : BlockIf) {
        self.elseif.insert(pos,block);
    }

    pub fn build(mut self) -> BlockIf {

        BlockIf {
            expression : self.expression.unwrap(),
            chunks : self.chunks.unwrap(),
            else_chunks : self.else_chunks,
            else_if : if self.elseif.len() > 0 { Some(self.elseif) } else { None },
        }
    }
}

#[derive(PartialEq,Clone,Debug)]
pub struct BlockIf {
    expression : Chunk,
    chunks : Vec<Chunk>,
    else_chunks : Option<Vec<Chunk>>,
    else_if : Option<Vec<BlockIf>>,
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
                let mut then_stuff : Option<Vec<Chunk>> = None; 

                for i in (0 .. matches.len() - 1).rev() {
                    match matches[i].0 {
                        TokenType::If => { 
                            let mut chunks : Vec<Chunk> = toks.drain(matches[i].1 - start + 1 .. matches[i+1].1 - start).collect();
                            builder.set_expression(chunks.remove(0));
                            
                            if let Some(then_chunks) = then_stuff {
                                builder.set_chunks(then_chunks);
                                then_stuff = None;
                            }
                        },
                        TokenType::Then => {
                            let chunks : Vec<Chunk> = toks.drain(matches[i].1 - start + 1 .. matches[i+1].1 - start).collect();
                            then_stuff = Some(chunks);
                        },
                        TokenType::Else => {
                            let chunks : Vec<Chunk> = toks.drain(matches[i].1 - start + 1 .. matches[i+1].1 - start).collect();
                            builder.set_else_chunks(chunks);
                        },
                        TokenType::Elseif => {
                            let mut chunks : Vec<Chunk> = toks.drain(matches[i].1 - start + 1 .. matches[i+1].1 - start).collect();
                            
                            match then_stuff {
                                None => return Err(format_err!("Cannot have an elseif without a then: ")),
                                Some(then_chunk) => {
                                    let elseif_block = BlockIf {
                                        expression : chunks.remove(0),
                                        chunks : then_chunk,
                                        else_chunks : None,
                                        else_if : None,
                                    };

                                    then_stuff = None;

                                    builder.insert_elseif(0, elseif_block);
                                }
                            }
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

    fn format_statement(chunks : &Vec<Chunk>) -> String {
        let mut statement : String = String::new();
        if chunks.len() == 1 {
            statement = format!("{}",chunks[0]);
        } else {
            for c in 0 .. chunks.len() {
                if c == 0 {
                    statement = format!("\n  {}\n",chunks[c]);
                } else {
                    statement = format!("  {}  {}\n",statement,chunks[c]);
                }
            }
        }
        statement
    }

    fn format_if(&self) -> String {
        format!("{}",self.expression)
    }

    fn format_then(&self) -> String {
        BlockIf::format_statement(&self.chunks)
    }

    fn format_else(&self) -> String {
        match self.else_chunks {
            Some(ref chunks) => format!(" else {}",BlockIf::format_statement(&chunks)),
            None => format!(""),
        } 
    }
}


impl std::fmt::Display for BlockIf {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        match self.else_if {
            None => write!(f,"(if {} then{}{} end)",
                self.format_if(),
                self.format_then(),
                self.format_else()),
                
            Some(ref elseif) => {
                let mut elseif_formatted = String::new();

                for i in 0 .. elseif.len() {
                    elseif_formatted = format!("{} elseif {} then {}",
                        elseif_formatted,
                        elseif[i].format_if(),
                        elseif[i].format_then()
                    )
                }
                
                write!(f,"(if {} then{}{}{} end)",
                    self.format_if(),
                    self.format_then(),
                    elseif_formatted,
                    self.format_else())
                
            }
        }
        
    }
}
