use crate::parser::Parser;
use failure_derive::Fail;
use failure::Error;

use crate::error::{ 
    display_error_general, display_error, 
    codeinfo::{ CodeInformation, CodeInfo }};

#[derive(Debug,Fail)]
pub enum ParserError {
    #[fail]
    GEN(String),        // general error

    #[fail]
    NOSTATEMENT(CodeInfo),      // can't reduce the thing into a statement

    #[fail]
    EXPECT(CodeInfo),       // got something unexpected

    #[fail]
    UNTERMINATED(CodeInfo), // can't find the end of what i'm looking for
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f : &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //! doing it this way so i only need to have the format defined
        //! once, allowing all error types to have the same formatting.
        
        // determines whats the right name to use for the error
        match self {
            ParserError::GEN(desc) => display_error_general(f, &desc),
            ParserError::NOSTATEMENT(info) => display_error(f, "Not a Statement", &info),
            ParserError::EXPECT(info) => display_error(f, "Unexpected Element", &info),
            ParserError::UNTERMINATED(info) => display_error(f, "Unterminated Phrase", &info),
        }
    }
}


impl ParserError {
    pub fn general(description : &str) -> Error {
        //! creates a general error

        ParserError::GEN(description.to_string()).into()
    }

    pub fn not_a_statement(parser : &Parser, line : usize, start : usize, end : usize) -> Error {
        //! creates an 'cant reduce to statement' error

        let mut code_info = CodeInformation::into_codeinfo(parser);

        code_info.description = String::from("can't reduce to a single statement");
        code_info.span = end - start;
        code_info.cursor_pos = start;
        code_info.line_number = line;
        
        ParserError::NOSTATEMENT(code_info).into()
    }

    pub fn unexpected(parser : &Parser, line : usize, start : usize, end : usize, description : &str) -> Error {
        //! creates an 'cant reduce to statement' error

        let mut code_info = CodeInformation::into_codeinfo(parser);

        code_info.description = description.to_string();
        code_info.span = end - start;
        code_info.cursor_pos = start;
        code_info.line_number = line;
        
        ParserError::NOSTATEMENT(code_info).into()
    }

    pub fn unterminated(parser : &Parser, line : usize, start : usize, end : usize, description : &str) -> Error {
        //! creates an 'cant reduce to statement' error

        let mut code_info = CodeInformation::into_codeinfo(parser);

        code_info.description = description.to_string();
        code_info.span = end - start;
        code_info.cursor_pos = start;
        code_info.line_number = line;
        
        ParserError::NOSTATEMENT(code_info).into()
    }
}