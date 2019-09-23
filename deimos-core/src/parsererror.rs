use failure_derive::Fail;
use failure::Error;
use std::fmt;

use crate::parser::Parser;
use crate::error::{ CodeInfo, error_display };

#[derive(Debug,Fail)]
pub enum ParserError {
    #[fail]
    UCS { info : CodeInfo }, // UnterminatedCodeSegment
}

impl fmt::Display for ParserError {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        //! doing it this way so i only need to have the format defined
        //! once, allowing all error types to have the same formatting.
        
        // determines whats the right name to use for the error
        let (error_type, info) = match self {
            ParserError::UCS { info } => ("Unterminated Code Segment", info),
        };

        error_display(f, info, "parser", error_type)
    }
}

#[allow(dead_code)]
impl ParserError {
    pub fn unterminated_code_segment(parser : &Parser, offset : usize, span : usize, description : &str) -> Error {
        //! creates an unterminated code segment error,
        //! 
        //! - offset : how many characters to the left should we offset, assumes
        //!            that we consumed some characters so we might need to back
        //!            track
        //! - span : the length of the part to highlight
        
        let mut info = CodeInfo::from(parser);
        info.pos -= offset;
        info.span = span;
        info.description = description.to_string();

        ParserError::UCS { info }.into()
    }
}