use crate::error::CodeInformation;
use crate::token::Token;
use crate::codewrap::CodeWrap;
use crate::scanner::{Scanner,TokenWrapped};
use crate::syntax::{
    exp, explist, prefixexp, statement, var, varlist, 
    tableconstructor, field, fieldlist, namelist,
    final_compress,
    SyntaxElement, SyntaxResult
};
use crate::parsererror::ParserError;

use failure::Error;

type Block = CodeWrap<SyntaxElement>;

pub struct Parser<'a> {
    pub file_name : String,
    pub raw_code : &'a str,
    pub blocks : Option<Block>,

    ////// working private members,
    // these are the raw tokens from the scanner
    tokens : Vec<TokenWrapped>,
    // these are the blocks from the reduction of phrases
    // eventually these should reduce down to a single block
    // that will be saved in `.blocks`
    working_block : Vec<Block>,
    // the actual block (do .. end), the statements inside it, 
    // and the statements that might be before it (in the case
    // of table constructors)
    //      Block : a block for the wrapping syntax element
    //      Vec<Block> : the insides of the above block
    //      Vec<Block> : anything that might be before the `Block`
    block_stack : Vec<(Block, Vec<Block>, Vec<Block>)>, 

}


impl<'a> std::default::Default for Parser<'a> {
    fn default() -> Parser<'a> {
        Parser {
            raw_code : "",
            file_name : String::from("buffer"),
            blocks : None,

            tokens : Vec::new(),
            working_block : Vec::new(),
            block_stack : Vec::new(),

        }
    }
}

impl<'a> CodeInformation for Parser<'a> {
    fn raw_code(&self) -> String { self.raw_code.to_string() }
    fn file_name(&self) -> String { self.file_name.to_string() }
}

