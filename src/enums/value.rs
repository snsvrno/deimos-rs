use failure::Error;

#[derive(Debug,Clone,Eq,PartialEq)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Nil
}

impl Value {
    pub fn add(value1 : &Value, value2 : &Value) -> Result<Value,Error> {
        match (value1,value2) {
            (Value::Int(i1),Value::Int(i2)) => Ok(Value::Int(i1+i2)),
            (_,_) => Err(format_err!("Cannot add {:?} and {:?}",value1,value2))
        }
    }

    pub fn subtract(value1 : &Value, value2 : &Value) -> Result<Value,Error> {
        match (value1,value2) {
            (Value::Int(i1),Value::Int(i2)) => Ok(Value::Int(i1-i2)),
            (_,_) => Err(format_err!("Cannot subtract {:?} and {:?}",value1,value2))
        }
    }
}