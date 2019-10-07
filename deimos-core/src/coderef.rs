#[derive(Debug)]
pub enum CodeRef<T> {
    CodeRef { 
        item : T, 
        code_start : usize, 
        code_end : usize ,
        line_number : usize,
    }, // the only thing    
}

impl<T> CodeRef<T> {
    pub fn item<'a>(&'a self) -> &'a T {
        let CodeRef::CodeRef { ref item, code_start : _, code_end : _, line_number : _ } = self;
        item
    }
}