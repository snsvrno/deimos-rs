//! the element is an organizational object that is designed to make 
//! the parsing structures and reading easier (by removing the tons of
//! recursive stuff that goes own when transforming var => varlist => 
//! explist => ....).
//!
//! the concept is there is a left and a right
//! ( left | right )
//! where the left is the `identifiers` and the right is the `elements`.
//!
//! the left is used to determine what kind of object we are looking at,
//! so for the following code: `x + 1` the left would be `+`. the right 
//! is the arguements for the left, so for the above example the right 
//! would be `x, 1`. so to show it all at once: `(+ | x, 1)`. this is 
//! suppose to also work for more complex things like an `if .. then .. end`
//! loop. for example given this `if y >= 1 then x = x + 1; y = y + 1 end`
//! you would get the following `(if,then,end | (>= | y, 1), (= | x,(+ | x,1)),
//! (= | y,(+ | y,1)))` and this should describe the block / object, and 
//! make it easier than getting crazy recursive structures. `element`
//! will have `impl` functions to check that a `(if,then,end | _)` is a
//! if loop (or a statement, or whatever) so that it should be easy to work with
//!
//! the intent is to have an object like `(bob|)`, which shows its a variable name,
//! and then check `.is_exp()`, `.is_var()`, `.is_exp_list()`, `.is_var_list()`,
//! etc .. and have them all pass. if we wrapped `bob` with `Exp(_)` then he might
//! not be a var anymore, and some of the pattern matching would not work anymore.

use crate::token::{CodeToken, Token};
use crate::coderef::CodeRef;
use crate::error::parser::ParserError;

use failure::Error;

pub type CodeElement = CodeRef<Element>;

#[derive(Debug)]
pub struct Element {
	identifiers : Vec<CodeToken>,
	elements : Vec<Box<CodeElement>>,
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    	match self.is_token() {
        	false => write!(f, "<{} | {}>", list_to_string(&self.identifiers), list_to_string(&self.elements)),
        	true => write!(f, "<{}>", self.identifiers[0]),
        }
    }
}

impl Element {
	pub fn new() -> Element {
		Element {
			identifiers : Vec::new(),
			elements : Vec::new(),
		}
	}

	pub fn create(mut ids : Vec<CodeElement>, mut els : Vec<CodeElement>) -> Result<Element,Error> {
		//! creates an element from two vectors of other elements. this is a quality
		//! of life function to make reading and working with these elements easier.

		let mut identifiers : Vec<CodeToken> = Vec::new();
		while ids.len() > 0 {
			let small_element = ids.remove(0);
			if let Some(token) = small_element.unwrap().consume_to_token() {
				identifiers.push(token);
			} else {
				return Err(ParserError::general("error making element::Element, element isn't a token"))
			}
		}
		
		let mut elements : Vec<Box<CodeElement>> = Vec::new();
		while els.len() > 0 {
			elements.push(Box::new(els.remove(0)));
		}

		Ok(Element { identifiers, elements })

	}

	pub fn codeelement_from_token(token : CodeToken) -> CodeElement {
		//! just wraps the token with an element (makes them both code variants)
        
        let CodeRef::CodeRef { item : _ , ref code_start,  ref code_end, ref line_number } = &token;

        let cs = *code_start;
        let ce = *code_end;
        let ln = *line_number;
        
        let new_item = Element {
        	identifiers : vec![token],
        	elements : Vec::new(),
        };

        CodeRef::CodeRef {
            item : new_item,
            code_end : ce,
            code_start : cs,
            line_number : ln,
        }

	}

	pub fn add_to_elements(&mut self, element : CodeElement) {
		self.elements.push(Box::new(element));
	}

	// THE SYNTAX FUNCTIONS ////////////////////////////////////////
	// this section of code takes the lua specification (duplicated in LUA-SPEC.md)
	// and implements checks on the element struct.
	
	pub fn is_block(&self) -> bool {
		self.is_chunk()
	}

	pub fn is_chunk(&self) -> bool {

        let Element { ref identifiers, ref elements } = self;

        if identifiers.len() == 0 && elements.len() > 0 {
        	// checks that all but the elements are statements
        	for i in 0 .. elements.len()-1 {
        		if !elements[i].i().is_statement() { return false; }
        	}

        	// checks if the last elmenet is a last_statement or a statemnet 
        	if elements[elements.len()-1].i().is_statement() || elements[elements.len()-1].i().is_last_statement() {
        		return true;
        	}
        }

        false
	}

