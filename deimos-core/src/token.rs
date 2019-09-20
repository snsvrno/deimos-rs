#[derive(Debug,PartialEq)]
pub enum Token {
    
    // single-character tokens /////////////////////
    Plus,           Minus,          Star,
    Slash,          Percent,        Carrot,
    Pound,          LessThan,       GreaterThan,
    Equal,          LeftParen,      RightParen,
    LeftMoustache,  RightMoustache, LeftBracket,
    RightBracket,   SemiColon,      Colon,
    Comma,          Period,

    // double-character tokens ////////////////////
    DoublePeriod,    EqualEqual,    NotEqual,
    GreaterEqual,    LessEqual,

    // triple-character tokens ////////////////////
    TriplePeriod,

    // keywords ///////////////////////////////////
    And,    Break,    Do,    Else,      Elseif,
    End,    False,    For,   Function,  If,
    In,     Local,    Nil,   Not,       Or,
    Repeat, Return,   Then,  True,      Until,
    While,

    // literals ///////////////////////////////////
    Identifier(String),    String(String),
    Number(f32),           MultiLineString(String),

    // other /////////////////////////////////////
    Comment(String),
    WhiteSpace,

    // special characters ////////////////////////
    EOL,
    EOF,
}

impl Token {

    pub fn is_eol(char : &str) -> bool {
        //! checks if the string is an end of line character
        
        match char {
            "\n" | "\r" => true,
            _ => false,
        }
    }
    pub fn is_valid_number_char(char : &str) -> bool {
        //! checks if the single length character 
        //! is a valid character that couild be in a number
        
        let allowable_ranges = vec![
            // (u start, u end, can start)
            (48,57), // 0-9
            (46,46), // .
        ];

        if char.len() == 1 {
            if let Some(c) = char.chars().next(){
                let code = c as u32;
                for range in allowable_ranges {
                    if range.0 <= code && code <= range.1 {
                        return true;
                    }
                }
            }
        }
        
        false
    }

    pub fn is_valid_word_char(char : &str, first : bool) -> bool {
        //! checks if the single length character 
        //! is a valid character that couild be in a word
        
        let allowable_ranges = vec![
            // (u start, u end, can start)
            (65,90,true), // A-Z
            (97,122,true), // a-z
            (48,57,false), // 0-9
            (95,95,true) // _
        ];

        if char.len() == 1 {
            if let Some(c) = char.chars().next(){
                let code = c as u32;
                for range in allowable_ranges {
                    if range.0 <= code && code <= range.1 {
                        if first && range.2 == false {
                            return false;
                        } else {
                            return true
                        }
                    }
                }
            }
        }
        
        false
    }

    pub fn inner_text(&self) -> String {
        //! only for use with testing, is unsafe.
        
        match self {
            Token::Identifier(text) | 
            Token::String(text) | 
            Token::MultiLineString(text) | 
            Token::Comment(text) => text.to_string(),

            _ => unimplemented!(),
        }
    }

    #[cfg(test)]
    pub fn is_same_type(&self, other : &Token) -> bool {
        //! checks if the two tokens are the same type, there are only
        //! a few cases where we can't rely on ==, defined below.
        
        match (self,other) {

            // these are the only cases where we can't use == because 
            // they have members that might be different, but we only
            // care about the type (if we are using this function).
            ( Token::Identifier(_), Token::Identifier(_) ) => true,
            ( Token::String(_), Token::String(_) ) => true,
            ( Token::MultiLineString(_), Token::MultiLineString(_) ) => true,
            ( Token::Number(_), Token::Number(_) ) => true,
            ( Token::Comment(_), Token::Comment(_) ) => true,

            // fallback to PartialEq / Eq
            _ => self == other
        }
    }

    pub fn match_keyword(word : &str) -> Option<Token> {
        //! list of all the fixed keywords in lua.
        
        match word {
            "and" => Some(Token::And),
            "break" => Some(Token::Break),
            "do" => Some(Token::Do),
            "else" => Some(Token::Else),
            "elseif" => Some(Token::Elseif),
            "end" => Some(Token::End),
            "false" => Some(Token::False),
            "for" => Some(Token::For),
            "function" => Some(Token::Function),
            "if" => Some(Token::If),
            "in" => Some(Token::In),
            "local" => Some(Token::Local),
            "nil" => Some(Token::Nil),
            "not" => Some(Token::Not),
            "or" => Some(Token::Or),
            "repeat" => Some(Token::Repeat),
            "return" => Some(Token::Return),
            "then" => Some(Token::Then),
            "true" => Some(Token::True),
            "until" => Some(Token::Until),
            "while" => Some(Token::While),
            _ => None,
        }
    }
}