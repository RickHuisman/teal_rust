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

    pub fn declaration(&mut self) -> ParseResult<Expr> {
        match self.peek_type()? {
            TokenType::Let => self.parse_let(),
            TokenType::Fun => self.parse_fun(),
            TokenType::Print => self.parse_print(),
            TokenType::If => self.parse_if(),
            TokenType::LeftBrace => self.parse_block(),
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
            Expr::number(4) // TODO: NIL.
            // Expr::Literal(LiteralExpr::Nil)
        };

        Ok(Expr::let_assign(ident, initializer))
    }

    fn parse_fun(&mut self) -> ParseResult<Expr> {
        self.expect(TokenType::Fun)?;

        let ident = self.parse_identifier()?;

        self.expect(TokenType::LeftParen)?;
        let args = self.parse_args()?;
        self.expect(TokenType::RightParen)?;

        self.expect(TokenType::LeftBrace)?;

        let body = self.block()?;

        let fun_decl = FunDecl::new(args, body);
        Ok(Expr::def(ident, fun_decl))
    }

    fn parse_print(&mut self) -> ParseResult<Expr> {
        self.expect(TokenType::Print)?;
        let expr = self.parse_expr_statement()?;
        Ok(Expr::print(expr))
    }

    fn parse_if(&mut self) -> ParseResult<Expr> {
        self.expect(TokenType::If)?;

        let condition = self.expression()?;
        let then = self.declaration()?;

        let else_ = if self.match_(TokenType::Else)? {
            Some(self.declaration()?)
        } else {
            None
        };

        Ok(Expr::if_else(condition, then, else_))
    }

    fn parse_block(&mut self) -> ParseResult<Expr> {
        self.expect(TokenType::LeftBrace)?;

        let mut expressions = vec![];

        while !self.check(TokenType::RightBrace)? && !self.check(TokenType::EOF)? {
            expressions.push(self.declaration()?);
        }

        self.expect(TokenType::RightBrace)?;

        Ok(Expr::Block(expressions))
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
        while !self.check(TokenType::RightBrace)? && !self.check(TokenType::EOF)? {
            exprs.push(self.declaration()?);
        }

        self.expect(TokenType::RightBrace)?;

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
