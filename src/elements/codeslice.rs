#[derive(Debug)]
pub struct CodeSlice {
    // for generating the section of code
    abs_start : usize,
    abs_end : usize,

    // for user display
    line_no : usize,
    start : usize,
    end : usize,
}

impl std::default::Default for CodeSlice {
    fn default() -> CodeSlice {
        CodeSlice {
            abs_start : 0,
            abs_end : 0,
            line_no : 0,
            start : 0,
            end : 0
        }
    }
}

impl CodeSlice {
    pub fn empty() -> CodeSlice { CodeSlice::default() }

    pub fn new(abs_start : usize, abs_end : usize, line : usize, line_start_pos : usize) -> CodeSlice {
        CodeSlice {
            abs_start : abs_start, abs_end : abs_end,
            line_no : line,
            start : abs_start - line_start_pos + 1,
            end : abs_end - line_start_pos + 1,
        }
    }

    pub fn slice_code<'a>(&self, raw_code : &'a str) -> &'a str {
        &raw_code[self.abs_start .. self.abs_end]
    }
}