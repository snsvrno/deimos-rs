use scanner::Scanner;
use failure::Error;

pub struct Tree<'a> {
    raw_code : &'a str,
}

impl<'a> Tree<'a> {
    pub fn from_scanner(mut scanner : Scanner<'a>) -> Result<Tree,Error> {
        let (raw_code,tokens) = scanner.explode();
        let mut tree = Tree {
            raw_code : raw_code,
        };


        Ok(tree)
    }
}
