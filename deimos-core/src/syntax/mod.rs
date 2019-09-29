mod element; pub use element::SyntaxElement;

pub mod exp;
pub mod explist;
pub mod statement;
pub mod var;
pub mod varlist;

pub enum SyntaxResult {
    Done,
    None,
    More,
    Error(usize,usize,String) // code_start, code_end, description
}