#[macro_export]
macro_rules! unary {
    ($op:expr,$t:expr) => ({
        let top = crate::elements::Statement::Token(token!($op));
        let tt = crate::elements::Statement::Token(token!($t));

        top.into_unary(tt)
    });
}