	pub fn is_statement(&self) -> bool {
		//! [x] varlist `=´ explist  
        //! [x] functioncall  
        //! [x] do block end  
        //! [x] while exp do block end  
        //! [x] repeat block until exp  
        //! [x] if exp then block {elseif exp then block} [else block] end  
        //! [x] for Name `=´ exp `,´ exp [`,´ exp] do block end  
        //! [x] for namelist in explist do block end  
        //! [x] function funcname funcbody  
        //! [x] local function Name funcbody  
        //! [x] local namelist [`=´ explist] 

        let Element { ref identifiers, ref elements } = self;

        // varlist `=´ explist  
       	if identifiers.len() == 1  && elements.len() == 2 {
        	if identifiers[0] == Token::Equal {
        		if elements[0].i().is_var_list() && elements[1].i().is_exp_list() {
        			return true;
        		}
        	}
        }

        if self.is_function_call() { return true; }
        if self.is_comment() { return true; } // not really a statement, but i want to keep this in the code.

        // do block end  
        if identifiers.len() == 2 && elements.len() == 1 {
        	if identifiers[0] == Token::Do 
        	&& identifiers[1] == Token::End 
        	&& elements[0].i().is_block() {
        		return true;
        	}
        }

        // while exp do block end  
        if identifiers.len() == 3 && elements.len() == 2 {
        	if identifiers[0] == Token::While 
        	&& identifiers[1] == Token::Do 
        	&& identifiers[2] == Token::End 
        	&& elements[0].i().is_exp() 
        	&& elements[1].i().is_block() {
    			return true;
    		}
        }

        // repeat block until exp 
        if identifiers.len() == 2 && elements.len() == 2 {
        	if identifiers[0] == Token::Repeat 
        	&& identifiers[1] == Token::Until 
        	&& elements[0].i().is_block() 
        	&& elements[1].i().is_exp(){
        		return true;
        	}
        }

        // if exp then block {elseif exp then block} [else block] end  
		if identifiers.len() >= 3 && elements.len() >= 2 {
        	if identifiers[0] == Token::If
        	&& identifiers[1] == Token::Then
        	&& identifiers[identifiers.len()-1] == Token::End
        	&& elements[0].i().is_exp()
        	&& elements[elements.len()-1].i().is_block() {
        		return true;
        		// TODO should we check more here?
        	}
        }

        // for Name `=´ exp `,´ exp [`,´ exp] do block end  
        if identifiers.len() >= 4 && elements.len() >= 4 {
        	if identifiers[0] == Token::For
        	&& identifiers[1] == Token::Equal
        	&& identifiers[2] == Token::Comma
        	&& identifiers[identifiers.len() - 1] == Token::End
        	&& identifiers[identifiers.len() - 2] == Token::Do
        	&& elements[0].i().is_name()
        	&& elements[1].i().is_exp()
        	&& elements[2].i().is_exp()
        	&& elements[elements.len()-1].i().is_block() {
        		return true;
        	}

        }

        // for namelist in explist do block end  
        if identifiers.len() == 4 && elements.len() == 3 {
        	if identifiers[0] == Token::For
        	&& identifiers[1] == Token::In
        	&& identifiers[2] == Token::Do
        	&& identifiers[2] == Token::End
        	&& elements[0].i().is_name_list()
        	&& elements[1].i().is_exp_list()
        	&& elements[2].i().is_block() {
        		return true;
        	}
        }

		// function funcname funcbody  
		if identifiers.len() == 1 && elements.len() == 2 {
			if identifiers[0] == Token::Function
			&& elements[0].i().is_func_name()
			&& elements[1].i().is_func_body() {
				return true;
			}
		}

        // local function Name funcbody  
        if identifiers.len() == 2 && elements.len() == 2 {
			if identifiers[0] == Token::Local
			&& identifiers[1] == Token::Function
			&& elements[0].i().is_func_name()
			&& elements[1].i().is_func_body() {
				return true;
			}
        }

        // local namelist `=´ explist
        if identifiers.len() == 2 && elements.len() == 2 {
            if identifiers[0] == Token::Local
            && identifiers[1] == Token::Equal
            && elements[0].i().is_name_list()
            && elements[1].i().is_exp_list() {
                return true;
            }
        }

        // local namelist
        if identifiers.len() == 1 && elements.len() == 1 {
            if identifiers[0] == Token::Local
            && elements[0].i().is_name_list() {
                return true;
            }
        }

        false
	}

