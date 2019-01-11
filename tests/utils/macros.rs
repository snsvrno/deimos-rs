#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! token {
    ($t:expr) => {
        match lua_interpreter::tokentype::TokenType::match_symbol($t) {
            Some(tt) => lua_interpreter::grammar::gram::Gram::Token(
                lua_interpreter::token::Token::simple(tt)),
            None => match lua_interpreter::tokentype::TokenType::match_keyword($t) {
                Some(tt) => {
                    let ttt = lua_interpreter::token::Token::simple(tt);
                    match lua_interpreter::grammar::literal::Literal::create_into_gram(ttt.clone()) {
                        Some(gram) => gram,
                        None => lua_interpreter::grammar::gram::Gram::Token(ttt)
                    }
                },
                None => match $t.parse::<f32>() {
                    Ok(float) => { 
                        let tt = lua_interpreter::tokentype::TokenType::Number(float);
                        let token = lua_interpreter::token::Token::simple(tt);
                        lua_interpreter::grammar::literal::Literal::create_into_gram(token).unwrap()
                    },
                    Err(_) => {
                        let tt = lua_interpreter::tokentype::TokenType::Identifier($t.to_string());
                        lua_interpreter::grammar::literal::Literal::create_into_gram(
                            lua_interpreter::token::Token::simple(tt)).unwrap()
                    }
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! unary {
    ($op:expr,$t:expr) => ({
        let unary = lua_interpreter::grammar::unary::Unary::create_into_gram(
            &token!($op),
            &lua_interpreter::grammar::expression::Expression::create_into_gram(&token!($t)).unwrap()
        ).unwrap();
        lua_interpreter::grammar::expression::Expression::create_into_gram(&unary).unwrap()
    });

    ($op:expr,> $t:expr) => ({
        let unary = lua_interpreter::grammar::unary::Unary::create_into_gram(
            &token!($op),
            &$t
        ).unwrap();
        lua_interpreter::grammar::expression::Expression::create_into_gram(&unary).unwrap()
    });
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! binary {
    ($op:expr,$left:expr,$right:expr) => ({
        let binary = lua_interpreter::grammar::binary::Binary::create_into_gram(
            &lua_interpreter::grammar::expression::Expression::create_into_gram(&token!($left)).unwrap(),
            &token!($op),
            &lua_interpreter::grammar::expression::Expression::create_into_gram(&token!($right)).unwrap()
        ).unwrap();
        lua_interpreter::grammar::expression::Expression::create_into_gram(&binary).unwrap()
    });

    ($op:expr,> $left:expr,$right:expr) => ({
        let binary = lua_interpreter::grammar::binary::Binary::create_into_gram(
            &$left,
            &token!($op),
            &lua_interpreter::grammar::expression::Expression::create_into_gram(&token!($right)).unwrap()
        ).unwrap();
        lua_interpreter::grammar::expression::Expression::create_into_gram(&binary).unwrap()
    });

    ($op:expr,$left:expr,> $right:expr) => ({
        let binary = lua_interpreter::grammar::binary::Binary::create_into_gram(
            &lua_interpreter::grammar::expression::Expression::create_into_gram(&token!($left)).unwrap(),
            &token!($op),
            &$right
        ).unwrap();
        lua_interpreter::grammar::expression::Expression::create_into_gram(&binary).unwrap()
    });

    ($op:expr,> $left:expr,> $right:expr) => ({
        let binary = lua_interpreter::grammar::binary::Binary::create_into_gram(
            &$left,
            &token!($op),
            &$right
        ).unwrap();
        lua_interpreter::grammar::expression::Expression::create_into_gram(&binary).unwrap()
    });
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! do_block {
    ($($t:expr),*) => ({
        let mut chunks : Vec<lua_interpreter::chunk::Chunk> = Vec::new();
        $(
            chunks.push(lua_interpreter::chunk::Chunk::wrap($t));
        )*
       lua_interpreter::grammar::blockdo::BlockDo::create_into_gram(chunks)
    });
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! tree {
    ($($t:expr),*) => ({
        let mut chunks : Vec<lua_interpreter::chunk::Chunk> = Vec::new();
        $(
            chunks.push(lua_interpreter::chunk::Chunk::wrap($t));
        )*
       lua_interpreter::tree::Tree::simple(chunks)
    });
}