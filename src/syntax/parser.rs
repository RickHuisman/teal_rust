use crate::syntax::ast::*;
use crate::syntax::error::{ParserError, ParseResult};
use crate::syntax::expr_parser;
use crate::syntax::token::{Token, TokenType};

pub struct Parser<'a> {
    tokens: &'a mut Vec<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a mut Vec<Token<'a>>) -> Self {
        tokens.reverse();
        Parser { tokens }
    }

    pub fn parse_top_level_expr(&mut self) -> ParseResult<Expr> {
        match self.peek_type()? {
            TokenType::Let => self.parse_let(),
            TokenType::Def => self.parse_def(),
            TokenType::Do => self.parse_do(),
            TokenType::Puts => self.parse_puts(),
            TokenType::If => self.parse_if(),
            _ => self.parse_expr_statement(),
        }
    }

    fn parse_let(&mut self) -> ParseResult<Expr> {
        self.expect(TokenType::Let)?;

        let ident = self.parse_identifier()?;

        let initializer = if self.match_(TokenType::Equal)? {
            self.parse_expr_statement()?
        } else {
            self.expect(TokenType::Semicolon)?;
            Expr::number(4.0) // TODO: NIL.
            // Expr::Literal(LiteralExpr::Nil)
        };

        Ok(Expr::let_assign(ident, initializer))
    }

    fn parse_def(&mut self) -> ParseResult<Expr> {
        self.expect(TokenType::Def)?;

        let ident = self.parse_identifier()?;

        self.expect(TokenType::LeftParen)?;
        let args = self.parse_args()?;
        self.expect(TokenType::RightParen)?;

        self.expect(TokenType::Semicolon)?;

        let body = self.block()?;
        let fun_decl = FunDecl::new(args, body);

        Ok(Expr::def(ident, fun_decl))
    }

    fn parse_do(&mut self) -> ParseResult<Expr> {
        self.expect(TokenType::Do)?;
        self.match_(TokenType::Semicolon)?;
        Ok(Expr::Block(self.block()?))
    }

    fn parse_puts(&mut self) -> ParseResult<Expr> {
        self.expect(TokenType::Puts)?;
        let expr = self.parse_expr_statement()?;
        Ok(Expr::puts(expr))
    }

    fn parse_if(&mut self) -> ParseResult<Expr> {
        // Consume "if".
        self.expect(TokenType::If)?;

        let cond = self.expression()?;

        self.expect(TokenType::Do)?;
        self.match_(TokenType::Semicolon)?;

        // Then
        let mut then = vec![];
        loop {
            if self.match_(TokenType::End)? {
                break;
            }
            if self.check(TokenType::Else)? {
                break;
            }

            then.push(self.parse_top_level_expr()?);
        }

        let else_clause = if self.match_(TokenType::Else)? {
            self.match_(TokenType::Semicolon)?;
            Some(self.block()?)
        } else {
            None
        };

        self.match_(TokenType::Semicolon)?;

        Ok(Expr::if_else(cond, then, else_clause))
    }

    pub fn parse_args(&mut self) -> ParseResult<Vec<Identifier>> {
        let mut params = vec![];
        while !self.check(TokenType::RightParen)? && !self.check(TokenType::EOF)? {
            params.push(self.parse_identifier()?);

            if !self.match_(TokenType::Comma)? {
                break;
            }
        }
        Ok(params)
    }

    pub fn parse_expr_statement(&mut self) -> ParseResult<Expr> {
        let expr = self.expression()?;
        self.match_(TokenType::Semicolon)?;
        Ok(expr)
    }

    pub fn expression(&mut self) -> ParseResult<Expr> {
        expr_parser::parse(self)
    }

    fn block(&mut self) -> ParseResult<BlockDecl> {
        let mut exprs = vec![];
        while !self.match_(TokenType::End)? {
            exprs.push(self.parse_top_level_expr()?);
        }

        self.match_(TokenType::Semicolon)?;

        Ok(exprs)
    }

    pub fn parse_identifier(&mut self) -> ParseResult<Identifier> {
        Ok(self.expect(TokenType::Identifier)?.source.to_string())
    }

    pub fn expect(&mut self, expect: TokenType) -> ParseResult<Token<'a>> {
        if self.check(expect.clone())? {
            return Ok(self.consume()?);
        }

        Err(ParserError::Expected(
            expect.clone(),                         // TODO: Clone
            self.peek_type()?.clone(),              // TODO: Clone
            self.peek()?.position.line.clone(), // TODO: Clone
        ))
    }

    pub fn consume(&mut self) -> ParseResult<Token<'a>> {
        self.tokens.pop().ok_or(ParserError::UnexpectedEOF)
    }

    pub fn peek(&self) -> ParseResult<&Token<'a>> {
        self.tokens.last().ok_or(ParserError::UnexpectedEOF)
    }

    pub fn peek_type(&self) -> ParseResult<&TokenType> {
        Ok(&self.peek()?.token_type)
    }

    pub fn match_(&mut self, token_type: TokenType) -> ParseResult<bool> {
        if !self.check(token_type)? {
            return Ok(false);
        }
        self.consume()?;
        Ok(true)
    }

    pub fn check(&self, token_type: TokenType) -> ParseResult<bool> {
        Ok(self.peek_type()? == &token_type)
    }

    pub fn is_eof(&self) -> ParseResult<bool> {
        Ok(self.check(TokenType::EOF)?)
    }
}