	pub fn is_last_statement(&self) -> bool {
		//! [x] return [explist]
		//! [x] break
 
        if let Some(token) = self.get_token() {
        	match token.item() {
        		Token::Return | Token::Break => return true,
        		_ => { },
        	}
        }

        let Element { ref identifiers, ref elements } = self;
        if identifiers.len() == 1 && elements.len() == 1 {
    		if identifiers[0] == Token::Return && elements[0].i().is_exp_list() {
    			return true;
    		}
        }

        false
	}

	pub fn is_func_name(&self) -> bool {
		//! [x] Name {`.´ Name} [`:´ Name]

		if self.is_name() { return true; }

        let Element { ref identifiers, ref elements } = self;

        if identifiers.len() > 0 && elements.len() == identifiers.len() + 1 {
        	
        	// checks if all the elements are names
        	let elements_ok : bool = {
        		let mut names : usize = 0;
        		for e in elements.iter() {
        			if e.i().is_name() { names += 1; }
        		}
        		names == identifiers.len()
        	};

        	// checks the identifiers
			let identifiers_ok : bool = {
				let mut passed_ids : usize = 0;

				for i in 0 .. identifiers.len() - 1 {
					if identifiers[i] == Token::Period { passed_ids += 1; }
				}

				if identifiers[identifiers.len()-1] == Token::Period || identifiers[identifiers.len()-1] == Token::Colon {
					passed_ids += 1;
				}

				passed_ids == identifiers.len()
			};

			// the final check
			if identifiers_ok && elements_ok { return true; }     	
        }

		false
	}

	pub fn is_var(&self) -> bool {
		//! [x] Name 
		//! [x] prefixexp `[´ exp `]´ 
		//! [X] prefixexp `.´ Name 

		if let Some(token) = self.get_token() {
			if token.i().is_name() { return true; }
		}

        let Element { ref identifiers, ref elements } = self;

        if identifiers.len() == 2 && elements.len() == 2 {
        	if identifiers[0] == Token::LeftBracket
			&& identifiers[1] == Token::RightBracket
			&& elements[0].i().is_prefix_exp()
			&& elements[1].i().is_exp() {
				return true;
			}
        }

        if identifiers.len() == 1 && elements.len() == 2 {
        	if identifiers[0] == Token::Period
			&& elements[0].i().is_prefix_exp()
			&& elements[1].i().is_name() {
				return true;
			}
        }

		false
	}

	pub fn is_var_list(&self) -> bool {
		
		if self.is_var() { return true; }


        let Element { ref identifiers, ref elements } = self;
        if identifiers.len() == 0 && elements.len() > 0 {
        	for i in 0 .. elements.len() {
        		if !elements[i].i().is_var() { return false; }
        	}
        	return true;
        }

        false
	}

	pub fn is_name(&self) -> bool {

        if let Some(token) = self.get_token() {
        	match token.item() {
        		Token::Identifier(_) => return true,
        		_ => { },
        	}
        }

		false
	}

	pub fn is_name_list(&self) -> bool {
		
		if self.is_name() { return true; }


        let Element { ref identifiers, ref elements } = self;
        if identifiers.len() == 0 && elements.len() > 0 {
        	for i in 0 .. elements.len() {
        		if !elements[i].i().is_name() { return false; }
        	}
        	return true;
        }

        false
	}

	pub fn is_exp(&self) -> bool {
		//! checks if the element matches the following patterns.
		//!
		//! [x] nil		
		//! [x] false
		//! [x] true
		//! [x] Number
		//! [x] String 
		//! [x] `...´ 		
		//! [x] function 
        //! [x] prefixexp
        //! [x] tableconstructor
        //! [x] exp binop exp
        //! [x] unop exp 

        // lets check all the single token options
        if let Some(token) = self.get_token() {
        	match token.item() {
        		Token::Nil | Token::False | Token::True |
        		Token::Number(_) | Token::String(_) | Token::TriplePeriod |
        		Token::MultiLineString(_) => return true,
        		_ => { },
        	}
        }

        if self.is_prefix_exp() { return true; }
        if self.is_function() { return true; }
        if self.is_table_constructor() { return true; }

        let Element { ref identifiers, ref elements } = self;

        // lets check for binop
        if identifiers.len() == 1 && elements.len() == 2 {
        	if identifiers[0].i().is_binop() && elements[0].i().is_exp() && elements[0].i().is_exp() {
        		return true;
        	}
        }

        // check for an unop
        if identifiers.len() == 1 && elements.len() == 1 {
        	if identifiers[0].i().is_unop() && elements[0].i().is_exp() {
        		return true;
        	}
        }

        // doesn't match anything
        false
	}

