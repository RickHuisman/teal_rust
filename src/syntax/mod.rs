use crate::syntax::ast::Program;
use crate::syntax::error::{LexResult, ParseResult};
use crate::syntax::lexer::Lexer;
use crate::syntax::parser::Parser;
use crate::syntax::token::{Token, TokenType};

mod token;
mod lexer;
mod parser;
mod expr_parser;
mod error;
pub mod ast;

pub fn parse<'a>(tokens: &'a mut Vec<Token<'a>>) -> ParseResult<Program> {
    let mut parser = Parser::new(tokens);

    let mut ast = vec![];
    while !parser.is_eof()? {
        ast.push(parser.declaration()?);
    }

    Ok(ast)
}

pub fn lex(source: &str) -> LexResult<Vec<Token>> {
    let mut lexer = Lexer::new(source);

    let mut tokens = vec![];
    loop {
        if let Some(token) = lexer.read_token()? {
            if let TokenType::EOF = token.token_type {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
    }

    Ok(tokens)
}