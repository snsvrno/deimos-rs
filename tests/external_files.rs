extern crate lua_interpreter; 

mod utils;

#[test]
#[ignore]
fn basic() {
    let code = utils::load_file("basic");

    if let Err(error) = lua_interpreter::parse(&code){
        panic!("{}",error);
    }

    assert!(true);
}
