use enums::value::Value;
use failure::Error;

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum EResult{
    Value(Value),
    Assignment(String,Value,bool), // bool means is_local
}

impl EResult {
    pub fn unwrap_value(self) -> Result<Value,Error> {
        match self {
            EResult::Value(value) => Ok(value),
            _ => Err(format_err!("EResult isn't a value, can't unwrap."))
        }
    }
}