#[derive(Debug)]
pub enum CodeRef<T> {
    CodeRef { 
        item : T, 
        code_start : usize, 
        code_end : usize ,
        line_number : usize,
    }, // the only thing    
}

impl<T> std::fmt::Display for CodeRef<T> where T : std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.item())
    }
}

impl<T> CodeRef<T> {
    pub fn item<'a>(&'a self) -> &'a T {
        //! easy way to access the inside of the coderef without having to do a pattern
        //! like below.

        let CodeRef::CodeRef { ref item, code_start : _, code_end : _, line_number : _ } = self;
        item
    }

    pub fn code_start<'a>(&'a self) -> usize {
        //! easy way to access the inside of the coderef without having to do a pattern
        //! like below.

        let CodeRef::CodeRef { item : _ , ref code_start, code_end : _, line_number : _ } = self;
        *code_start
    }

    pub fn code_end<'a>(&'a self) -> usize {
        //! easy way to access the inside of the coderef without having to do a pattern
        //! like below.

        let CodeRef::CodeRef { item : _, code_start : _, ref code_end, line_number : _ } = self;
        *code_end
    }

    pub fn line_number<'a>(&'a self) -> usize {
        //! easy way to access the inside of the coderef without having to do a pattern
        //! like below.

        let CodeRef::CodeRef { item: _ , code_start : _, code_end : _, ref line_number } = self;
        *line_number
    }

    pub fn i<'a>(&'a self) -> &'a T {
        //! a lazy shorthad for self.item, used because the code might get really
        //! messy, so i thought a .i might be easier?

        self.item()
    }

    pub fn unwrap(self) -> T {
        //! removes the coderef from the item

        let CodeRef::CodeRef { item, code_start : _, code_end : _, line_number : _ } = self;
        item
    } 
}

impl<T> PartialEq<T> for CodeRef<T> where T : PartialEq {
    // implemented so i can directly compare a wrapped item
    // with a raw item

    fn eq(&self, other: &T) -> bool {
        self.item() == other
    }
}