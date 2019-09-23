use failure_derive::Fail;
use failure::Error;
use std::fmt;

use crate::parser::Parser;
use crate::error::{ CodeInfo, error_display };

#[derive(Debug,Fail)]
pub enum SyntaxError {
    #[fail]
    GEN { info : CodeInfo }, // UnterminatedCodeSegment
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        //! doing it this way so i only need to have the format defined
        //! once, allowing all error types to have the same formatting.
        
        // determines whats the right name to use for the error
        let (error_type, info) = match self {
            SyntaxError::GEN { info } => ("General Syntax Error", info),
        };

        error_display(f, info, "syntax", error_type)
    }
}

#[allow(dead_code)]
impl SyntaxError {
    pub fn general(start : usize, end : usize, description : &str) -> Error {
        //! creates an general syntax error,

        let info = CodeInfo {
            raw_code : String::new(),
            pos : start,
            file_name : String::new(),
            line_number : 0,
            span : end - start + 1,
            description : description.to_string(),
        };
    
        SyntaxError::GEN { info }.into()
    }
}