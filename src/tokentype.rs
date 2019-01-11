#[derive(Debug,PartialEq,Clone)]
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

    WhiteSpace,
    EOL,
    EOF,

}

impl TokenType {
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

    pub fn match_keyword(text : &str) -> Option<TokenType> {
        match text {
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
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenType::Identifier(text) => write!(f,"{}",text),
            TokenType::String(text) => write!(f,"{}",text),
            TokenType::Number(number) => write!(f,"{}",number),
            _ => write!(f,"{:?}",self)
        }
    }
}