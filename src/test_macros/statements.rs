#[macro_export]
macro_rules! unary {
    ($op:expr,$t:expr) => ({
        let top = crate::elements::Statement::Token(token!($op));
        let tt = crate::elements::Statement::Token(token!($t));

        top.into_unary(tt)
    });
}

#[macro_export]
macro_rules! binary {
    ($op:expr,$t1:expr,$t2:expr) => ({
        let top = crate::elements::Statement::Token(token!($op)); 
        let tt1 = crate::elements::Statement::Token(token!($t1));
        let tt2 = crate::elements::Statement::Token(token!($t2));

        top.into_binary(tt1,tt2)
    });

    ($op:expr,s $t1:expr,$t2:expr) => ({
        let top = crate::elements::Statement::Token(token!($op));
        let tt2 = crate::elements::Statement::Token(token!($t2));

        top.into_binary($t1,tt2)
    });
}
