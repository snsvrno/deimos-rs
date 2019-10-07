use failure_derive::Fail;
use failure::Error;

use crate::scanner::Scanner;
use crate::error::{ 
    display_error_general, display_error, 
    codeinfo::{ CodeInformation, CodeInfo }};

#[derive(Debug,Fail)]
pub enum ScannerError {
    #[fail]
    GEN(String),        // general error
                        // 
    #[fail]
    UCS(CodeInfo), // UnterminatedCodeSegment
    
    #[fail]                        
    IC(CodeInfo), // Illegal Character / Symbol
    
    #[fail]                        
    NP(CodeInfo), // number parsing issue
}

impl std::fmt::Display for ScannerError {
    fn fmt(&self, f : &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //! doing it this way so i only need to have the format defined
        //! once, allowing all error types to have the same formatting.
        
        // determines whats the right name to use for the error
        match self {
            ScannerError::GEN(desc) => display_error_general(f, &desc),
            ScannerError::UCS(code_info) => display_error(f, "unterminated code segment", &code_info),
            ScannerError::IC(code_info) => display_error(f, "unknown character", &code_info),
            ScannerError::NP(code_info) => display_error(f, "number parsing", &code_info),
        }
    }
}


impl ScannerError {
    pub fn general(description : &str) -> Error {
        //! creates a general error

        ScannerError::GEN(description.to_string()).into()
    }

    pub fn unterminated_code_segment(scanner : &Scanner, offset : usize, span : usize, description : &str) -> Error {
        //! creates an unterminated code segment error,
        //! 
        //! - offset : how many characters to the left should we offset, assumes
        //!            that we consumed some characters so we might need to back
        //!            track
        //! - span : the length of the part to highlight

        let mut code_info = CodeInformation::into_codeinfo(scanner);

        code_info.description = description.to_string();
        code_info.span = span;
        code_info.cursor_pos = code_info.cursor_pos - offset;
        
        ScannerError::UCS(code_info).into()
    }

    pub fn illegal_character(scanner : &Scanner, description : Option<&str>) -> Error {
        //! creates an illegal or unknown character error,

        let mut code_info = CodeInformation::into_codeinfo(scanner);

        if let Some(description) = description {
            code_info.description = description.to_string();
        } else {
            code_info.description = "unknown character".to_string();
        }

        ScannerError::IC(code_info).into()
    }

    pub fn number_parsing(scanner : &Scanner, span : usize, description : &str) -> Error {
        //! creates an illegal or unknown character error,

        let mut code_info = CodeInformation::into_codeinfo(scanner);

        code_info.description = description.to_string();
        code_info.span = span;

        ScannerError::NP(code_info).into()
    }
}