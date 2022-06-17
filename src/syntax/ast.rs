use crate::syntax::error::{ParserError, ParseResult};
use crate::syntax::token::TokenType;

// TODO: Change to &[Expr].
pub type Program = Vec<Expr>;
pub type Identifier = String;
pub type BlockDecl = Vec<Expr>;

#[derive(PartialEq, Debug)]
pub enum Expr {
    Block(Vec<Expr>),
    Binary {
        left: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOperator,
        expr: Box<Expr>,
    },
    LetAssign {
        ident: Identifier,
        initializer: Box<Expr>,
    },
    LetGet {
        ident: Identifier,
    },
    LetSet {
        ident: Identifier,
        expr: Box<Expr>,
    },
    Print {
        value: Box<Expr>,
    },
    IfElse {
        condition: Box<Expr>,
        then: Box<Expr>,
        else_: Option<Box<Expr>>,
    },
    Def {
        ident: Identifier,
        decl: FunDecl,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Literal(LiteralExpr),
}

impl Expr {
    pub fn binary(left: Expr, op: BinaryOperator, right: Expr) -> Self {
        Expr::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }

    pub fn unary(op: UnaryOperator, expr: Expr) -> Self {
        Expr::Unary { op, expr: Box::new(expr) }
    }

    pub fn let_assign(ident: Identifier, initializer: Expr) -> Self {
        Expr::LetAssign {
            ident,
            initializer: Box::new(initializer),
        }
    }

    pub fn let_get(ident: Identifier) -> Self {
        Expr::LetGet { ident }
    }

    pub fn let_set(ident: Identifier, expr: Expr) -> Self {
        Expr::LetSet {
            ident,
            expr: Box::new(expr),
        }
    }

    pub fn def(ident: Identifier, decl: FunDecl) -> Self {
        Expr::Def { ident, decl }
    }

    pub fn call(callee: Expr, args: Vec<Expr>) -> Self {
        Expr::Call { callee: Box::new(callee), args }
    }

    pub fn number(n: i32) -> Expr {
        Expr::Literal(LiteralExpr::Number(n))
    }

    pub fn string(s: String) -> Expr {
        Expr::Literal(LiteralExpr::String(s))
    }

    pub fn true_() -> Expr {
        Expr::Literal(LiteralExpr::True)
    }

    pub fn false_() -> Expr {
        Expr::Literal(LiteralExpr::False)
    }

    pub fn print(value: Expr) -> Self {
        Expr::Print { value: Box::new(value) }
    }

    pub fn if_else(condition: Expr, then: Expr, else_: Option<Expr>) -> Self {
        let else_boxed = match else_ {
            None => None,
            Some(e) => Some(Box::new(e)),
        };

        Expr::IfElse { condition: Box::new(condition), then: Box::new(then), else_: else_boxed }
    }
}

#[derive(PartialEq, Debug)]
pub enum LiteralExpr {
    Number(i32),
    String(String),
    True,
    False,
}

#[derive(PartialEq, Debug)]
pub enum BinaryOperator {
    Equal,
    BangEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Subtract,
    Add,
    Divide,
    Multiply,
}

impl BinaryOperator {
    pub fn from_token(token_type: &TokenType) -> ParseResult<BinaryOperator> {
        Ok(match token_type {
            TokenType::Minus => BinaryOperator::Subtract,
            TokenType::Plus => BinaryOperator::Add,
            TokenType::Star => BinaryOperator::Multiply,
            TokenType::Slash => BinaryOperator::Divide,
            TokenType::BangEqual => BinaryOperator::BangEqual,
            TokenType::Equal => BinaryOperator::Equal,
            TokenType::EqualEqual => BinaryOperator::Equal,
            TokenType::LessThan => BinaryOperator::LessThan,
            TokenType::LessThanEqual => BinaryOperator::LessThanEqual,
            TokenType::GreaterThan => BinaryOperator::GreaterThan,
            TokenType::GreaterThanEqual => BinaryOperator::GreaterThanEqual,
            _ => return Err(ParserError::ExpectedBinaryOperator(token_type.clone())),
        })
    }
}

#[derive(PartialEq, Debug)]
pub enum UnaryOperator {
    Negate,
    Not,
}

impl UnaryOperator {
    pub fn from_token(token_type: &TokenType) -> ParseResult<UnaryOperator> {
        Ok(match token_type {
            TokenType::Minus => UnaryOperator::Negate,
            TokenType::Bang => UnaryOperator::Not,
            _ => return Err(ParserError::ExpectedUnaryOperator(token_type.clone())),
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct FunDecl {
    pub args: Vec<Identifier>,
    pub body: BlockDecl,
}

impl FunDecl {
    pub fn new(args: Vec<Identifier>, body: BlockDecl) -> Self {
        FunDecl { args, body }
    }
}