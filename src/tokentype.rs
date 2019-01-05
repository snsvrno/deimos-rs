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

impl std::fmt::Display for TokenType {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}