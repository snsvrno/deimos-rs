#[derive(PartialEq,Debug)]
pub enum TokenType {
    /// Tokens taken from the Lua [5.1 manual](https://www.lua.org/manual/5.1/manual.html#2.1)

    // single-character tokens
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Carrot,
    Pound,
    LessThan,
    GreaterThan,
    Equal,
    LeftParen,
    RightParen,
    LeftMoustache,
    RightMoustache,
    LeftBracket,
    RightBracket,
    SemiColon,
    Colon,
    Comma,
    Period,

    // double-character tokens
    DoublePeriod,
    EqualEqual,
    NotEqual,
    GreaterEqual,
    LessEqual,

    // triple-character tokens
    TriplePeriod,

    // keywords
    And,
    Break,
    Do,
    Else,
    Elseif,
    End,
    False,
    For,
    Function,
    If,
    In,
    Local,
    Nil,
    Not,
    Or,
    Repeat,
    Return,
    Then,
    True,
    Until,
    While,

    // literals
    Identifier(String),
    String(String),
    Number(f32),

    Comment(String),
    WhiteSpace,
    EOL,
    EOF,

}

impl TokenType {

    pub fn valid_word_char(char : &str, first : bool) -> bool {
        let allowable_ranges = vec![
            // (u start, u end, can start)
            (65,90,true), // A-Z
            (97,122,true), // a-z
            (48,57,false), // 0-9
            (95,95,false) // _
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

    pub fn valid_number_char(char : &str) -> bool {
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

    #[cfg(test)]
    pub fn match_symbol(text : &str) -> Option<TokenType> {
        match text {
            "+" => Some(TokenType::Plus),
            "-" => Some(TokenType::Minus),
            "*" => Some(TokenType::Star),
            "/" => Some(TokenType::Slash),
            "%" => Some(TokenType::Percent),
            "^" => Some(TokenType::Carrot),
            "#" => Some(TokenType::Pound),
            "<=" => Some(TokenType::LessEqual),
            "<" => Some(TokenType::LessThan),
            ">=" => Some(TokenType::GreaterEqual),
            ">" => Some(TokenType::GreaterThan),
            "==" => Some(TokenType::EqualEqual),
            "=" => Some(TokenType::Equal),
            "(" => Some(TokenType::LeftParen),
            ")" => Some(TokenType::RightParen),
            "[" => Some(TokenType::LeftBracket),
            "]" => Some(TokenType::RightBracket),
            "{" => Some(TokenType::LeftMoustache),
            "}" => Some(TokenType::RightMoustache),
            ";" => Some(TokenType::SemiColon),
            ":" => Some(TokenType::Colon),
            ")," => Some(TokenType::Comma),
            "." => Some(TokenType::Period),
            ".." => Some(TokenType::DoublePeriod),
            "..." => Some(TokenType::TriplePeriod),
            "~=" => Some(TokenType::NotEqual),
            _ => None,
        }
    }

    pub fn match_keyword(word : &str) -> Option<TokenType> {
        match word {
            "and" => Some(TokenType::And),
            "break" => Some(TokenType::Break),
            "do" => Some(TokenType::Do),
            "else" => Some(TokenType::Else),
            "elseif" => Some(TokenType::Elseif),
            "end" => Some(TokenType::End),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "function" => Some(TokenType::Function),
            "if" => Some(TokenType::If),
            "in" => Some(TokenType::In),
            "local" => Some(TokenType::Local),
            "nil" => Some(TokenType::Nil),
            "not" => Some(TokenType::Not),
            "or" => Some(TokenType::Or),
            "repeat" => Some(TokenType::Repeat),
            "return" => Some(TokenType::Return),
            "then" => Some(TokenType::Then),
            "true" => Some(TokenType::True),
            "until" => Some(TokenType::Until),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }

    pub fn oop(&self) -> usize {
        match self {
            TokenType::Carrot => 1,
            TokenType::Star |
            TokenType::Slash => 2,
            TokenType::Plus |
            TokenType::Minus => 3,
            TokenType::DoublePeriod =>4,
            TokenType::LessThan |            
            TokenType::LessEqual |
            TokenType::GreaterThan |
            TokenType::GreaterEqual |
            TokenType::NotEqual |
            TokenType::EqualEqual => 5,
            TokenType::And => 6,
            TokenType::Or => 7,
            _ => 0,
        }
    }

    pub fn oop_binary(t1 : &TokenType, t2 : &TokenType) -> bool {
        //! going to assume that both tokens are of type binary, since
        //! this should only be called when comparing two binaries.
        
        // ungraceful panic.
        if t1.oop() == 0 || t2.oop() == 0 {
            panic!("One of these isn't a binary operator!! {:?} or {:?} ?",t1,t2);
        }

        // checks if t1's tier is lower than t2 (meaning higher priority). If
        // they are the same tier then still returns false because this used
        // when creating binaries and is read left to right, which is the order
        // of precidence, so if they are the same tier than their are already
        // assembled correctly.
        t1.oop() < t2.oop()
    }
}

mod tests {
    
    #[test]
    fn  allowable_characters() {
        use crate::elements::TokenType;

        assert!(!TokenType::valid_word_char("_",true));

        let letters = vec![
            "Q","W","E","R","T","Y","U","I","O","P","L","K","J",
            "H","G","F","D","S","A","Z","X","C","V","B","N","M",
            "q","w","e","r","t","y","u","i","o","p","l","k","j",
            "h","g","f","d","s","a","z","x","c","v","b","n","m"
        ];

        let symbols = vec![
            "!","@","#","$","%","^","&","*","(",")","{","}","|",
            ":","\"","<",">","?","-","=","+"
        ];

        let numbers = vec!["0","1","2","3","4","5","6","7","8","9","0"];
        
        for l in letters {
            assert!(TokenType::valid_word_char(l,true));
            assert!(TokenType::valid_word_char(l,false));
        } 

        for s in symbols {
            assert!(!TokenType::valid_word_char(s,true));
            assert!(!TokenType::valid_word_char(s,false));
        } 

        for n in numbers {
            assert!(!TokenType::valid_word_char(n,true));
            assert!(TokenType::valid_word_char(n,false));
        } 
    }
}
