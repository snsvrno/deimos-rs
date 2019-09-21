use crate::statement::Statement;
use crate::error::CodeInformation;

pub struct Parser<'a> {
    pub file_name : String,
    pub raw_code : &'a str,
    pub statements : Vec<Statement>,
}

impl<'a> std::default::Default for Parser<'a> {
    fn default() -> Parser<'a> {
        Parser {
            raw_code : "",
            file_name : String::from("buffer"),
            statements : Vec::new(),
        }
    }
}

impl<'a> CodeInformation for Parser<'a> {
    fn raw_code(&self) -> String { self.raw_code.to_string() }
    fn file_name(&self) -> String { self.file_name.to_string() }
}
