pub mod codeinfo; use codeinfo::CodeInfo;
pub mod scanner;
pub mod parser;

const LEFT_PADDING : &str = "  ";
const MARKER : &str = "^";
const TERMINAL_WIDTH : usize = 70;

pub fn display_error_general(f : &mut std::fmt::Formatter<'_>, description : &str) -> std::fmt::Result {
    //! a general error message, this doesn't show code, instead some general message 
    //! most of the time this is a program error.
    
    write!(f, "ERROR : {}", description)
}

pub fn display_error(f : &mut std::fmt::Formatter<'_>, error_type : &str, info : &CodeInfo) -> std::fmt::Result {

    let position_on_line = get_position_on_line(&info.raw_code, info.cursor_pos);
    let arrow = build_marker(info.span,position_on_line);

    write!(f, "error: {error_type}\n    --> {file}:{line2}:{code_start}\n     |\n {line}|{padding}{code}\n     |{padding}{arrow} {description}\n",
        error_type = error_type,
        file = info.file_name,
        line = pad_number(info.line_number, 4),
        code_start = position_on_line + 1,
        line2 = format!("{}",info.line_number),
        code = slice_code(&info.raw_code, info.cursor_pos),
        padding = LEFT_PADDING,
        arrow = arrow,
        description = new_line_format(&info.description, TERMINAL_WIDTH, arrow.len() + LEFT_PADDING.len() + 6),
    )
}

fn new_line_format(text : &str, width : usize, padding : usize) -> String {
    //! tries to do a smart text wrap on the description

    if width < padding { return text.to_string(); } // a console sizing issue, lets just give them bad text        
    let real_width : usize = width - padding;

    let padding_string = pad(padding + 1);

    let mut new_text : String = String::new();
    let mut factor = 1;
    loop {
        if factor * real_width > text.len() { 
            new_text = format!("{}{}{}", 
                new_text, 
                if new_text.len() == 0 { "".to_string() } else { format!("\n{}", padding_string)  },
                &text[(factor-1) * real_width ..]
            );
            break; 
        }

        new_text = format!("{}{}{}", 
            new_text,
            if new_text.len() == 0 { "".to_string() } else { format!("\n{}", padding_string) },
            &text[(factor-1) * real_width .. factor * real_width]
        );
        factor += 1;
    }

    new_text
}

fn get_position_on_line(code : &str, start : usize) -> usize {
    //! gets where the start is relative to the start of that line
    
    use crate::token::Token;
    
    // determines where the start of the line is
    let line_start : usize = {
        let mut pos : usize = start;
        
        loop {
            if pos == 0 { break; }

            if Token::is_eol(&code[pos - 1 .. pos]) { break; }
            pos = pos - 1;
        }    

        pos
    };

    // and we go forward until we don't get a whitespace
    for i in line_start .. code.len() {
        if &code[i .. i+1] != " " {
            println!("{} {} {}",start, line_start, i);
            return start - i;
        }
    }
    
    start - line_start
}

fn slice_code(code : &str, start : usize) -> String {
    //! gets the line of code, looks forward and backward from
    //! the start to get the entire line.
    
    use crate::token::Token;

    // going to go backwards until we get the start of the line
    // or we get zero.
    let line_start : usize = {
        let mut pos : usize = start;
        
        loop {
            if pos == 0 { break; }

            if Token::is_eol(&code[pos - 1 .. pos]) { break; }
            pos = pos - 1;
        }    

        pos
    };

    // now we do the same thing and go the other way
    let line_end : usize = {
        let mut pos : usize = start;
        
        loop {
            if pos >= code.len() { break; }

            if Token::is_eol(&code[pos .. pos+1]) { break; }
            pos = pos + 1;
        }    

        pos
    };

    let code_slice = code[line_start .. line_end].to_string();

    // now we need to remove the leading zeros (if any)
    for i in 0 .. code_slice.len() {
        if &code_slice[i .. i+1] != " " {
            return code_slice[i ..].to_string();
        }
    }

    code_slice
}

fn build_marker(width : usize, position_on_line : usize) -> String {
    // uses the MARKER constant to build a string of
    // marker that is width wide (width many characters)

    let mut string = String::new(); 

    loop {
        if string.len() >= width { break; }
        string = format!("{}{}",string,MARKER);
    }

    format!("{}{}",pad(position_on_line),string)
}

fn pad(width : usize) -> String {
    //! makes a string of spaces of the desired width
    
    let mut string = String::new(); 

    loop {
        if string.len() >= width { break; }
        string = format!("{}{}",string," ");
    }

    string
}

fn pad_number(number : usize, width : usize) -> String {
    // writes the number in the string, and then adds white space to 
    // the width 

    let mut string = format!("{}",number);

    loop {
        if string.len() >= width { break; }
        string = format!("{}{}",string," ");
    }

    string
}