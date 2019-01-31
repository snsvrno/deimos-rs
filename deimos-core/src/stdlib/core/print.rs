use crate::elements::Statement;
use crate::elements::Scope;

use failure::Error;

pub fn print(scope : &mut Scope, args : &Statement) -> Result<Statement,Error> {
    
    match args.eval(scope)?.as_user_output() {
        Some(output) => println!("{}",output),
        None => println!("ERROR : {}",args),
    }
    Ok(Statement::Empty)
}
