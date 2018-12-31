extern crate lua_interpreter;

#[test]
fn basic_parse() {
    match lua_interpreter::parse("4+3") {
        Ok(result) => assert_eq!(7,result),
        Err(error) => { println!("ERROR: {}",error); assert!(false); }
    }
    match lua_interpreter::parse("10-7") {
        Ok(result) => assert_eq!(3,result),
        Err(error) => { println!("ERROR: {}",error); assert!(false); }
    }
}
