/// the lua print function, taken from https://www.lua.org/manual/5.1/manual.html#5.1
/// 
/// # assert (v [, message])
/// Issues an error when the value of its argument v is false
/// (i.e., nil or false); otherwise, returns all its arguments.
/// message is an error message; when absent, it defaults to "assertion failed!" 

use enums::eresult::EResult;
use enums::value::Value;

pub fn assert() -> EResult {
    EResult::Value(Value::Bool(true))
}