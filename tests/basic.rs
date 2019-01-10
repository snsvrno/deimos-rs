extern crate lua_interpreter; 

mod helpers;

#[test]
fn basic() {
    let code = helpers::load_file("basic");

    if let Err(error) = lua_interpreter::parse(&code){
        panic!("{}",error);
    }

    assert!(false);
}
