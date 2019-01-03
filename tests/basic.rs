extern crate lua_interpreter;

use lua_interpreter:: {Parser, Value};

#[test]
fn basic() {
    let code = r"
    bob = 10 + 5
    jim = 30 - 2
    linda = 1332-2;
    do
        local bob = bob + 5
        assert()
    end
    ";

    let mut parser = Parser::new(&code);

    if let Err(error) = parser.eval() {
        println!("ERROR : {}",error);
        assert!(false);
    }

    assert_eq!(parser.value_of("bob").unwrap(), &Value::Int(15));
    assert_eq!(parser.value_of("jim").unwrap(), &Value::Int(28));
    assert_eq!(parser.value_of("linda").unwrap(), &Value::Int(1330));

    assert!(false);
}
