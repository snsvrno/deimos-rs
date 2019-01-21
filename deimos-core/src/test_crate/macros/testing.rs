#[macro_export]
macro_rules! assert_float {
    ($left:expr,$right:expr) => ({
        let difference = $left - $right;

        if difference.abs() > 0.01 {
            panic!("Float values aren't close enough:\nleft: {}\nright: {}",$left,$right)
        }
    });

    ($left:expr,$right:expr,$comp:expr) => ({
        let difference = abs($left - $right);

        if difference > $comp {
            panic!("Float values aren't close enough:\nleft: {}\nright: {}",$left,$right)
        }
    });
}

#[macro_export]
macro_rules! print_em {
    ($left:expr) => ({
        println!("{}",$left);
    });
}