use failure_derive::Fail;
use failure::Error;
use std::fmt;

use crate::scanner::Scanner;
use crate::error::{ CodeInfo, error_display };

#[derive(Debug,Fail)]
pub enum ScannerError {
    #[fail]
    UCS { info : CodeInfo }, // UnterminatedCodeSegment
    
    #[fail]                        
    IC { info : CodeInfo }, // Illegal Character / Symbol
    
    #[fail]                        
    NP { info : CodeInfo }, // number parsing issue
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        //! doing it this way so i only need to have the format defined
        //! once, allowing all error types to have the same formatting.
        
        // determines whats the right name to use for the error
        let (error_type, info) = match self {
            ScannerError::UCS { info } => ("Unterminated Code Segment", info),
            ScannerError::IC { info } => ("Illegal Character", info),
            ScannerError::NP { info } => ("Error Parsing Number", info),
        };

        error_display(f, info, "scanner", error_type)
    }
}

impl ScannerError {
    pub fn unterminated_code_segment(scanner : &Scanner, offset : usize, span : usize, description : &str) -> Error {
        //! creates an unterminated code segment error,
        //! 
        //! - offset : how many characters to the left should we offset, assumes
        //!            that we consumed some characters so we might need to back
        //!            track
        //! - span : the length of the part to highlight
        
        let mut info = CodeInfo::from(scanner);
        info.pos -= offset;
        info.span = span;
        info.description = description.to_string();

        ScannerError::UCS { info }.into()
    }

    pub fn illegal_character(scanner : &Scanner) -> Error {
        //! creates an illegal or unknown character error,
        //! 
        
        let mut info = CodeInfo::from(scanner);

        ScannerError::IC { info }.into()
    }

    pub fn number_parsing(scanner : &Scanner) -> Error {
        //! creates an illegal or unknown character error,
        //! 
        
        let mut info = CodeInfo::from(scanner);

        ScannerError::NP { info }.into()
    }
}