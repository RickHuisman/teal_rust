use crate::syntax::token::TokenType;

pub type LexResult<T> = Result<T, SyntaxError>;

#[derive(Debug)]
pub enum SyntaxError {
    UnexpectedEOF,
    UnexpectedChar,
    UnterminatedString,
}

pub type ParseResult<T> = Result<T, ParserError>;

// TODO: Use Token not TokenType.
#[derive(Debug)]
pub enum ParserError {
    Expected(TokenType, TokenType, usize),
    Unexpected(TokenType),
    ExpectedPrimary(TokenType),
    ExpectedUnaryOperator(TokenType),
    ExpectedBinaryOperator(TokenType),
    UnexpectedEOF,
}
