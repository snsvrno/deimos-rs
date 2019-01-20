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

    ($op:expr,$t1:expr,s $t2:expr) => ({
        let top = crate::elements::Statement::Token(token!($op));
        let tt1 = crate::elements::Statement::Token(token!($t1));

        top.into_binary(tt1,$t2)
    });

    ($op:expr,s $t1:expr,s $t2:expr) => ({
        let top = crate::elements::Statement::Token(token!($op));

        top.into_binary($t1,$t2)
    });
}

#[macro_export]
macro_rules! do_end {
    ($($statement:expr),*) => ({
        let mut list : Vec<Box<crate::elements::Statement>> = Vec::new();

        $(
            list.push(Box::new($statement));
        )*

        crate::elements::Statement::DoEnd(list)
    });
}

#[macro_export]
macro_rules! while_do_end {
    ($expr:expr, $($statement:expr),*) => ({
        let expr = crate::elements::Statement::Token(token!($expr));
        crate::elements::Statement::WhileDoEnd(Box::new(expr),list!($($statement),*))
    });
    
    (s $expr:expr, $($statement:expr),*) => ({
        crate::elements::Statement::WhileDoEnd(Box::new(expr),list!($($statement),*))
    });
}

#[macro_export]
macro_rules! list {
    ($($statement:expr),*) => ({
        let mut list : Vec<Box<crate::elements::Statement>> = Vec::new();

        $(
            list.push(Box::new($statement));
        )*

        list
    });
}

#[macro_export]
macro_rules! empty {
    () => ({
        crate::elements::Statement::Empty
    });
}


#[macro_export]
macro_rules! assignment {
    ($vars:expr,$exprs:expr) => ({
        crate::elements::Statement::Assignment($vars,$exprs)
    });
}
#[macro_export]
macro_rules! assignment_local {
    ($vars:expr,$exprs:expr) => ({
        crate::elements::Statement::AssignmentLocal($vars,$exprs)
    });
}

#[macro_export]
macro_rules! statement {
    ($token:expr) => ({
        crate::elements::Statement::Token(token!($token))
    });
}
