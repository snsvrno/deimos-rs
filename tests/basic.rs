extern crate lua_interpreter;

use lua_interpreter:: {Parser, Value};

#[test]
fn basic() {
    let code = r"
    bob = 10 + 5
    jim = 30 - 2
    linda = 1332-2;
    bob = bob + 5
    ";

    let mut parser = Parser::new(&code);

    parser.eval();

    assert_eq!(parser.value_of("bob").unwrap(), &Value::Int(20));
    assert_eq!(parser.value_of("jim").unwrap(), &Value::Int(28));
    assert_eq!(parser.value_of("linda").unwrap(), &Value::Int(1330));
}
