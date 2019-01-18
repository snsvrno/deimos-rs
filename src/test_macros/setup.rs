#[macro_export]
macro_rules! setup_simple {
    ($code:expr) => ({
        setup_simple!($code,1)
    });

    ($code:expr,$statements:expr) => ({
        let scanner = crate::scanner::Scanner::init($code).scan().unwrap();
        let parser = crate::parser::Parser::from_scanner(scanner).unwrap();
        assert_eq!(1,parser.chunks.len());
        parser
    });
}

#[macro_export]
macro_rules! setup {
    ($code:expr) => ({
        let scanner = crate::scanner::Scanner::init($code).scan().unwrap();
        let parser = crate::parser::Parser::from_scanner(scanner).unwrap();
        parser
    });
}
