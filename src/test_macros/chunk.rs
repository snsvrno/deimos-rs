#[macro_export]
macro_rules! chunk {
    ($($statement:expr),*) => ({
        let mut statements : Vec<crate::elements::Statement> = Vec::new();

        $(
            statements.push($statement);
        )*

        crate::elements::Chunk::new(statements)
    });
}