use crate::error::codeinfo::CodeInformation;
use crate::element::{Element, CodeElement};
use crate::scanner::Scanner;
use crate::token::{CodeToken, Token};
use crate::error::parser::ParserError;
use crate::coderef::CodeRef::CodeRef;

use failure::Error;

pub struct Parser<'a> {
    pub file_name : String,
    pub raw_code : &'a str, 
    pub blocks : Option<CodeElement>, 
}

impl<'a> CodeInformation for Parser<'a> {
    fn raw_code(&self) -> String { self.raw_code.to_string() }
    fn file_name(&self) -> String { self.file_name.to_string() }
}

impl<'a> Parser<'a> {
    pub fn from_scanner(scanner : Scanner<'a>) -> Result<Parser<'a>,Error> {
        //! creates a parser object from a scanner object. this
        //! will consume the scanner.

        let tokens = scanner.tokens;
        
        let mut parser = Parser {
            file_name : scanner.file_name,
            raw_code : scanner.raw_code,
            blocks : None,
        };

        let blocks = parser.process(tokens)?;
        parser.blocks = Some(blocks);

        Ok(parser)
    }

    // PRIVATE FUNCTIONS /////////////////////////////////////
    //////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////

    fn process(&mut self, mut tokens : Vec<CodeToken>) -> Result<CodeElement, Error> {
        //! will attempt to parse the object

        #[cfg(feature = "dev-testing")]
        println!(".. running process");

        let mut working_phrase : Vec<CodeElement> = Vec::new();

        loop {

            // the next statement of code, using LUA's statement rules
            match Parser::get_next_statement(&mut tokens) {
                None => break,
                Some(mut statement) => {
                    // now we try and match that statement to something
                    // from the lua syntax

                    working_phrase.push(self.parse(&mut statement, &mut tokens)?);
                }
            }
        }

        self.process_block(working_phrase)
    }

    fn process_block(&self, working_phrase : Vec<CodeElement>) -> Result<CodeElement,Error> {

        if working_phrase.len() == 0 {
            return Err(ParserError::general("parser found empty `working_phrase`?"))
        }

        // now we need to check if we can build a chunk and block out of the working_phrase
        for i in 0 .. working_phrase.len() - 1 {
            // checking all but the last element if it is a statement.

            if !working_phrase[i].i().is_statement() {

                #[cfg(feature = "dev-testing")]
                println!("{}",working_phrase[i].i().pretty_print());

                return Err(ParserError::not_a_statement(&self,
                    working_phrase[i].line_number(), 
                    working_phrase[i].code_start(),
                    working_phrase[i].code_end()
                ));
            }
        }

        // assuming that all passed, lets check if the last elemnet is a statement or last_statemnet
        if !(working_phrase[working_phrase.len()-1].i().is_statement() 
            || working_phrase[working_phrase.len()-1].i().is_last_statement()
        ) {
            let pos = working_phrase.len()-1;

            #[cfg(feature = "dev-testing")]
            println!("{}",working_phrase[pos].i().pretty_print());

            return Err(ParserError::not_a_statement(&self,
                working_phrase[pos].line_number(),
                working_phrase[pos].code_start(),
                working_phrase[pos].code_end()
            ));
        }

        // everything check'd out, so lets build that chunk / block
        
        let chunk = {
            let code_start : usize = working_phrase[0].code_start();
            let line_number : usize = working_phrase[0].line_number();
            let code_end : usize = working_phrase[working_phrase.len()-1].code_end();

            CodeRef {
                item : Element::create(vec![], working_phrase)?, 
                code_start, 
                code_end,
                line_number,

            }
        };

        Ok(chunk)
    }

    fn parse(&mut self, elements : &mut Vec<CodeElement>, token_pool : &mut Vec<CodeToken>) -> Result<CodeElement, Error> {

        #[cfg(feature = "dev-testing")]
        println!(".. running parse");

        loop {
            
            if Parser::process_comment(elements)? { continue; }

            // stat ::=  varlist `=´ explist | 
            if Parser::process_statement_assignment(elements)? { continue; }
            
            // stat ::=  do block end | 
            if self.process_statement_do_end(elements, token_pool)? { continue; }

            // stat ::=  while exp do block end | 
            if self.process_while_do(elements, token_pool)? { continue; }
            
            // stat ::=  repeat block until exp |
            if self.process_statement_repeat_until(elements, token_pool)? { continue; } 

            // stat ::=  if exp then block {elseif exp then block} [else block] end | 
            if self.process_if_loop(elements, token_pool)? { continue; }
            
            // stat ::=  for Name `=´ exp `,´ exp [`,´ exp] do block end | 
            // stat ::=  for namelist in explist do block end | 
            if self.process_for_loop(elements, token_pool)? { continue; }

            // stat ::=  function funcname funcbody | local function Name funcbody
            if self.process_function_definition(elements, token_pool)? { continue; }
            
            // stat ::=  local namelist [`=´ explist] 
            if Parser::process_statement_local_assignment(elements)? { continue; }

            // laststat ::= return [explist] | break
            if Parser::process_last_statement(elements)? { continue; }
            
            // funcname ::= Name {`.´ Name} [`:´ Name]

            // varlist ::= var {`,´ var}
            if Parser::process_var_list(elements)? { continue; }

            // var ::=  prefixexp `[´ exp `]´ | prefixexp `.´ Name 
            if Parser::process_var(elements)? { continue; }

            // namelist ::= Name {`,´ Name}
            if Parser::process_name_list(elements)? { continue; }

            // to make sure we catch a name_list,... before it
            // converted to an expression list, because a name_list
            // can also be an exp_list, and then we can add `...` because 
            // that is also an expression..
            if Parser::process_parlist(elements)? { continue; }

            // functioncall ::=  prefixexp args | prefixexp `:´ Name args 
            if Parser::process_functioncall(elements)? { continue; }
            if Parser::process_args(elements)? { continue; }

            // function ::= function funcbody (this is anon function, not named functions)
            if Parser::process_function(elements)? { continue; }
            if self.process_funcbody(elements, token_pool)? { continue; }

            // exp ::=  exp binop exp
            if Parser::process_binop(elements)? { continue; }

            // exp ::=  unop exp
            if Parser::process_unop(elements)? { continue; }

            // explist ::= {exp `,´} exp
            if Parser::process_exp_list(elements)? { continue; }

            // prefixexp ::= `(´ exp `)´
            if Parser::process_prefix_exp(elements)? { continue; }

            // tableconstructor ::= `{´ [fieldlist] `}´
            if self.process_table_constructor(elements, token_pool)? { continue; }
            if Parser::process_field(elements)? { continue; }
            if Parser::process_field_list(elements)? { continue; }

            break;
        }

        match elements.len() {
            1 => Ok(elements.remove(0)),
            0 => return Err(ParserError::general("parser found an empty statement?")),
            _ => return Err(ParserError::not_a_statement(&self,
                elements[0].line_number(), elements[0].code_start(),
                elements[elements.len()-1].code_end()))
        }
    }

    fn get_next_statement(tokens : &mut Vec<CodeToken>) -> Option<Vec<CodeElement>> {
        //! gets the next state of tokens that makes as statement. there are a few
        //! cases where this won't be accurate (such as table definitions using ';')
        //! because it looks for EOL and ';' characters to draw the statement line
        //! 
        //! also added that comments are treated as newline elements, because each
        //! comment is kind of its own entity in the world, so if you have code and 
        //! then a comment afterwards that can be considered as 2 statements.

        let mut phrase : Vec<CodeElement> = Vec::new();
        loop {
            // makes sure we still have tokens to work with.
            if tokens.len() == 0 { break; }

            let token = tokens.remove(0);

            // for a comment, we need to place it back inside so we get it later
            if token == Token::Comment("".to_string()) && phrase.len() > 0 {
                tokens.insert(0,token);
                break;
            }
            // we reached the end and have some tokens in the phrase,
            // lets send that back
            if (token == Token::EOL || token == Token::SemiColon) && phrase.len() > 0 { break; }
            // we reached a line break with nothing on it, but we
            // don't have anything to send back, we aren't necessarily
            // at the end of the token list (since that would break us
            // above) so lets just keep trying
            else if token == Token::EOL || token == Token::SemiColon || token == Token::WhiteSpace { continue; }
            // the default action, send it to the phrase
            else { 
                let new_element = Element::codeelement_from_token(token);
                phrase.push(new_element);
            }

        }

        match phrase.len() {
            0 => None,
            _ => Some(phrase)
        }
    }

    fn get_tokens_until_token(&mut self, elements : &mut Vec<CodeElement>, token_pool : &mut Vec<CodeToken>, token : Token) -> Result<(),Error> {
    
        let mut nesting_stack : Vec<Token> = Vec::new();
        let original_length : usize = elements.len();
        let mut cursor : usize = 0;
        let mut added_semicolon = false;

        loop {
            // checks if we run out of runway
            if cursor >= elements.len() { 
                // first we check for the semicolon hack
                if !added_semicolon {
                    let code_start = elements[elements.len()-1].code_end();
                    let line_number = elements[elements.len()-1].line_number();
                    let code_end = code_start + 1;

                    elements.push(Element::codeelement_from_token(CodeRef{
                        item : Token::SemiColon,
                        code_start, code_end, line_number
                    }));

                    added_semicolon = true;
                }


                if token_pool.len() > 0 {
                    // if we ran out of space we should first see if we can
                    // add another character from the pool
                    let popd_token = token_pool.remove(0);
                    
                    if popd_token != Token::WhiteSpace {
                        elements.push(Element::codeelement_from_token(popd_token)); 
                    }
                    
                    continue;
                } else {
                    // if not we will error because we haven't found what we were 
                    // looking for (assuming because we are here)
                    return Err(ParserError::unterminated(&self, 
                        elements[0].line_number(), elements[0].code_start(),
                        elements[elements.len()-1].code_end(),&format!("can't find the '{}' to close the phrase",token))
                    );
                }
            }

            if let Some(element_token) = elements[cursor].i().get_token() {
                // checks if we should be adding another nesting level
                if let Some(ending) = element_token.i().matching_set() { nesting_stack.push(ending); }
                // now checks if we ended a nest
                else if nesting_stack.len() > 0 {
                    if element_token == &nesting_stack[nesting_stack.len()-1] {
                        // we remove that level
                        nesting_stack.pop();
                    }
                }
            }

            // we increment it
            cursor += 1;

            // we are also checking against the original length, because we
            // need to first capture the opening token, we might be too early
            // in the phrase and there could be other tokens infront that would
            // cause us to break before we want to.
            if nesting_stack.len() == 0 && cursor > original_length { break; }
        }

        if let Some(last_token) = elements[elements.len()-1].i().get_token() {
            if last_token != &token {
                return Err(ParserError::unterminated(&self, 
                    elements[0].line_number(), elements[0].code_start(),
                    elements[elements.len()-1].code_end(),&format!("expected '{}' to close the phrase, but found '{}'",token, last_token))
                );
            }
        } 

        Ok(())    
    }

    // checking functions
    fn process_comment(elements : &mut Vec<CodeElement>) -> Result<bool,Error> {
        if elements.len() >= 2 {
            if elements[0].i().matches_token(Token::Minus)
            && elements[0].i().matches_token(Token::Minus) {

                let minus1 = elements.remove(0);
                let minus2 = elements.remove(1);

                let code_start : usize = minus1.code_start();
                let line_number : usize = minus1.line_number();
                let code_end : usize = elements[elements.len()-1].code_end();

                let item = Element::create(vec![minus1, minus2], elements.drain(..).collect())?;

                elements.insert(0, CodeRef{
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed comment");

                return Ok(true);
            }
        }

        Ok(false)
    }
    
    fn process_exp_list(elements : &mut Vec<CodeElement>) -> Result<bool,Error> {
        //! [ ] {exp `,´} exp

        // we need to start at every position and keep going until we hit 
        // something that doesn't fit the pattern anymore
        for i in 0 .. elements.len() {
            // the big loop, this is the starting character

            // if the first element isn't an expression we should just go
            // to the next iteration of the big loop
            if !elements[i].i().is_exp() { continue; }

            let mut ending : usize = i; 

            for j in i+1 .. elements.len() {
                // this is the little loop, `i` is always the first character
                // or the start of the element phrase.  

                if (j-i) % 2 == 1 {
                    // this should be the alternated tokens, so in ourcase
                    // these should be commas
                    if let Some(token) = elements[j].i().get_token() {
                        if token != &Token::Comma { break; }
                    } else { break; /* this doesn't fit the pattern, we should leave */ }
                } else {
                    // this should be an expression
                    if !elements[j].i().is_exp() { break; }
                }

                // move the ending because it matches.
                ending = j;
            }

            // now we check if we got anything useful.
            if i != ending && (ending-i) % 2 == 0 {
                // we are checking that (1) they are not the same, and the difference
                // is even => a,b,c should be 0,4 which is even. you can't end an exp
                // list with a , so there should always be 5 tokens, which makes a dif
                // of 4

                let mut exps : Vec<CodeElement> = Vec::new();

                for cc in 0 .. (ending-i+1) {
                    // we will iterate and remove all the components, and add the exp
                    // to the vec<>
 
                    let token = elements.remove(i);
                    if cc % 2 == 0 {
                        exps.push(token);
                    }
                }

                if exps.len() == 0 {
                    return Err(ParserError::general("expression list was found, but parsed as empty??"));
                }

                let code_start : usize = exps[0].code_start();
                let line_number : usize = exps[0].line_number();
                let code_end : usize = exps[exps.len()-1].code_end();

                let item = Element::create(vec![], exps)?;

                elements.insert(i, CodeRef{
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed explist");

                return Ok(true);
            }
        }

        Ok(false)
    }

    fn process_var(elements : &mut Vec<CodeElement>) -> Result<bool, Error> {
        //! - prefixexp `[´ exp `]´
        //! - prefixexp `.´ Name 

        // checks `prefix.name`
        if elements.len() >= 3 { for i in 0 .. elements.len() - 2 {
            if elements[i].i().is_prefix_exp()
            && elements[i+1].i().matches_token(Token::Period)
            && elements[i+2].i().is_name() {

                let prefix = elements.remove(i);
                let period = elements.remove(i);
                let name = elements.remove(i);

                let code_start : usize = prefix.code_start();
                let line_number : usize = prefix.line_number();
                let code_end : usize = name.code_end();

                let item = Element::create(vec![period], vec![prefix, name])?;

                elements.insert(i, CodeRef {
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed var(3)");

                return Ok(true);
            }
        }}

        // checks `prefix[exp]`
        if elements.len() >= 4 { for i in 0 .. elements.len() - 3 {
            if elements[i].i().is_prefix_exp()
            && elements[i+1].i().matches_token(Token::LeftBracket)
            && elements[i+2].i().is_exp() 
            && elements[i+3].i().matches_token(Token::RightBracket) {

                let prefix = elements.remove(i);
                let left = elements.remove(i);
                let exp = elements.remove(i);
                let right = elements.remove(i);

                let code_start : usize = prefix.code_start();
                let line_number : usize = prefix.line_number();
                let code_end : usize = right.code_end();

                let item = Element::create(vec![left, right], vec![prefix, exp])?;

                elements.insert(i, CodeRef {
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed var(2)");

                return Ok(true);
            }
        }}

        Ok(false)
    }

    fn process_var_list(elements : &mut Vec<CodeElement>) -> Result<bool,Error> {
        //! [ ] {var `,´} var

        // TODO , generic this and make all the list use the same base function.

        // we need to start at every position and keep going until we hit 
        // something that doesn't fit the pattern anymore
        for i in 0 .. elements.len() {
            // the big loop, this is the starting character

            // if the first element isn't an expression we should just go
            // to the next iteration of the big loop
            if !elements[i].i().is_var() { continue; }

            let mut ending : usize = i; 

            for j in i+1 .. elements.len() {
                // this is the little loop, `i` is always the first character
                // or the start of the element phrase.  

                if (j-i) % 2 == 1 {
                    // this should be the alternated tokens, so in ourcase
                    // these should be commas
                    if let Some(token) = elements[j].i().get_token() {
                        if token != &Token::Comma { break; }
                    } else { break; /* this doesn't fit the pattern, we should leave */ }
                } else {
                    // this should be an expression
                    if !elements[j].i().is_var() { break; }
                }

                // move the ending because it matches.
                ending = j;
            }

            // now we check if we got anything useful.
            if i != ending {
                // we are checking that (1) they are not the same, we can't end
                // a list with a comma, but we might have one because of something
                // else (like namelist, ...) so we should not check for it here.
                
                // we get rid of the ending ',' is there is one by moving the
                // ending
                if elements[ending].i().matches_token(Token::Comma) { ending -= 1; }

                let mut exps : Vec<CodeElement> = Vec::new();

                for cc in 0 .. (ending-i+1) {
                    // we will iterate and remove all the components, and add the exp
                    // to the vec<>
 
                    let token = elements.remove(i);
                    if cc % 2 == 0 {
                        exps.push(token);
                    }
                }

                if exps.len() == 0 {
                    return Err(ParserError::general("expression list was found, but parsed as empty??"));
                }

                let code_start : usize = exps[0].code_start();
                let line_number : usize = exps[0].line_number();
                let code_end : usize = exps[exps.len()-1].code_end();

                let item = Element::create(vec![], exps)?;

                elements.insert(i, CodeRef{
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed var list");

                return Ok(true);
            }
        }

        Ok(false)
    }


    fn process_name_list(elements : &mut Vec<CodeElement>) -> Result<bool,Error> {
        //! [ ] {Name `,´} Name

        // we need to start at every position and keep going until we hit 
        // something that doesn't fit the pattern anymore
        for i in 0 .. elements.len() {
            // the big loop, this is the starting character

            // if the first element isn't an expression we should just go
            // to the next iteration of the big loop
            if !elements[i].i().is_name() { continue; }

            let mut ending : usize = i; 

            for j in i+1 .. elements.len() {
                // this is the little loop, `i` is always the first character
                // or the start of the element phrase.  

                if (j-i) % 2 == 1 {
                    // this should be the alternated tokens, so in ourcase
                    // these should be commas
                    if let Some(token) = elements[j].i().get_token() {
                        if token != &Token::Comma { break; }
                    } else { break; /* this doesn't fit the pattern, we should leave */ }
                } else {
                    // this should be an expression
                    if !elements[j].i().is_name() { break; }
                }

                // move the ending because it matches.
                ending = j;
            }

            // now we check if we got anything useful.
            if i != ending {
                // we are checking that (1) they are not the same, we can't end
                // a list with a comma, but we might have one because of something
                // else (like namelist, ...) so we should not check for it here.
                
                // we get rid of the ending ',' is there is one by moving the
                // ending
                if elements[ending].i().matches_token(Token::Comma) { ending -= 1; }

                let mut exps : Vec<CodeElement> = Vec::new();

                for cc in 0 .. (ending-i+1) {
                    // we will iterate and remove all the components, and add the exp
                    // to the vec<>
 
                    let token = elements.remove(i);
                    if cc % 2 == 0 {
                        exps.push(token);
                    }
                }

                if exps.len() == 0 {
                    return Err(ParserError::general("expression list was found, but parsed as empty??"));
                }

                let code_start : usize = exps[0].code_start();
                let line_number : usize = exps[0].line_number();
                let code_end : usize = exps[exps.len()-1].code_end();

                let item = Element::create(vec![], exps)?;

                elements.insert(i, CodeRef{
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed name list");

                return Ok(true);
            }
        }

        Ok(false)
    }

    fn process_args(elements : &mut Vec<CodeElement>) -> Result<bool, Error> {
        // `(´ [explist] `)´

        if elements.len() >= 3 { for i in 0 .. elements.len() - 2 { 
            if elements[i].i().matches_token(Token::LeftParen)
            && elements[i+1].i().is_exp_list()
            && elements[i+2].i().matches_token(Token::RightParen) {
                let left = elements.remove(i);
                let explist = elements.remove(i);
                let right = elements.remove(i);

                let code_start : usize = left.code_start();
                let line_number : usize = left.line_number();
                let code_end : usize = right.code_end();

                let item = Element::create(vec![left, right], vec![explist])?;
                elements.insert(i, CodeRef {
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed args");

                return Ok(true);
            }
        }}

        Ok(false)
    }

    fn process_function(elements : &mut Vec<CodeElement>) -> Result<bool, Error> {
        //! function funcbody
        if elements.len() >= 2 { for i in 0 .. elements.len()-1 {
            if elements[i].i().matches_token(Token::Function) 
            && elements[i+1].i().is_func_body() {
                let function = elements.remove(i);
                let funcbody = elements.remove(i);

                let code_start : usize = function.code_start();
                let line_number : usize = function.line_number();
                let code_end : usize = funcbody.code_end();

                let item = Element::create(vec![function], vec![funcbody])?;
                elements.insert(i, CodeRef {
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed function");

                return Ok(true);

            }
        }}

        Ok(false)
    }

    fn process_funcbody(&mut self, elements : &mut Vec<CodeElement>, token_pool : &mut Vec<CodeToken>) -> Result<bool, Error> {
        // `(´ [parlist] `)´ block end
        // most of the time, this is multilined, so we're going to only
        // check for 3 tokens first.

        if elements.len() >= 3 { for i in 0 .. elements.len() - 2 {
            if elements[i].i().matches_token(Token::LeftParen)
            && elements[i+1].i().is_par_list()
            && elements[i+2].i().matches_token(Token::RightParen) {
                self.get_tokens_until_token(elements, token_pool, Token::End)?;

                let left = elements.remove(i);
                let parlist = elements.remove(i);
                let right = elements.remove(i);
                let final_end = elements.remove(elements.len()-1);
                let block = self.process(elements.drain(i ..).collect())?;

                let code_start : usize = left.code_start();
                let line_number : usize = left.line_number();
                let code_end : usize = final_end.code_end();

                let item = Element::create(vec![left,right,final_end], vec![parlist,block])?;

                elements.insert(i, CodeRef {
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed funcbody");

                return Ok(true);
            }
        }}

        Ok(false)
    }

    fn process_parlist(elements : &mut Vec<CodeElement>) -> Result<bool, Error> {
        // namelist `,´ `...´

        if elements.len() >= 3 { for i in 0 .. elements.len() - 2 {
            if elements[i].i().is_name_list()
            && elements[i+1].i().matches_token(Token::Comma)
            && elements[i+2].i().matches_token(Token::TriplePeriod) {
                let namelist = elements.remove(i);
                let _comma = elements.remove(i);
                let tripleperiod = elements.remove(i);

                let code_start : usize = namelist.code_start();
                let line_number : usize = namelist.line_number();
                let code_end : usize = tripleperiod.code_end();

                let item = Element::create(vec![], vec![namelist,tripleperiod])?;
                elements.insert(i, CodeRef {
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed parlist");

                return Ok(true);
            }
        }}
    
        Ok(false)
    }

    fn process_functioncall(elements : &mut Vec<CodeElement>) -> Result<bool, Error> {
        //!  - prefixexp args
        //!  - prefixexp `:´ Name args 

        // prefixexp:Name args
        if elements.len() >= 4 { for i in 0 .. elements.len() - 3 {
            if elements[i].i().is_prefix_exp()
            && elements[i+1].i().matches_token(Token::Colon) 
            && elements[i+2].i().is_name() 
            && elements[i+3].i().is_args() {

                let prefix = elements.remove(i);
                let colon = elements.remove(i);
                let name = elements.remove(i);
                let args = elements.remove(i);

                let code_start : usize = prefix.code_start();
                let line_number : usize = prefix.line_number();
                let code_end : usize = args.code_end();

                let item = Element::create(vec![colon], vec![prefix, name, args])?;
                
                elements.insert(i, CodeRef {
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed function call (4)");

                return Ok(true);
            }
        }}

        // prefixexp args
        if elements.len() >= 2 { for i in 0 .. elements.len() - 1 {
            if elements[i].i().is_prefix_exp()
            && elements[i+1].i().is_args() {
                let prefix = elements.remove(i);
                let args = elements.remove(i);

                let code_start : usize = prefix.code_start();
                let line_number : usize = prefix.line_number();
                let code_end : usize = args.code_end();

                let item = Element::create(vec![], vec![prefix, args])?;
                elements.insert(i, CodeRef {
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed function call (2)");

                return Ok(true);
            }
        }}

        Ok(false)
    }

    fn process_prefix_exp(statement : &mut Vec<CodeElement>) -> Result<bool,Error> {
        //! prefixexp ::= `(´ exp `)´

        if statement.len() >= 3 { for i in 0 .. statement.len() - 2 {

            if statement[i].i().matches_token(Token::LeftParen) 
            && statement[i+1].i().is_exp() 
            && statement[i+2].i().matches_token(Token::RightParen) {

                let left = statement.remove(i);
                let exp = statement.remove(i);
                let right = statement.remove(i);

                let code_start : usize = left.code_start();
                let line_number : usize = left.line_number();
                let code_end : usize = right.code_end();

                let item = Element::create(vec![left,right],vec![exp])?;

                statement.insert(i,CodeRef{
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed prefixexp");

                return Ok(true);
            }
        }}

        Ok(false)
    }

    fn process_binop(statement : &mut Vec<CodeElement>) -> Result<bool,Error> {
        if statement.len() >= 3 { for i in 0 .. statement.len() - 2 {
            // checks the standard format of `EXP (binop) EXP`

            if statement[i].i().is_exp() && statement[i+1].i().is_binop_token() && statement[i+2].i().is_exp() {
                // remove the pieces we care about
                let exp1 = statement.remove(i);
                let op = statement.remove(i);
                let exp2 = statement.remove(i);

                let code_start = exp1.code_start();
                let line_number = exp1.line_number();
                let code_end = exp2.code_end();

                let item = Element::create(vec![op],vec![exp1, exp2])?;

                statement.insert(i, CodeRef { item, code_end, code_start, line_number });

                #[cfg(feature = "dev-testing")]
                println!(".. processed binop");

                return Ok(true);
            }
        }}

        Ok(false)
    }

    fn process_unop(statement : &mut Vec<CodeElement>) -> Result<bool,Error> {
        if statement.len() >= 2 { for i in 0 .. statement.len() - 1 {
            // checks for the `(unop) EXP` format

            if statement[i].i().is_unop_token() && statement[i+1].i().is_exp() {
                let op = statement.remove(i);
                let exp = statement.remove(i);

                let code_start = op.code_start();
                let line_number = op.line_number();
                let code_end = exp.code_end();

                let item = Element::create(vec![op], vec![exp])?;

                statement.insert(i, CodeRef { item, code_end, code_start, line_number });

                #[cfg(feature = "dev-testing")]
                println!(".. processed unop");

                return Ok(true);
            }
        }}

        Ok(false)
    }

    fn process_statement_assignment(statement: &mut Vec<CodeElement>) -> Result<bool,Error> {
        //! varlist `=´ explist

        if statement.len() == 3 {
            if let Some(ref token) = statement[1].i().get_token() {
                if token.i() == Token::Equal 
                && statement[0].i().is_var_list()
                && statement[2].i().is_exp_list() {
                    
                    let vars = statement.remove(0);
                    let op = statement.remove(0);
                    let exp = statement.remove(0);

                    let code_start = vars.code_start();
                    let code_end = exp.code_end();
                    let line_number = vars.line_number();

                    let new_element = Element::create(
                        vec![op],
                        vec![vars, exp])?;

                    statement.insert(0,CodeRef{
                        item : new_element,
                        code_start, code_end, line_number
                    });

                    #[cfg(feature = "dev-testing")]
                    println!(".. processed 'statement assignment'");
                    
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    fn process_statement_local_assignment(elements : &mut Vec<CodeElement>) -> Result<bool,Error> {
        //! local namelist [`=´ explist] 

        // checks for blank assignment
        if elements.len() == 2 {
            if elements[0].i().matches_token(Token::Local)
            && elements[1].i().is_name_list() {
                let local = elements.remove(0);
                let name_list = elements.remove(0);

                let code_start = local.code_start();
                let code_end = name_list.code_end();
                let line_number = local.line_number();

                let item = Element::create(vec![local], vec![name_list])?;

                elements.insert(0,CodeRef{
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed statement local assignment (2)");

                return Ok(true);
            }
        }

        // checks for real assignment
        if elements.len() == 4 {
            if elements[0].i().matches_token(Token::Local)
            && elements[1].i().is_name_list() 
            && elements[2].i().matches_token(Token::Equal) 
            && elements[3].i().is_exp_list() {

                let local = elements.remove(0);
                let name_list = elements.remove(0);
                let eq = elements.remove(0);
                let exp_list = elements.remove(0);

                let code_start = local.code_start();
                let code_end = exp_list.code_end();
                let line_number = local.line_number();

                let item = Element::create(vec![local,eq], vec![name_list,exp_list])?;

                elements.insert(0,CodeRef{
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed statement local assignment (4)");

                return Ok(true);
            }
        }

        Ok(false)
    }
    fn process_if_loop(&mut self, elements : &mut Vec<CodeElement>, token_pool : &mut Vec<CodeToken>) -> Result<bool, Error> {
        //! if exp then block {elseif exp then block} [else block] end

        if elements[0].i().matches_token(Token::If) {
            self.get_tokens_until_token(elements, token_pool, Token::End)?;

            let if_token = elements.remove(0);
            let end_token = elements.remove(elements.len()-1);
            let mut empty_pool : Vec<CodeToken> = Vec::new();

            let code_start = if_token.code_start();
            let code_end = end_token.code_end();
            let line_number = if_token.line_number();

            let (then_token, exp) = if let Some(pos) = Parser::contains_token(elements, Token::Then) {
                let mut insides : Vec<CodeElement> = elements.drain(.. pos).collect();
                let exp = self.parse(&mut insides, &mut empty_pool)?;

                (elements.remove(0), exp)
            } else {
                return Err(ParserError::unexpected(self, if_token.line_number(), if_token.code_start(), end_token.code_end(),
                    "a if-then-end requires a `then`"));
            };

            let mut toks : Vec<CodeElement> = vec![if_token, then_token];
            let mut elms : Vec<CodeElement> = vec![exp];

            let mut working : Vec<CodeToken> = Vec::new();
            // now we need to figure out where the block ends, we coul;d have an `elseif`, and `else` or nothing at all.
            // we'll do this by reading through the remaining elements in a loop until we've got everything done.
            // if exp then block {elseif exp then block} [else block] end

            loop {
                if elements.len() == 0 { break; }

                let token = elements.remove(0);

                match token.i().get_token() {
                    Some(CodeRef { item : Token::Elseif, .. }) => { 
                        // the working stuff should be a block
                        let block = self.process(working)?;
                        working = Vec::new();

                        toks.push(token);
                        elms.push(block);
                    },
                    Some(CodeRef { item : Token::Else, .. }) => { 
                        // the working stuff should be a block
                        let block = self.process(working)?;
                        working = Vec::new();

                        toks.push(token);
                        elms.push(block);
                    },
                    Some(CodeRef { item : Token::Then, .. }) => { 
                        // before a `then` is always some kind of condition
                        let mut insides : Vec<CodeElement> = working.drain(..).collect();
                        let exp = self.parse(&mut insides, &mut empty_pool)?;

                        elms.push(exp);
                    },
                    Some(_) => {
                        // this is safe because we needed to get here from
                        // .get_token() which can only return Some(_) from
                        // a token.
                        working.push(token.unwrap().consume_to_token().unwrap());
                    },
                    None => {
                        return Err(ParserError::unexpected(self, token.line_number(), token.code_start(), token.code_end(),
                            "unexpected item"));     
                    }
                }

                if elements.len() == 0 {
                    // doing it here also so we can keep the code for this inside the loop,
                    // and not have a weird block at the end of the loop
                    // effectively we are checking 2x every iteration of the loop, but 
                    // i'm ok with that right now. TODO : probably can clean this up
                    if working.len() > 0 {
                        // the working stuff should be a block
                        let block = self.process(working)?;
                        working = Vec::new();

                        elms.push(block);
                    }
                }
            }

            toks.push(end_token);
            let item = Element::create(toks,elms)?;

            elements.insert(0,CodeRef{
                item, code_start, code_end, line_number
            });

            #[cfg(feature = "dev-testing")]
            println!(".. processed if-then-else");

            return Ok(true);
        }
    

        Ok(false)
    }

    fn process_for_loop(&mut self, elements : &mut Vec<CodeElement>, token_pool : &mut Vec<CodeToken>) -> Result<bool, Error> {
        //! stat ::=  for Name `=´ exp `,´ exp [`,´ exp] do block end | 
        //! stat ::=  for namelist in explist do block end | 
        //! 
        //! checks if we start off with a `for` and then continues working to find the right
        //! pattern, does both for loops.

        if elements[0].i().matches_token(Token::For) {
            self.get_tokens_until_token(elements, token_pool, Token::End)?;

            let for_token = elements.remove(0);
            let end_token = elements.remove(elements.len()-1);
            let (do_token, block) = {
                let mut do_pos : usize = 0;
                    
                loop {
                    // finding the 'do' element in the phrase

                    if do_pos >= elements.len() { 
                        return Err(ParserError::unexpected(self,
                            elements[do_pos-1].line_number(),
                            elements[do_pos-1].code_start(),
                            elements[do_pos-1].code_end(),
                            "must have a 'do' element when using a for loop"
                        ));
                    }

                    if elements[do_pos].i().matches_token(Token::Do) {
                        break;
                    }

                    do_pos += 1;
                }

                let block = self.process(elements.drain(do_pos + 1 ..).collect())?;
                let do_token = elements.remove(do_pos);

                (do_token, block)
            };

            // we did everything we can for the shared space between the two, now we need to figure 
            // out which direction to go
            if let Some(pos) = Parser::contains_token(&elements, Token::In) {
                // for <<namelist in explist>> do block end

                let name_list = {
                    let mut blank_pool : Vec<CodeToken> = Vec::new();
                    let mut insides : Vec<CodeElement> = elements.drain(.. pos).collect();
                    self.parse(&mut insides, &mut blank_pool)?
                };
                let in_token = elements.remove(0);
                let exp_list = {
                    let mut blank_pool : Vec<CodeToken> = Vec::new();
                    let mut insides : Vec<CodeElement> = elements.drain(..).collect();
                    self.parse(&mut insides, &mut blank_pool)?
                };

                if !name_list.i().is_name_list() {
                    return Err(ParserError::unexpected(self, name_list.line_number(), name_list.code_start(), name_list.code_end(),
                        "expected a namelist"));
                }

                if !exp_list.i().is_exp_list() {
                    return Err(ParserError::unexpected(self, exp_list.line_number(), exp_list.code_start(), exp_list.code_end(),
                        "expected an explist"));
                }

                let code_start = for_token.code_start();
                let code_end = end_token.code_end();
                let line_number = for_token.line_number();

                let item = Element::create(
                    vec![for_token,in_token,do_token,end_token], 
                    vec![name_list,exp_list,block]
                )?;

                elements.insert(0,CodeRef{
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed for-in-loop");
                
                return Ok(true);

            } else if let Some(_) = Parser::contains_token(&elements, Token::Equal) {
                // for <<Name `=´ exp `,´ exp [`,´ exp]>> do block end | 

                let name = elements.remove(0);
                if !name.i().is_name() {
                    return Err(ParserError::unexpected(self, name.line_number(), name.code_start(), name.code_end(),
                        "expected a name"));    
                }

                let eq_token = elements.remove(0);
                if !eq_token.i().matches_token(Token::Equal) {
                    return Err(ParserError::unexpected(self, eq_token.line_number(), eq_token.code_start(), eq_token.code_end(),
                        "expected `=`"));    
                }

                let exp1 = elements.remove(0);
                if !exp1.i().is_exp() {
                    return Err(ParserError::unexpected(self, exp1.line_number(), exp1.code_start(), exp1.code_end(),
                        "expected expression"));    
                }

                let comma1 = elements.remove(0);
                if !comma1.i().matches_token(Token::Comma) {
                    return Err(ParserError::unexpected(self, comma1.line_number(), comma1.code_start(), comma1.code_end(),
                        "expected `,`"));    
                }

                let exp2 = elements.remove(0);
                if !exp2.i().is_exp() {
                    return Err(ParserError::unexpected(self, exp2.line_number(), exp2.code_start(), exp2.code_end(),
                        "expected expression"));    
                }

                if elements.len() != 0 && elements.len() != 2 {
                    return Err(ParserError::unexpected(self, elements[0].line_number(), elements[0].code_start(), elements[elements.len()-1].code_end(),
                        "expected either nothing, or `, exp`"));    
                }

                let code_start = for_token.code_start();
                let code_end = end_token.code_end();
                let line_number = for_token.line_number();

                let mut exps : Vec<CodeElement> = vec![name, exp1, exp2];
                let mut toks : Vec<CodeElement> = vec![for_token, eq_token, comma1];

                if elements.len() == 2 {
                    // gets the last section if defined.
                    
                    let comma = elements.remove(0);
                    if !comma.i().matches_token(Token::Comma) {
                        return Err(ParserError::unexpected(self, comma.line_number(), comma.code_start(), comma.code_end(),
                            "expected `,`"));    
                    }

                    let exp = elements.remove(0);
                    if !exp.i().is_exp() {
                        return Err(ParserError::unexpected(self, exp.line_number(), exp.code_start(), exp.code_end(),
                            "expected expression"));    
                    }

                    toks.push(comma);
                    exps.push(exp);
                }

                exps.push(block);
                toks.push(do_token);
                toks.push(end_token);

                let item = Element::create(
                    toks, 
                    exps
                )?;

                elements.insert(0,CodeRef{
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed for-=-loop");
                
                return Ok(true);

            } else {
                return Err(ParserError::unexpected(self, for_token.line_number(), for_token.code_start(), for_token.code_end(),
                    "expected either an 'in' phrase or a '=' phrase for this for-loop"));
            }

        }

        Ok(false)
    }

    fn process_function_definition(&mut self, elements : &mut Vec<CodeElement>, token_pool : &mut Vec<CodeToken>) -> Result<bool, Error> {

        // stat ::=  local function Name funcbody |  
        if elements.len() >= 4 { for i in 0 .. elements.len() - 3 {
            if elements[i].i().matches_token(Token::Local) 
            && elements[i+1].i().matches_token(Token::Function)
            && elements[i+2].i().is_name()
            && elements[i+3].i().is_func_body() {

                let local = elements.remove(0);
                let function = elements.remove(0);
                let name = elements.remove(0);
                let funcbody = elements.remove(0);

                let code_start = local.code_start();
                let code_end = funcbody.code_end();
                let line_number = local.line_number();

                let item = Element::create(vec![local,function], vec![name,funcbody])?;

                elements.insert(0,CodeRef{
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed function definition (local)");

                return Ok(true);

            }
        }}
        
        // stat ::=  function funcname funcbody |
        if elements[0].i().matches_token(Token::Function) {
            self.get_tokens_until_token(elements, token_pool, Token::End)?;

            let (start_of_funcbody, end_of_funcbody) = {
                let mut pos = 0;
                let mut end = 0;
                for j in 0 .. elements.len() {
                    if elements[j].i().matches_token(Token::LeftParen) {
                        pos = j-1;
                    }
                    if elements[j].i().matches_token(Token::RightParen) {
                        end = j;
                        break;
                    }
                }
                (pos, end)
            };

            let function = elements.remove(0);
            let final_end = elements.remove(elements.len()-1);

            // a hack to build the funcbody object.
            let funcbody = {
                let funcbody = self.process(elements.drain(end_of_funcbody .. ).collect())?;
                let parmaarea = {
                    let mut param : Vec<CodeToken> = elements.drain(start_of_funcbody + 1 ..  elements.len() - 1).collect();
                    let mut temp_elements = Parser::get_next_statement(&mut param).unwrap();
                    self.parse(&mut temp_elements, &mut param)?
                };
                
                if !funcbody.i().is_block() {
                    return Err(ParserError::unexpected(&self, funcbody.line_number(), funcbody.code_start(), funcbody.code_end(),
                        "expected a function body here."));
                }
                if !parmaarea.i().is_par_list() {
                    return Err(ParserError::unexpected(&self, parmaarea.line_number(), parmaarea.code_start(), parmaarea.code_end(),
                        "expected function parmeters here."));
                }

                let right = elements.remove(elements.len()-1);
                let left = elements.remove(elements.len()-1);

                let code_start = left.code_start();
                let code_end = final_end.code_end();
                let line_number = left.line_number();

                let item = Element::create(vec![left,right,final_end], vec![parmaarea,funcbody])?;

                CodeRef{ item, code_start, code_end, line_number }
            };

            if !funcbody.i().is_func_body() {
                return Err(ParserError::unexpected(&self, funcbody.line_number(), funcbody.code_start(), funcbody.code_end(),
                    "expected a function body here."));
            }

            let funcname = {
                let mut funcname : Vec<CodeToken> = elements.drain(..).collect();
                let mut temp_elements = Parser::get_next_statement(&mut funcname).unwrap();
                self.parse(&mut temp_elements, &mut funcname)?
            };

            if !funcname.i().is_func_name() {
                return Err(ParserError::unexpected(&self, funcname.line_number(), funcname.code_start(), funcname.code_end(),
                    "expected a function name here."));
            }

            let code_start = function.code_start();
            let code_end = funcbody.code_end();
            let line_number = function.line_number();

            let item = Element::create(vec![function], vec![funcname,funcbody])?;

            elements.insert(0,CodeRef{
                item, code_start, code_end, line_number
            });

            #[cfg(feature = "dev-testing")]
            println!(".. processed function definition");

            return Ok(true);
        }

        Ok(false)
    }

    fn process_while_do(&mut self, elements : &mut Vec<CodeElement>, token_pool : &mut Vec<CodeToken>) -> Result<bool, Error> {

        if elements.len() > 0 {
            if elements[0].i().matches_token(Token::While) {
                self.get_tokens_until_token(elements, token_pool, Token::End)?;

                let while_token = elements.remove(0);
                let end_token = elements.remove(elements.len()-1);
                let (condition, do_token, block) = {
                    let mut do_pos : usize = 0;
                    
                    loop {
                        // finding the 'do' element in the phrase

                        if do_pos >= elements.len() { 
                            return Err(ParserError::unexpected(self,
                                elements[do_pos-1].line_number(),
                                elements[do_pos-1].code_start(),
                                elements[do_pos-1].code_end(),
                                "must have a 'do' element when using a while"
                            ));
                        }

                        if elements[do_pos].i().matches_token(Token::Do) {
                            break;
                        }

                        do_pos += 1;
                    }

                    let insides : Vec<CodeToken> = elements.drain(do_pos + 1 ..).collect();
                    let block = self.process(insides)?;
                    let do_token : CodeElement = elements.remove(do_pos);

                    let mut bank : Vec<CodeToken> = Vec::new();
                    let condition = self.parse(elements, &mut bank)?;

                    (condition, do_token, block)
                };

                let code_start = while_token.code_start();
                let code_end = end_token.code_end();
                let line_number = while_token.line_number();

                let item = Element::create(
                    vec![while_token, do_token, end_token], 
                    vec![condition, block])?;

                elements.insert(0,CodeRef{
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed while-do");

                return Ok(true);
            }
        }

        Ok(false)
    }

    fn process_statement_repeat_until(&mut self, elements : &mut Vec<CodeElement>, token_pool : &mut Vec<CodeToken>) -> Result<bool, Error> {

        // check that the first token is a `repeat`
        if elements.len() > 0 {
            if elements[0].i().matches_token(Token::Repeat) {
                self.get_tokens_until_token(elements, token_pool, Token::Until)?;

                // gets my bits to build the element
                let repeat_token = elements.remove(0);
                let until_token = elements.remove(elements.len()-1);
                let repeat_statement = self.process(elements.drain(..).collect())?;
                let until_statement = {
                    match Parser::get_next_statement(token_pool) {
                        None => return Err(ParserError::unterminated(&self, 
                            repeat_token.line_number(), repeat_token.code_start(), repeat_token.code_end(),
                            "expects a `until` statement for this `repeat`, none found.")),

                        Some(mut statement) => {
                            self.parse(&mut statement, token_pool)?
                        },
                    }
                };

                let code_start = repeat_token.code_start();
                let code_end = until_statement.code_end();
                let line_number = repeat_token.line_number();

                let item = Element::create(vec![repeat_token, until_token], vec![repeat_statement, until_statement])?;

                elements.insert(0,CodeRef{
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed repeat-until");

                return Ok(true);
            }
        }

        Ok(false)
    }

    fn process_statement_do_end(&mut self, elements : &mut Vec<CodeElement>, token_pool : &mut Vec<CodeToken>) -> Result<bool, Error> {

        // check that the first token is a do, can't be any other way. do has to start the statement

        if elements.len() > 0 {
            if elements[0].i().matches_token(Token::Do) {
                self.get_tokens_until_token(elements, token_pool, Token::End)?;
                
                let insides : Vec<CodeToken> = {
                    let mut tokens : Vec<CodeToken> = Vec::new();

                    for token in elements.drain(1 .. elements.len() - 1) {
                        if let Some(t) = token.unwrap().consume_to_token() {
                            tokens.push(t);
                        }
                    }

                    tokens
                };
                
                let block = self.process(insides)?;

                let do_token = elements.remove(0);
                let end_token = elements.remove(0);

                let code_start = do_token.code_start();
                let code_end = end_token.code_end();
                let line_number = do_token.line_number();

                let item = Element::create(vec![do_token,end_token], vec![block])?;

                elements.insert(0,CodeRef{
                    item, code_start, code_end, line_number
                });
                
                #[cfg(feature = "dev-testing")]
                println!(".. processed do-end");
                
                return Ok(true);
            }
        }


        Ok(false)
    }

    fn process_last_statement(elements : &mut Vec<CodeElement>) -> Result<bool,Error> {
        //! - return [explist]
        //! - break

        // first checks if we have the single token versions of these things
        // this needs to start the section of elements, so we are going with that.
        if elements.len() == 1 {
            if let Some(token) = elements[0].i().get_token() {
                if token == &Token::Break || token == &Token::Return {

                    let removed_token = elements.remove(0);

                    let code_start = removed_token.code_start();
                    let code_end = removed_token.code_end();
                    let line_number = removed_token.line_number();

                    let item = Element::create(vec![removed_token], vec![])?;

                    elements.insert(0, CodeRef{
                        item, code_start, code_end, line_number
                    });

                    #[cfg(feature = "dev-testing")]
                    println!(".. processed last statement (0)");

                    return Ok(true)
                }
            }
        }

        // now we check for the return were we can return an expression
        if elements.len() == 2 {
            if let Some(token) = elements[0].i().get_token() {
                if token == &Token::Return && elements[1].i().is_exp_list() {
                    let return_token = elements.remove(0);
                    let exp_list = elements.remove(0);

                    let code_start = return_token.code_start();
                    let code_end = exp_list.code_end();
                    let line_number = return_token.line_number();

                    let item = Element::create(vec![return_token], vec![exp_list])?;

                    elements.insert(0, CodeRef{
                        item, code_start, code_end, line_number
                    });
                    
                    #[cfg(feature = "dev-testing")]
                    println!(".. processed last statement");
                    
                    return Ok(true)
                }
            }
        }

        Ok(false)
    }

    fn process_table_constructor(&mut self, elements : &mut Vec<CodeElement>, token_pool : &mut Vec<CodeToken>) -> Result<bool, Error> {

        //! tableconstructor ::= `{´ [fieldlist] `}´

        // first we check for the `{`, if we find that we see if we have a `}`, if not
        // then we ask for all the tokens until the `}`. **we need to worry about nesting**
        for i in 0 .. elements.len() {
            if elements[i].i().matches_token(Token::LeftMoustache) {
                self.get_tokens_until_token(elements, token_pool, Token::RightMoustache)?;

                // we are going to pull out the insides for processing
                let mut insides : Vec<CodeElement> = elements.drain(i+1 .. elements.len()-1).collect();
                let mut pool : Vec<CodeToken> = Vec::new();
                // and we parse the insides separately
                let processed_insides = self.parse(&mut insides, &mut pool)?;
                // now we check it worked out 
                if !processed_insides.i().is_field_list() {
                    return Err(ParserError::unexpected(&self,
                        processed_insides.line_number(),
                        processed_insides.code_start(),
                        processed_insides.code_end(),
                        "expected a field list here."
                    ));
                }
                
                let left = elements.remove(i);
                let right = elements.remove(elements.len()-1);

                let code_start = left.code_start();
                let code_end = right.code_end();
                let line_number = left.line_number();

                let item = Element::create(vec![left, right], vec![processed_insides])?;

                elements.insert(i, CodeRef {
                    item, code_end, code_start, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed table const");

                return Ok(true);
            }
        }

        Ok(false)
    }
    
    fn process_field_list(elements : &mut Vec<CodeElement>) -> Result<bool,Error> {
        //! [ ] {exp `,´} exp

        // we need to start at every position and keep going until we hit 
        // something that doesn't fit the pattern anymore
        for i in 0 .. elements.len() {
            // the big loop, this is the starting character

            // if the first element isn't an expression we should just go
            // to the next iteration of the big loop
            if !elements[i].i().is_field() { continue; }

            let mut ending : usize = i; 

            for j in i+1 .. elements.len() {
                // this is the little loop, `i` is always the first character
                // or the start of the element phrase.  

                if (j-i) % 2 == 1 {
                    // this should be the alternated tokens, so in ourcase
                    // these should be commas or semicolon.
                    if let Some(token) = elements[j].i().get_token() {
                        if token != &Token::Comma && token != &Token::SemiColon { break; }
                    } else { break; /* this doesn't fit the pattern, we should leave */ }
                } else {
                    // this should be an expression
                    if !elements[j].i().is_field() { break; }
                }

                // move the ending because it matches.
                ending = j;
            }

            // now we check if we got anything useful.
            if i != ending {
                // we are checking that they are not the same, for other lists
                // we make sure the total number of items is odd, but this might
                // not be the case here because you can end a field list with a 
                // field list separator (, or ;)

                let mut exps : Vec<CodeElement> = Vec::new();

                for cc in 0 .. (ending-i+1) {
                    // we will iterate and remove all the components, and add the exp
                    // to the vec<>
 
                    let token = elements.remove(i);
                    if cc % 2 == 0 {
                        exps.push(token);
                    }
                }

                if exps.len() == 0 {
                    return Err(ParserError::general("field list was found, but parsed as empty??"));
                }

                let code_start : usize = exps[0].code_start();
                let line_number : usize = exps[0].line_number();
                let code_end : usize = exps[exps.len()-1].code_end();

                let item = Element::create(vec![], exps)?;

                elements.insert(i, CodeRef{
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed field list");

                return Ok(true);
            }
        }

        Ok(false)
    }

    fn process_field(elements : &mut Vec<CodeElement>) -> Result<bool,Error> {
        //! - `[´ exp `]´ `=´ exp
        //! - Name `=´ exp

        // checking for `[exp] = exp` 
        if elements.len() >= 5 { for i in 0 .. elements.len() - 4 {
            if elements[i].i().matches_token(Token::LeftBracket)
            && elements[i+1].i().is_exp()
            && elements[i+2].i().matches_token(Token::RightBracket)
            && elements[i+3].i().matches_token(Token::Equal)
            && elements[i+4].i().is_exp() {

                let left = elements.remove(i);
                let exp1 = elements.remove(i);
                let right = elements.remove(i);
                let op = elements.remove(i);
                let exp2 = elements.remove(i);

                let code_start = left.code_start();
                let code_end = exp2.code_end();
                let line_number = left.line_number();

                let item = Element::create(vec![left, right, op], vec![exp1, exp2])?;

                elements.insert(i, CodeRef{
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed field (5)");

                return Ok(true);
            }

        }}

        // checking for `Name = exp`
        if elements.len() >=3 { for i in 0 .. elements.len() - 2 {
            if elements[i].i().is_name()
            && elements[i+1].i().matches_token(Token::Equal)
            && elements[i+2].i().is_exp() {

                let name = elements.remove(i);
                let op = elements.remove(i);
                let exp = elements.remove(i);

                let code_start = name.code_start();
                let code_end = exp.code_end();
                let line_number = name.line_number();

                let item = Element::create(vec![op], vec![name, exp])?;

                elements.insert(i, CodeRef{
                    item, code_start, code_end, line_number
                });

                #[cfg(feature = "dev-testing")]
                println!(".. processed field (3)");

                return Ok(true);
            }
        }}


        Ok(false)
    }

    fn contains_token(list : &Vec<CodeElement>, token : Token) -> Option<usize> {
        //! checks the list if it contains a certain token

        for i in 0 .. list.len() {
            if list[i].i().matches_token(&token) { return Some(i); }
        }

        None
    }

}

#[cfg(test)]
mod tests {

    #[test]
    //#[ignore]
    pub fn test_failure() {
        use crate::scanner::Scanner;
        use crate::parser::Parser;
        
        let code : &str = r#"
        -- comment here
        x = 1 + - 2
        x,y = 2,3
        tabletest = { a = 1; b = 2; c = 2+3, ["return"] = 4; }
        bob[x] = 4
        jim = 3 + bob.x
        local bob,jim,mary = 1,2+3,(4*5)
        local bob

        while bob >= 4 do
            -- this is the inside of the loop
            bob = bob + 3
        end

        for i,v in pairs(table) do
            print(i,v)
        end

        for b = 1,2 do
            print(b)
        end

        for b = 1,20,2 do
            print(b)
        end

        function test2(a,s)
            -- this doesn't do anything either!!!
            print('test')
        end

        if bob == 1 then
            bob =  bob + 1
        elseif op == "a" then
            print("not here")
        elseif bob < 3 and bob > 3 then
            bob = "not possible"
        else
            error("this is a problem")
        end


        local aTestFunction = function (q,w,e,r,t,...)
            -- this function does some cool stuff,
            -- you should really check it out!!

            --[[ this is another comment
            just to make sure it all really works in the 
            end
            ]]--

            q = w + e + r
            y = 10

            return q,w,e,r,t,y
        end


        do
            x = 5
            repeat 
                x = x + 1
                x = x - 1
                x = x + 1
            until
                x >= 10
        end

        repeat
            x = x + 1
            do
                local bob = 10
                bob:hiddenFunction("other stuff")
                print("this is the bob" .. bob)
                add(1,2,3,4)
            end
        until x >= 10

        repeat
            y = y + 1
            local othervar = 10
            y = y + othervar
        until 
            y >= 100

        return (2 + 3)
        "#;

        let scanner = Scanner::from_str(code,Some("testfile.lua")).unwrap();
        let parser = Parser::from_scanner(scanner);

        match parser {
            Ok(_) => { },
            Err(error) => { println!("{}",error); assert!(false); },
        }
    }

}