#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub source: &'a str,
    pub position: Position,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, source: &'a str, position: Position) -> Self {
        Token {
            token_type,
            source,
            position,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Star,
    Slash,
    Semicolon,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,

    String,
    Number,
    True,
    False,

    Let,
    Print,
    If,
    Else,
    Fun,
    Identifier,

    EOF,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

impl Position {
    pub fn new(start: usize, end: usize, line: usize) -> Self {
        Position { start, end, line }
    }
}

pub trait ToKeyword {
    fn to_keyword(self) -> TokenType;
}

impl ToKeyword for &str {
    fn to_keyword(self) -> TokenType {
        match self {
            "fun" => TokenType::Fun,
            "let" => TokenType::Let,
            "print" => TokenType::Print,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "true" => TokenType::True,
            "false" => TokenType::False,
            _ => TokenType::Identifier,
        }
    }
}

