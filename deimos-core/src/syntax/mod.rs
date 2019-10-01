mod element; pub use element::SyntaxElement;

pub mod exp;
pub mod explist;
pub mod statement;
pub mod var;
pub mod varlist;

use crate::codewrap::CodeWrap;

pub enum SyntaxResult {
    Done,
    Wrap(CodeWrap<SyntaxElement>),
    None,
    More,
    Error(usize,usize,String) // code_start, code_end, description
}

pub fn final_compress(elements : &mut Vec<CodeWrap<SyntaxElement>>) -> SyntaxResult {
	//! takes a list of SyntaxElements and attempts to make them into
	//! a block, using the block and chunk
	//!
	//! chunk ::= {stat [`;´]} [laststat [`;´]]
	//! block ::= chunk


	if elements.len() == 0 {
		return SyntaxResult::Error(0,0,"No elements found to compress?".to_string());
	}

	// check if each element before the last is a statement, if it isn't then we need to error
	for i in 0 .. elements.len() - 1 {
		if !elements[i].item().is_statement() {

			let CodeWrap::CodeWrap(_, start, end) = elements[i];
			return SyntaxResult::Error(start,end,"Line isn't a statement, but must be one".to_string());
		}
	}

	// now we check the last element to see if its a statement or last_statement
	if !(elements[elements.len()-1].item().is_statement() || elements[elements.len()-1].item().is_last_statement()) {

		let CodeWrap::CodeWrap(_, start, end) = elements[elements.len()-1];
		return SyntaxResult::Error(start,end,"Line isn't a statement, but must be one".to_string());
	}

	// if we are here that means every line is a valid statement, so we can make the chunk => block
	let CodeWrap::CodeWrap(_, start, _) = elements[0];
	let CodeWrap::CodeWrap(_, _ , end) = elements[elements.len()-1];
	let statement_list : Vec<Box<SyntaxElement>> = {
		// we are removing the elements of the `elements` array and 
		// putting it inside of the new `statement_list` array. we 
		// don't care about the start and stop for the code because we
		// saved it above.
		let mut new_list : Vec<Box<SyntaxElement>> = Vec::new();

		loop {
			if elements.len() == 0 { break; }
			let CodeWrap::CodeWrap(element, _, _) = elements.remove(0);
			new_list.push(Box::new(element));
		}

		new_list
	};

	let chunk = SyntaxElement::Chunk(statement_list);
	let block = SyntaxElement::Block(Box::new(chunk));

	SyntaxResult::Wrap(CodeWrap::CodeWrap(block, start, end))
}