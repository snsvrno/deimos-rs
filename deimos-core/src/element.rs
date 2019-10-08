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
	
	pub fn is_var(&self) -> bool {
		//! checks if the element is a var
		//!
		//! [x] Name 
		//! [ ] prefixexp `[´ exp `]´ 
		//! [ ] prefixexp `.´ Name 

		if let Some(token) = self.get_token() {
			if token.i().is_name() { return true; }
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
		//! [ ] function 
        //! [x] prefixexp
        //! [ ] tableconstructor
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

	pub fn is_prefix_exp(&self) -> bool {
		//! checks if this is a prefix expression
		//! 
		//! [x] var 
		//! [ ] functioncall 
		//! [x] `(´ exp `)´

		if self.is_var() { return true; }

        let Element { ref identifiers, ref elements } = self;

        // checking for an expression surrounded by '(' ')'
        if identifiers.len() == 2 && elements.len() == 1 {
        	if identifiers[0].i() == Token::LeftParen && identifiers[1].i() == Token::RightParen && elements[0].i().is_exp() {
        		return true;
        	}
        }

        false
	}

	pub fn is_token(&self) -> bool {
        //! checks if this element is actually a wrapped token,

        let Element { ref identifiers, ref elements } = self;

        if identifiers.len() == 1 && elements.len() == 0 { 
        	true
        } else {
        	false
        }
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

	// PRIVATE FUNCTIONS
}

fn list_to_string<T>(list : &Vec<T>) -> String where T : std::fmt::Display {
	let mut string : String = String::new();

	for i in list.iter() {
		string = format!("{}{}, ", string, i);
	}

	string
}