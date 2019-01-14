#[macro_export]
macro_rules! comment {
    ($t:expr) => ({
        crate::elements::Token::simple(crate::elements::TokenType::Comment($t.to_string()))
    });
}

macro_rules! comment_tt {
    ($t:expr) => ({
        crate::elements::TokenType::Comment($t.to_string())
    });
}