use crate::syntax::ast::{BinaryOperator, Expr, UnaryOperator};
use crate::syntax::error::{ParserError, ParseResult};
use crate::syntax::parser::Parser;
use crate::syntax::token::TokenType;

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    None,
    Assign,
    // =
    Or,
    And,
    Equality,
    // == !=
    Comparison,
    // < <= > >=
    Term,
    // + -
    Factor,
    // * /
    Unary,
    // ! -
    Call,
    // ()
    Primary,
}

impl From<&TokenType> for Precedence {
    fn from(token: &TokenType) -> Precedence {
        match token {
            TokenType::Equal => Precedence::Assign,
            TokenType::BangEqual | TokenType::EqualEqual => Precedence::Equality,
            TokenType::Plus | TokenType::Minus => Precedence::Term,
            TokenType::Star | TokenType::Slash => Precedence::Factor,
            TokenType::Bang => Precedence::Unary,
            TokenType::LeftParen => Precedence::Call,
            TokenType::Dot => Precedence::Call,
            _ => Precedence::None,
        }
    }
}

pub fn parse(parser: &mut Parser) -> ParseResult<Expr> {
    parse_expr(parser, Precedence::None) // TODO: Change to Precedence::Assignment?
}

fn parse_expr(parser: &mut Parser, precedence: Precedence) -> ParseResult<Expr> {
    let mut expr = parse_prefix(parser)?;
    while !parser.is_eof()? {
        let next_precedence = Precedence::from(parser.peek_type()?);
        if precedence >= next_precedence {
            break;
        }
        expr = parse_infix(parser, expr)?;
    }
    Ok(expr)
}

fn parse_prefix(parser: &mut Parser) -> ParseResult<Expr> {
    match parser.peek_type()? {
        TokenType::Number | TokenType::Identifier | TokenType::String | TokenType::True | TokenType::False => {
            parse_primary(parser)
        }
        TokenType::Bang | TokenType::Minus => parse_unary(parser),
        _ => Err(ParserError::Unexpected(parser.peek_type()?.clone())),
    }
}

fn parse_infix(parser: &mut Parser, left: Expr) -> ParseResult<Expr> {
    match parser.peek_type()? {
        TokenType::BangEqual
        | TokenType::EqualEqual
        | TokenType::LessThan
        | TokenType::LessThanEqual
        | TokenType::GreaterThan
        | TokenType::GreaterThanEqual
        | TokenType::Plus
        | TokenType::Minus
        | TokenType::Star
        | TokenType::Slash => parse_binary(parser, left),
        TokenType::LeftParen => parse_call(parser, left),
        _ => Err(ParserError::Unexpected(parser.peek_type()?.clone())),
    }
}

fn parse_primary(parser: &mut Parser) -> ParseResult<Expr> {
    let token = parser.consume()?;
    match token.token_type {
        TokenType::Number => Ok(Expr::number(token.source.parse::<f64>().unwrap())),
        TokenType::String => Ok(Expr::string(token.source.to_string())),
        TokenType::True => Ok(Expr::true_()),
        TokenType::False => Ok(Expr::false_()),
        TokenType::Identifier => {
            let ident = token.source.to_string();

            Ok(if parser.match_(TokenType::Equal)? {
                let expr = parser.expression()?;
                Expr::let_set(ident, expr)
            } else {
                Expr::let_get(ident)
            })
        }
        _ => Err(ParserError::ExpectedPrimary(token.token_type.clone())),
    }
}

fn parse_binary(parser: &mut Parser, left: Expr) -> ParseResult<Expr> {
    let op_token = parser.consume()?;
    let precedence = Precedence::from(&op_token.token_type);
    let op = BinaryOperator::from_token(&op_token.token_type)?;
    let right = parse_expr(parser, precedence)?;

    Ok(Expr::binary(left, op, right))
}

fn parse_unary(parser: &mut Parser) -> ParseResult<Expr> {
    let op_token = parser.consume()?;
    let op = UnaryOperator::from_token(&op_token.token_type)?;
    let right = parse_expr(parser, Precedence::Unary)?;

    Ok(Expr::unary(op, right))
}

fn parse_call(parser: &mut Parser, left: Expr) -> ParseResult<Expr> {
    parser.expect(TokenType::LeftParen)?;

    let mut args = vec![];
    while !parser.check(TokenType::RightParen)? && !parser.check(TokenType::EOF)? {
        args.push(parser.expression()?);

        if !parser.match_(TokenType::Comma)? {
            break;
        }
    }

    parser.expect(TokenType::RightParen)?;

    Ok(Expr::call(left, args))
}