	pub fn is_exp_list(&self) -> bool {
		
		if self.is_exp() { return true; }


        let Element { ref identifiers, ref elements } = self;
        if identifiers.len() == 0 && elements.len() > 0 {
        	for i in 0 .. elements.len() {
        		if !elements[i].i().is_exp() { return false; }
        	}
        	return true;
        }

        false
	}

	pub fn is_prefix_exp(&self) -> bool {
		//! checks if this is a prefix expression
		//! 
		//! [x] var 
		//! [x] functioncall 
		//! [x] `(´ exp `)´

		if self.is_var() { return true; }
		if self.is_function_call() { return true; }

        let Element { ref identifiers, ref elements } = self;

        // checking for an expression surrounded by '(' ')'
        if identifiers.len() == 2 && elements.len() == 1 {
        	if identifiers[0].i() == Token::LeftParen && identifiers[1].i() == Token::RightParen && elements[0].i().is_exp() {
        		return true;
        	}
        }

        false
	}

	pub fn is_function_call(&self) -> bool {
		//! [x] prefixexp args
		//! [x] prefixexp `:´ Name args 
        
        let Element { ref identifiers, ref elements } = self;

        if identifiers.len() == 0 && elements.len() == 2 {
        	if elements[0].i().is_prefix_exp()
        	&& elements[1].i().is_args() {
        		return true;
        	}
        }

        if identifiers.len() == 1 && elements.len() == 3 {
        	if identifiers[0] == Token::Colon
        	&& elements[0].i().is_prefix_exp()
        	&& elements[1].i().is_name() 
        	&& elements[1].i().is_args() {
        		return true;
        	}
        }

        false
	}

	pub fn is_args(&self) -> bool {
		//! [ ] `(´ [explist] `)´ 
		//! [ ] tableconstructor
		//! [ ] String 

		if self.is_string() { return true; }
		if self.is_table_constructor() { return true; }

        let Element { ref identifiers, ref elements } = self;

        if identifiers.len() == 2 {
        	if identifiers[0] == Token::LeftParen
        	&& identifiers[1] == Token::RightParen {
        		// makes sure the elements (if len == 1) 
        		// is a explist
        		if elements.len() == 0 { return true; }
        		else if elements.len() == 1 { return elements[0].i().is_exp_list(); }
        	}	
        }

        false
	}

	pub fn is_function(&self) -> bool {
		//! [x] function funcbody
		
		let Element { ref identifiers, ref elements } = self;

		if identifiers.len() == 1 && elements.len() == 1 {
			if identifiers[0] == Token::Function
			&& elements[0].i().is_func_body() {
				return true;
			}
		}

		false
	}

	pub fn is_func_body(&self) -> bool {
		//! [x] `(´ [parlist] `)´ block end
		
		let Element { ref identifiers, ref elements } = self;

		if identifiers.len() == 3 && elements.len() == 1 {
			if identifiers[0] == Token::LeftParen
			&& identifiers[1] == Token::RightParen
			&& identifiers[2] == Token::End
			&& elements[0].i().is_block() {
				return true;
			}
		}

		if identifiers.len() == 3 && elements.len() == 2 {
			if identifiers[0] == Token::LeftParen
			&& identifiers[1] == Token::RightParen
			&& identifiers[2] == Token::End
			&& elements[0].i().is_par_list() 
			&& elements[1].i().is_block() {
				return true;
			}
		}

		false
	}

