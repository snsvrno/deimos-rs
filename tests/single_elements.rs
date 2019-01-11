extern crate lua_interpreter; 

#[macro_use] mod utils;
use crate::utils::*;

#[test]
fn unary() {
    let code = r"
    bob = -5
    bob = not true
    bob = -10 + 1
    bob = 10 + -1
    ";
    
    // builds what the output should be, so we can check it 
    // and make sure its parsed the same. (using test only macros)
    let check_against = tree!(
        binary!("=","bob",> unary!("-","5")),
        binary!("=","bob",> unary!("not","true")),
        binary!("=","bob",
            > binary!("+",
                > unary!("-","10"),
                "1")),
        binary!("=","bob",
            > binary!("+",
                "10",
                > unary!("-","1")))
    );
    
    match lua_interpreter::parse(&code){ 
        Err(error) => panic!("{}",error), 
        Ok(tree) => assert_eq!(tree,check_against),
    }
}

#[test]
fn do_single_block() {
    let code = "do bob = 5 end";
    let code2 = r"
    do
        bob = 5
    end
    ";
    
    // builds what the output should be, so we can check it 
    // and make sure its parsed the same. (using test only macros)
    let check_against = tree!(do_block!(
        binary!("=","bob","5")
    ));
    
    match lua_interpreter::parse(&code){ 
        Err(error) => panic!("{}",error), 
        Ok(tree) => assert_eq!(tree,check_against),
    }

    match lua_interpreter::parse(&code2){ 
        Err(error) => panic!("{}",error), 
        Ok(tree) => assert_eq!(tree,check_against),
    }
}

#[test]
fn do_nested_block() {
    let code = r"
    do
        bob = 5
        do jan = 1 end
        
        do 
            jill = 23 + 5 - 1
            bob = 5 * 3
        end
    end
    ";
    
    // builds what the output should be, so we can check it 
    // and make sure its parsed the same. (using test only macros)
    let check_against = tree!(do_block!(
        binary!("=","bob","5"),
        do_block!(
            binary!("=","jan","1")),
        do_block!(
            binary!("=","jill",
                > binary!("-",
                    > binary!("+","23","5"),
                    "1")),
            binary!("=","bob",
                > binary!("*","5","3")))
    ));
    
    match lua_interpreter::parse(&code){ 
        Err(error) => panic!("{}",error), 
        Ok(tree) => assert_eq!(tree,check_against),
    }
}