impl<'a> Parser<'a>{
    pub fn from_scanner(scanner : Scanner<'a>) -> Parser<'a> {

        Parser {
            raw_code : scanner.raw_code,
            tokens : scanner.tokens,

            .. Parser::default()
        }
    }


    pub fn parse(mut self) -> Result<Self,Error> {
        //! works through the tokens and makes them into statements, chunks and blocks
        //! 1. it will loop through and attempt to collect all the tokens into statements
        //! 2. next it will try to group the statements as chunks
        //! 3. if we only get one chunk at the end it will then wrap it as a block
        //!    and call it a day.

        loop {
            // we get the next set of tokens that defines a phrase
            match self.get_next_working_phrase() {
                None => break,
                Some(mut phrase) => {
                    loop {
                        // this big look was made because a tableconstructor can
                        // become an expression, so i need to be able to go back over this
                        // once we make a tableconstructor (which is handled outside the 
                        // main loop)               

                        loop {
                            // check for chunk
                            // check for block
                            // don't do these here, just leaving this so i remember about them

                            println!("-------------------------");
                            for p in phrase.iter() {
                                println!("{:?}",p.item());
                            }

                            // check for statement
                            match statement::process(&mut phrase) { 
                                SyntaxResult::Done => continue,
                                SyntaxResult::Wrap(wrap_type) => {
                                    // we go inside a block, need to track that here
                                    self.block_stack.push((wrap_type, Vec::new(), Vec::new()));
                                    
                                    // and need to remove the current phrase, it should
                                    // always be garbage (or already stored in the stack)
                                    // because of the way the scanner breaks tokens and 
                                    // statements, TODO : need to make tests to prove this
                                    phrase = Vec::new();
                                    continue;
                                }
                                _ => { },
                            }
                            // check for laststatement
                            // check for funcname
                            
                            // check for varlist
                            match varlist::process(&mut phrase) { 
                                SyntaxResult::Done => continue,
                                _ => { },
                            }
                            
                            // check for var
                            match var::process(&mut phrase) { 
                                SyntaxResult::Done => continue,
                                _ => { },
                            }
                            // check for namelist
                            match namelist::process(&mut phrase) { 
                                SyntaxResult::Done => continue,
                                _ => { },
                            }
                            
                            // check for explist
                            match explist::process(&mut phrase) { 
                                SyntaxResult::Done => continue,
                                _ => { },
                            }

                            // check for expression
                            if exp::process(&mut phrase) { continue; }

                            // check for prefixexp
                            if prefixexp::process(&mut phrase) { continue; }

                            // check for functioncall
                            // check for args
                            // check for function
                            // check for functionbody
                            // check for parlist

                            // check for tableconstructor
                            match tableconstructor::process(&mut phrase) {
                                SyntaxResult::TableConst(pos) => {
                                    // gets the stuff before the table definition
                                    // if anything
                                    let the_before : Vec<Block> = phrase.drain(.. pos).collect();
                                    phrase.remove(0); // removes the "{" because we don't need that anymore.

                                    // we go inside a block (table constructor), need to track that here
                                    let dumb_table_const = CodeWrap::CodeWrap(
                                        SyntaxElement::TableConstructor(Box::new(SyntaxElement::Empty)), 0, 0);
                                    self.block_stack.push((dumb_table_const, Vec::new(), the_before));

                                    continue;
                                },
                                _ => { },
                            }
                            
                            break;
                        }

                        ///////////////////////////////////////////////
                        // checks for any statement closes in the stack
                        if phrase.len() > 0 && self.block_stack.len() > 0 {
                            // we check the last element of the phrase if its a token,
                            // if it is we check against all the statement types that can contain
                            // other statements inside of it and checks if we have the closer for the 
                            // current block_stack item
                            let pos = phrase.len() - 1;
                            if let CodeWrap::CodeWrap(SyntaxElement::Token(ref token),_, code_end) = phrase[pos] {
                                // gets the stack_item
                                let stack_pos = self.block_stack.len() - 1;

                                if &self.block_stack[stack_pos].0.item().ending_token() == token {
                                    // now we are going to construct the insides, and the end,
                                    // taking the start and anything addition from inside the `stack_item`

                                    // gets the stack
                                    let (CodeWrap::CodeWrap(ref stack_item, code_start, _), ref mut stack, ref mut prefix) = self.block_stack.remove(stack_pos);

                                    // we need to take the stuff left in the phrase (not that last token) and 
                                    // put it in the stack so we can process it correctly.
                                    phrase.remove(pos);
                                    stack.append(&mut phrase);

                                    // this match will return the code item that we will insert,
                                    let code_item : SyntaxResult = match stack_item {
                                        // Table Constructors are special in that are expecting
                                        // everything to resolve into a fields and not a statements
                                        // so we need a special catch here.
                                        
                                        SyntaxElement::TableConstructor(_) => tableconstructor::finalize(stack),
                                        SyntaxElement::StatementDoEnd(_) => statement::doend::finalize(stack),
                                        //SyntaxElement::StatementWhile(_,_) => statement::whiledoend::finalize(stack),

                                        _ => unimplemented!(),
                                    };

                                    // any error handling
                                    let new_element = match code_item {
                                        SyntaxResult::Ok(item) => CodeWrap::CodeWrap(item, code_start, code_end),
                                        SyntaxResult::Error(error_start,error_end,d) => {
                                            let (e_start, e_end) = match (error_start, error_end) {
                                                (0, 0) => (code_start, code_end),
                                                (_, 0) => (error_start, error_start),
                                                (_, _) => (error_start, error_end),
                                            };
                                            return Err(ParserError::general_error(&self, e_start, e_end,&d));
                                        }
                                        _ => unimplemented!(),
                                    };

                                    // we remove the ending token and replace it with the new
                                    // statement block
                                    phrase.push(new_element);                                

                                    // and we insert all the prefix stuff (if any)
                                    loop {
                                        if prefix.len() == 0 { break; }
                                        let len = prefix.len()-1;
                                        let piece = prefix.remove(len);
                                        phrase.insert(0,piece);
                                    }
                                }

                                continue;
                            }
                        } 

                        break;
                    }

                    // adds the phrases to wherever they need to go
                    if phrase.len() > 0 {
                        // only does this if we have items in the phrase

                        if self.block_stack.len() > 0 {
                            // we have a stack, so add the items to the 
                            // points in the stack
                            let item_no = self.block_stack.len() - 1;
                            let (_, ref mut working_phrase, _) = self.block_stack[item_no];
                            working_phrase.append(&mut phrase);
                        } else {
                            // merges the items into the working_block
                            self.working_block.append(&mut phrase);
                        }    
                    }
                }
            }
        }

        println!("===");
        for p in &self.working_block {
            println!("{}",p.item());
        }

        Ok(self)
    }

    ///////////////////////////////////////////////////////////////////////////////////
    // PRIVATE FUNCTIONS //////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////////////

    fn get_next_working_phrase(&mut self) -> Option<Vec<CodeWrap<SyntaxElement>>> {
        //! pops the tokens from the scanner (stored in self.tokens) until it 
        //! gets to a EOL or EOF and then sets it as the working_phrase. will
        //! return true or false depending on the result
        //!
        //! if there are no tokens to return, it returns None

        let mut tokens : Vec<CodeWrap<SyntaxElement>> = Vec::new();
        loop {
            loop {
                // checks that we still have tokens left to pop
                if self.tokens.len() == 0 { break; }

                match self.tokens.remove(0) {
                    CodeWrap::CodeWrap(Token::EOL, _, _) | 
                    CodeWrap::CodeWrap(Token::EOF, _, _) => break,
                    CodeWrap::CodeWrap(token, start, end) => tokens.push(CodeWrap::CodeWrap(SyntaxElement::Token(token), start, end)),
                }

            }

            // need to have this so we can check if we have 
            // empty lines (blank lines)
            match ( tokens.len(), self.tokens.len() ) {
                (0, 0) => break,
                (0, _) => continue,
                (_, _) => break,
            }
        }

        match tokens.len() {
            0 => None,
            _ => Some(tokens),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::token::Token;
    use crate::parser::Parser;
    use crate::scanner::Scanner;

    #[test]
    // #[ignore]
    pub fn quick_failure_to_see_parse() {
        let code = r#"do
            local jim
            local bob = 1 + 4
            bob = { a = 1, b = 2 }

            do
                x = x + 1
                x = -x
            end
        end"#;

        match Scanner::from_str(&code,None).scan() {
            Err(error) => println!("{}",error),
            Ok(scanner) => { 
                match Parser::from_scanner(scanner).parse() {
                    Err(error) => println!("{}",error),
                    Ok(parser) => { },
                }
            },
        }

        assert!(false)
    }

}