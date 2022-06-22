mod watwriter;

use crate::codegen::watwriter::{Function, FunctionType, Global, Module, Statement, ValueType};
use crate::syntax::ast::{BinaryOperator, BlockDecl, Expr, Identifier, LiteralExpr, Program, UnaryOperator};

pub fn generate_assembly(program: Program) -> String {
    let mut compiler = Compiler::new();

    for expr in program {
        generate_expr(&mut compiler, expr);
    }

    // Add main function.
    compiler.module.add_function(compiler.current.clone());

    compiler.to_wat()
}

fn generate_expr(compiler: &mut Compiler, expr: Expr) {
    match expr {
        Expr::Block(expressions) => generate_block(compiler, expressions),
        Expr::Binary { left, op, right } => generate_binary(compiler, left, op, right),
        Expr::Unary { op, expr } => generate_unary(compiler, op, expr),
        Expr::LetAssign { ident, initializer } => generate_let_assign(compiler, ident, initializer),
        Expr::LetGet { ident } => generate_let_get(compiler, ident),
        Expr::LetSet { ident, expr } => generate_let_set(compiler, ident, expr),
        Expr::Print { value } => generate_print(compiler, value),
        Expr::IfElse { condition, then, else_ } => generate_if_else(compiler, condition, then, else_),
        Expr::Fun { ident, params, body, } => generate_fun(compiler, ident, params, body),
        Expr::Call { callee, args } => generate_call(compiler, callee, args),
        Expr::Literal(l) => generate_literal(compiler, l),
    }
}

fn generate_block(compiler: &mut Compiler, expressions: Vec<Expr>) {
    for e in expressions {
        generate_expr(compiler, e);
    }
}

fn generate_let_assign(compiler: &mut Compiler, ident: Identifier, initializer: Box<Expr>) {
    let global = Global {
        name: ident.clone(),
        mutable: true,
        value_type: ValueType::F64,
    };

    compiler.module.add_global(global);

    // Generate initializer.
    generate_expr(compiler, *initializer);

    if compiler.current.function_type == FunctionType::Script {
        // Global var.
        let s = Statement::String(format!("global.set ${}", ident.clone()));
        compiler.current.add_statement(s);
    } else {
        // Local var.
        compiler.current.add_local(ident.clone());

        let s = Statement::String(format!("local.set ${}", ident.clone()));
        compiler.current.add_statement(s);
    };
}

fn generate_let_get(compiler: &mut Compiler, ident: Identifier) {
    let s = if compiler.is_local(&ident) {
        // Local var.
        Statement::String(format!("local.get ${}", ident.clone()))
    } else {
        // Global var.
        Statement::String(format!("global.get ${}", ident.clone()))
    };

    compiler.current.add_statement(s);
}

fn generate_let_set(compiler: &mut Compiler, ident: Identifier, expr: Box<Expr>) {
    generate_expr(compiler, *expr);

    let s = if compiler.is_local(&ident) {
        // Local var.
        Statement::String(format!("local.set ${}", ident.clone()))
    } else {
        // Global var.
        Statement::String(format!("global.set ${}", ident.clone()))
    };

    compiler.current.add_statement(s);
}

fn generate_print(compiler: &mut Compiler, value: Box<Expr>) {
    generate_expr(compiler, *value);

    let s = Statement::String("call $log".to_string());

    compiler.current.add_statement(s);
}

fn generate_if_else(compiler: &mut Compiler, condition: Box<Expr>, then: Box<Expr>, else_: Option<Box<Expr>>) {
    generate_expr(compiler, *condition);

    let if_ = r#"(if (then"#;

    compiler.current.add_statement(Statement::String(if_.to_string()));

    // Generate then.
    generate_expr(compiler, *then);

    let then_ = r#") (else"#;
    compiler.current.add_statement(Statement::String(then_.to_string()));

    // Generate else.
    if else_.is_some() {
        generate_expr(compiler, *else_.unwrap());
    }

    compiler.current.add_statement(Statement::String("))".to_string()));
}

fn generate_binary(compiler: &mut Compiler, left: Box<Expr>, op: BinaryOperator, right: Box<Expr>) {
    generate_expr(compiler, *left);
    generate_expr(compiler, *right);

    generate_binary_op(compiler, op);
}

fn generate_binary_op(compiler: &mut Compiler, op: BinaryOperator) {
    let operator = match op {
        BinaryOperator::Subtract => "i32.sub",
        BinaryOperator::Add => "i32.add",
        BinaryOperator::Divide => "i32.div_u",
        BinaryOperator::Multiply => "i32.mul",
        BinaryOperator::Equal => "i32.eq",
        BinaryOperator::BangEqual => "i32.ne",
        BinaryOperator::GreaterThan => "i32.gt_u",
        BinaryOperator::GreaterThanEqual => "i32.ge_u",
        BinaryOperator::LessThan => "i32.lt_u",
        BinaryOperator::LessThanEqual => "i32.le_u"
    }.to_string();

    compiler.current.add_statement(Statement::String(operator));
}

fn generate_unary(compiler: &mut Compiler, op: UnaryOperator, expr: Box<Expr>) {
    generate_expr(compiler, *expr);

    match op {
        UnaryOperator::Negate => {
            compiler.current.add_statement(Statement::String("i32.neg".to_string()));
        },
        UnaryOperator::Not => todo!()
    }
}

fn generate_fun(compiler: &mut Compiler, ident: Identifier, params: Vec<Identifier>, body: BlockDecl) {
    let main_clone = compiler.current.clone();

    let f = Function::new(ident, params, Some(ValueType::I32), vec![], FunctionType::Function);
    compiler.current = f;

    // Compile function expressions.
    for expr in body {
        generate_expr(compiler, expr);
    }

    compiler.module.add_function(compiler.current.clone());

    compiler.current = main_clone;
}

fn generate_call(compiler: &mut Compiler, callee: Box<Expr>, args: Vec<Expr>) {
    // Generate args.
    for a in args {
        generate_expr(compiler, a);
    }

    let fun_name = match *callee {
        Expr::LetGet { ident } => ident,
        _ => todo!()
    };

    let call = Statement::Call(fun_name);
    compiler.current.add_statement(call);
}

fn generate_literal(compiler: &mut Compiler, l: LiteralExpr) {
    let statement = match l {
        LiteralExpr::Number(n) => Statement::Const(n),
        LiteralExpr::String(str) => Statement::String("i32.const 0".to_string()),
        LiteralExpr::True => Statement::Const(1),
        LiteralExpr::False => Statement::Const(0),
    };

    compiler.current.add_statement(statement);
}

#[derive(Clone)]
struct Compiler {
    module: Module,
    current: Function,
}

impl Compiler {
    pub fn new() -> Self {
        let main_fun = Function::new("main".to_string(), vec![], None, vec![], FunctionType::Script);
        Self {
            module: Module::new(),
            current: main_fun,
        }
    }

    pub fn is_local(&self, local: &Identifier) -> bool {
        if self.current.params.contains(local) {
            return true;
        }

        self.current.locals.contains(local)
    }

    pub fn to_wat(self) -> String {
        self.module.to_wat()
    }
}
