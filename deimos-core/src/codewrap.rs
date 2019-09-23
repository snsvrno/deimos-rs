/// this is designed to be a wrapping object around
/// tokens and statements and other things so that 
/// raw_code information is transfered and usable

pub trait CodeWrappable { }

#[derive(Debug)]
pub enum CodeWrap<CW:CodeWrappable> {
    // token, where it starts, where it ends
    CodeWrap(CW,usize, usize),
}

impl<CW:CodeWrappable> CodeWrap<CW> {
    pub fn item(&self) -> &CW {
        if let CodeWrap::CodeWrap(ref inside, _ , _) = self {
            inside
        } else {
            unimplemented!();
        }
    }

    pub fn start(&self) -> usize {
        if let CodeWrap::CodeWrap(_,start , _) = self {
            *start
        } else {
            unimplemented!();
        }
    }

    pub fn end(&self) -> usize {
        if let CodeWrap::CodeWrap(_, _, end) = self {
            *end
        } else {
            unimplemented!();
        }
    }
}

impl<CW:CodeWrappable> PartialEq<CW> for CodeWrap<CW> where CW : PartialEq {
    //! used to compare a wrapped item with an unwrapped item
    //! making comparisons much easier
    
    fn eq(&self, other : &CW) -> bool {
        self.item() == other
    }
}