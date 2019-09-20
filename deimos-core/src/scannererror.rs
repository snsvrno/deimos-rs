use failure_derive::Fail;
use failure::Error;
use std::fmt;

use crate::scanner::Scanner;

const CODE_LEADING_SPACE : usize = 10;
const CODE_AFTER_SPACE : usize = 10;
const LEFT_PADDING : &'static str = "     ";
const TRUNCATED_CODE : &'static str = " ... ";

#[derive(Debug)]
pub struct CodeInfo {
    pub raw_code : String,
    pub pos : usize,
    pub file_name : String,
    pub line_number : usize,
    pub span : usize,
    pub description : String,
}

impl CodeInfo {
    pub fn from(scanner : &Scanner) -> CodeInfo {
        CodeInfo {
            raw_code : scanner.raw_code.to_string(),
            pos : scanner.current_pos,
            file_name : scanner.file_name.to_string(),
            line_number : scanner.line_number,
            span : 1,
            description: String::new(),
        }
    }
}

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

        // gets the front of the code to display, makes sure we don't 
        // access parts that don't exist
        let code_start = {
            let pos = info.pos - CODE_LEADING_SPACE;
            pos
        };

        // gets the end of the code display, makes sure we don't access
        // parts of the string that don't exist
        let code_end = {
            let pos = info.pos + CODE_AFTER_SPACE;
            if pos > info.raw_code.len() - 1 { info.raw_code.len() - 1 } else { pos }
        };

        // will mark the beginning and the end of the block to note that 
        // there was other code that is removed, need to make this 
        // smarter when i actually have better code segments
        // 
        // TODO : need to make it know whats the beginning of the line
        let code = {
            let mut code = info.raw_code[code_start .. code_end].to_string();

            if code_start != 0 {
                code = format!("{}{}",TRUNCATED_CODE,code);
            };

            if code_end != info.raw_code.len() {
                code = format!("{}{}",code,TRUNCATED_CODE);
            };

            code
        };

        // adds spacing behind the line number so the line lines up
        let line_number = {
            let mut number = String::new();
            number = format!(" {}",info.line_number);
            for i in format!("{}",info.line_number).len() ..  4 {
                number = format!("{}{}",number," ");
            }
            number
        };

        // determines how many spaces to use in order to get the arrow
        // pointing at the right character
        let arrow = {
            let mut string = String::new();

            for i in 0 .. info.span {
                string = format!("{}{}",string,"^");
            }

            for i in code_start .. info.pos {
                string = format!("{}{}"," ",string);
            }

            // this part adds leading space because of the added
            // truncated code display
            // 
            // TODO : need to update this when i fix how
            //        it know what the beginning of the line is
            if code_start != 0 {
                for i in 0 .. TRUNCATED_CODE.len() {
                    string = format!("{}{}"," ",string);
                }
            }

            // adds some extra text if there is any
            if info.description.len() > 0 {
                string = format!("{} {}", string, info.description);  
            }
        
            string
        };

        write!(f, "error: {error_type}\n  --> {file}\n     |\n{line}|{padding}{code}\n     |{padding}{arrow}\n",
            error_type = error_type,
            file = info.file_name,
            line = line_number,
            code = code,
            padding = LEFT_PADDING,
            arrow = arrow
        )
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