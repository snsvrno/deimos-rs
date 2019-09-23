use std::fmt;

use crate::scanner::Scanner;
use crate::parser::Parser;
use crate::token::Token;

pub trait CodeInformation {
    // a trait that dictates the required information
    // for correct error reporting
     
    fn raw_code(&self) -> String { String::new() }
    fn cursor_pos(&self) -> usize { 0 }
    fn file_name(&self) -> String { String::new() }
    fn line_number(&self) -> usize { 0 }
    fn item_span(&self) -> usize { 1 }
    fn description(&self) -> String { String ::new() }

    fn gather_information(&self) -> CodeInfo {
        //! gets all the above informations together and creates
        //! the CodeInfo object

        CodeInfo {
            raw_code : CodeInformation::raw_code(self),
            pos : CodeInformation::cursor_pos(self),
            file_name : CodeInformation::file_name(self),
            line_number : CodeInformation::line_number(self),
            span : CodeInformation::item_span(self),
            description: CodeInformation::description(self),
        }
    }
    
}

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
    pub fn from<C:CodeInformation>(object : &C) -> CodeInfo {
        CodeInformation::gather_information(object)
    }
}

const LEFT_PADDING : &'static str = "     ";
const DESCRIPTION_WIDTH : usize = 40;

pub fn error_display(f : &mut fmt::Formatter<'_>, info : &CodeInfo, error_type : &str, error_name : &str) -> fmt::Result {

        // gets the front of the code to display, tries to isolate 
        // beginning of the line
        let mut distance_on_line = 0;
        let code_start = {
            let mut pos = info.pos;

            // loops backwards until it gets to a carrage return (newline)
            loop {
                
                // makes sure we don't access outside of the `raw_code`
                if pos == 0 {
                    break;
                }

                // checks if the character is a carrage return
                match &info.raw_code[pos - 1 .. pos] {
                    "\n" | "\r" => break,
                    _ => pos -= 1,
                }
            }

            // used to calculate the point on the line we are actually at
            // we take a snapshot of the start (above) and then subtract the 
            // real start of the code (below) to get the character number
            // where the code starts.
            distance_on_line = pos;

            // now we move forward to get rid of the leading tabs / spaces
            // (depending on what settings you use in the code)
            loop {
                let char =  &info.raw_code[pos .. pos + 1];

                if Token::is_whitespace(char) {
                    pos += 1;
                } else {
                    break;
                }
            }

            distance_on_line = pos - distance_on_line + 1;
            pos
        };

        // gets the end of the code display, makes sure we don't access
        // parts of the string that don't exist, but really we are looking
        // for the newline (carrage return)
        let code_end = {
            let mut pos = info.pos;

            // looking for the end of the string, or the carrage return
            loop {

                // makes sure we don't access outside of the `raw_code`
                if pos > info.raw_code.len() - 1 {
                    break;
                }

                // checks if the character is a carrage return
                let char =  &info.raw_code[pos .. pos + 1];
                if Token::is_eol(char) {
                    break;
                } else {
                    pos += 1;
                }
            }

            pos
        };

        // the segment of code that will be displayed.
        let code = info.raw_code[code_start .. code_end].to_string();

        // adds spacing behind the line number so the line lines up
        let line_number = {
            let mut number = String::new();
            number = format!(" {}",info.line_number);
            for _ in format!("{}",info.line_number).len() ..  4 {
                number = format!("{}{}",number," ");
            }
            number
        };

        // determines how many spaces to use in order to get the arrow
        // pointing at the right character
        let arrow = {
            let mut string = String::new();

            for _ in 0 .. info.span {
                string = format!("{}{}",string,"^");
            }

            for _ in code_start .. info.pos {
                string = format!("{}{}"," ",string);
            }

            // adds some extra text if there is any
            if info.description.len() > 0 {
                if info.description.len() <= DESCRIPTION_WIDTH {
                    string = format!("{} {}", string, info.description);
                } else {
                    // my attempt at doing a multiline thing
                    let header = {
                        let mut spacer = String::from("       ");
                        for i in (1 .. string.len()).rev() {
                            if &string[i .. i+1] == "\n" { break; } else { spacer = format!(" {}",spacer); }
                        }
                        spacer = format!("{}{}",LEFT_PADDING,spacer);
                        spacer
                    };

                    let mut count = 1;
                    while info.description.len() >  count * DESCRIPTION_WIDTH {
                        let section = &info.description[(count-1) * DESCRIPTION_WIDTH .. count * DESCRIPTION_WIDTH];
                        if count == 1 {
                            string = format!("{} {}\n", string, section);
                        } else {
                            string = format!("{}{} {}\n", string, header, section);
                        }
                        count += 1;
                    }

                    // need to add the last bit
                    string = format!("{}{} {}", string, header, &info.description[(count-1) * DESCRIPTION_WIDTH ..]);


                }
            }
        
            string
        };

        write!(f, "error: {error_type}\n  --> {file}:{line2}:{code_start}\n     |\n{line}|{padding}{code}\n     |{padding}{arrow}\n",
            error_type = error_type,
            file = info.file_name,
            line = line_number,
            code_start = distance_on_line,
            line2 = format!("{}",info.line_number),
            code = code,
            padding = LEFT_PADDING,
            arrow = arrow
        )
}