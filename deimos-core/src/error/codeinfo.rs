pub trait CodeInformation {
    // a trait that dictates the required information
    // for correct error reporting
     
    fn raw_code(&self) -> String { String::new() }
    fn cursor_pos(&self) -> usize { 0 }
    fn file_name(&self) -> String { String::new() }
    fn line_number(&self) -> usize { 0 }
    fn item_span(&self) -> usize { 1 }
    fn description(&self) -> String { String ::new() }    

    fn into_codeinfo(&self) -> CodeInfo {
        CodeInfo { 
            description : CodeInformation::description(self),
            span : CodeInformation::item_span(self),
            cursor_pos : CodeInformation::cursor_pos(self),
            raw_code : CodeInformation::raw_code(self),
            file_name : CodeInformation::file_name(self),
            line_number : CodeInformation::line_number(self),
        }
    }
}

#[derive(Debug)]
pub struct CodeInfo {
    // the text that goes next to the underlined section
    pub description : String,
    // the length of the underlined section in the code segment
    pub span : usize,
    // the position from the start of the code string that the error starts
    pub cursor_pos : usize, 
    // a string of the code
    pub raw_code : String,
    // the stream or filename
    pub file_name : String,
    // code line number
    pub line_number : usize,
}