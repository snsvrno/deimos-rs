#[derive(Debug,Clone)]
pub struct CodeSlice {
    // for getting the substring
    pos_start : usize,
    pos_end : usize,

    // for user display
    line_no : usize,
    start : usize,
    end : usize,
}

impl CodeSlice {
    pub fn empty() -> CodeSlice {
        CodeSlice {
            pos_start : 0,
            pos_end : 0,
            line_no : 0,
            start : 0,
            end : 0,
        }
    }

    pub fn new(abs_start : usize, abs_end : usize, line : usize, line_start_pos : usize) -> CodeSlice {
        CodeSlice {
            pos_start : abs_start,
            pos_end : abs_end,
            line_no : line,
            start : abs_start - line_start_pos + 1,
            end : abs_end - line_start_pos + 1,
        }
    }
}