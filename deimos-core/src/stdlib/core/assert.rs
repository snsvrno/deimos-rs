use crate::elements::Statement;
use crate::elements::Scope;

use failure::{Error,format_err};

pub fn assert(scope : &mut Scope, args : &Statement) -> Result<Statement, Error> {
    
    let test = args.as_list()[0].eval(scope)?;
    
    let result = if test.is_bool() {
        test.as_bool() 
    } else { 
        false
    };
        
    match result {
        true => Ok(Statement::Empty),
        false => {
            if args.as_list().len() > 1 {
                Err(format_err!("{}",args.as_list()[1].as_string()))
            } else {
                Err(format_err!("assertion failed!"))
            }
        }
    }
}
