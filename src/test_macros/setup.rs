#[macro_export]
macro_rules! setup_simple {
    ($code:expr) => ({
        let scanner = crate::scanner::Scanner::init($code).scan().unwrap();
        crate::parser::Parser::from_scanner(scanner).unwrap()
    });
}