	pub fn is_par_list(&self) -> bool {
		//! [x] namelist [`,´ `...´]
		//! [x] `...´

        if let Some(token) = self.get_token() {
        	match token.item() {
        		Token::TriplePeriod => return true,
        		_ => { },
        	}
        }

        if self.is_name_list() { return true; }

		let Element { ref identifiers, ref elements } = self;

		if identifiers.len() == 0 && elements.len() == 2 {
			if elements[0].i().is_name_list() {
				if let Some(token) = elements[1].i().get_token() {
					if token.i() == Token::TriplePeriod { return true; }
				}
			}
		}

		false
	}

    pub fn is_comment(&self) -> bool {
        //! - -- something here

        if let Some(token) = self.get_token() {
            match token.item() {
                Token::Comment(_) => return true,
                _ => { },
            }
        }

        false
    }

	pub fn is_table_constructor(&self) -> bool {
		//! [x] `{´ [fieldlist] `}´

		let Element { ref identifiers, ref elements } = self;
		
		if identifiers.len() == 2 && elements.len() == 0 {
			if identifiers[0] == Token::LeftMoustache
			&& identifiers[1] == Token::RightMoustache {
				return true;
			}
		} 

		if identifiers.len() == 2 && elements.len() == 1 {
			if identifiers[0] == Token::LeftMoustache
			&& identifiers[1] == Token::RightMoustache 
			&& elements[0].i().is_field_list() {
				return true;
			}
		} 

		false
	}

	pub fn is_field_list(&self) -> bool {

		if self.is_field() { return true; }

        let Element { ref identifiers, ref elements } = self;

        if identifiers.len() == 0 && elements.len() > 0 {
        	for i in 0 .. elements.len() {
        		if !elements[i].i().is_field() { return false; }
        	}
        	return true;
        }

        false
	}

	pub fn is_field(&self) -> bool {
		//! [x] `[´ exp `]´ `=´ exp
		//! [x] Name `=´ exp
		//! [x] exp

		if self.is_exp() { return true; }

        let Element { ref identifiers, ref elements } = self;

        if identifiers.len() == 1 && elements.len() == 2 {
			if identifiers[0] == Token::Equal 
			&& elements[0].i().is_name()
			&& elements[1].i().is_exp() {
				return true;
			}
        }

        if identifiers.len() == 3 && elements.len() == 2 {
			if identifiers[0] == Token::LeftBracket 
			&& identifiers[1] == Token::RightBracket 
			&& identifiers[2] == Token::Equal 
			&& elements[0].i().is_exp()
			&& elements[1].i().is_exp() {
				return true;
			}
        }

		false
	}

	// these aren't in the spec (explicitly) but useful checks that make things 
	// easier.

	pub fn is_token(&self) -> bool {
        //! checks if this element is actually a wrapped token,

        let Element { ref identifiers, ref elements } = self;

        if identifiers.len() == 1 && elements.len() == 0 { 
        	true
        } else {
        	false
        }
	}

    pub fn matches_token(&self, token : Token) -> bool {
        //! used to check if this object is a token of type `token`

        if let Some(inside_token) = self.get_token() {
            return inside_token == &token; 
        }

        false
    }

	pub fn get_token<'a>(&'a self) -> Option<&'a CodeToken> {
		//! checks if this item is a token and then returns that
		//! token
		
		match self.is_token() {
			false => None,
			true => {
        		let Element { ref identifiers, elements : _ } = self;
				Some(&identifiers[0])
			}
		}
	}

	pub fn consume_to_token(self) ->  Option<CodeToken> {
		//! checks if this item is a token and then consumes it
		//! i think it consumes it either way though ...

		match self.is_token() {
			false => None,
			true => {
				let Element { mut identifiers, elements : _ } = self;
				Some(identifiers.remove(0))
			}
		}
	}

	pub fn is_unop_token(&self) -> bool {
		if let Some(token) = self.get_token() {
			return token.item().is_unop();
		}
		false
	}

	pub fn is_binop_token(&self) -> bool {
		if let Some(token) = self.get_token() {
			return token.item().is_binop();
		}
		false
	}

	pub fn is_string(&self) -> bool {
		if let Some(token) = self.get_token() {
			match token.i() {
				Token::String(_) | Token::MultiLineString(_) => return true,
				_ => { }
			}
		}
		false		
	}

	// PRIVATE FUNCTIONS
}

fn list_to_string<T>(list : &Vec<T>) -> String where T : std::fmt::Display {
	let mut string : String = String::new();

	for i in list.iter() {
		string = format!("{}{}, ", string, i);
	}

	string
}