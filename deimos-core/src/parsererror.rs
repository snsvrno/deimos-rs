use failure_derive::Fail;
use failure::Error;
use std::fmt;

use crate::parser::Parser;
use crate::error::{ CodeInfo, error_display };

#[derive(Debug,Fail)]
pub enum ParserError {
    #[fail]
    GEN { info : CodeInfo }, // UnterminatedCodeSegment
}

impl fmt::Display for ParserError {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        //! doing it this way so i only need to have the format defined
        //! once, allowing all error types to have the same formatting.
        
        // determines whats the right name to use for the error
        let (error_type, info) = match self {
            ParserError::GEN { info } => ("General Parsing Error", info),
        };

        error_display(f, info, "parser", error_type)
    }
}

#[allow(dead_code)]
impl ParserError {
    pub fn general_error(parser : &Parser, start : usize, end : usize, description : &str) -> Error {
        //! creates a general parsing error,
        //! 
        //! - start : where in the code does this error start
        //! - end : where in the code does this error end

        let mut info = CodeInfo::from(parser);
        info.pos = start;
        info.span = end - start + 1;
        info.description = description.to_string();

        ParserError::GEN { info }.into()
    }
}