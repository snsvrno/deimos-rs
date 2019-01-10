extern crate lua_interpreter; 

mod helpers;

#[test]
fn basic() {
    let code = helpers::load_file("basic");

    lua_interpreter::parse(&code);

    assert!(false);
}
