mod watwriter;

use crate::codegen::watwriter::{Function, FunctionType, Global, Module, Statement, ValueType};
use crate::syntax::ast::{BinaryOperator, Expr, FunDecl, Identifier, LiteralExpr, Program, UnaryOperator};

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
        Expr::Block(_) => todo!(),
        Expr::Binary { left, op, right } => generate_binary(compiler, left, op, right),
        Expr::Unary { op, expr } => generate_unary(compiler, op, expr),
        Expr::LetAssign { ident, initializer } => generate_let_assign(compiler, ident, initializer),
        Expr::LetGet { ident } => generate_let_get(compiler, ident),
        Expr::LetSet { ident, expr } => generate_let_set(compiler, ident, expr),
        Expr::Puts { value } => generate_puts(compiler, value),
        Expr::IfElse { .. } => todo!(),
        Expr::Def { ident, decl } => generate_fun(compiler, ident, decl),
        Expr::Call { callee, args } => generate_call(compiler, callee, args),
        Expr::Literal(l) => generate_literal(compiler, l),
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

fn generate_puts(compiler: &mut Compiler, value: Box<Expr>) {
    generate_expr(compiler, *value);

    let s = Statement::String("call $log".to_string());

    compiler.current.add_statement(s);
}

fn generate_binary(compiler: &mut Compiler, left: Box<Expr>, op: BinaryOperator, right: Box<Expr>) {
    generate_expr(compiler, *left);
    generate_expr(compiler, *right);

    generate_binary_op(compiler, op);
}

fn generate_binary_op(compiler: &mut Compiler, op: BinaryOperator) {
    let operator = match op {
        BinaryOperator::Subtract => "f64.sub",
        BinaryOperator::Add => "f64.add",
        BinaryOperator::Divide => "f64.div",
        BinaryOperator::Multiply => "f64.mul",
        _ => todo!(),
    }.to_string();

    compiler.current.add_statement(Statement::String(operator));
}

fn generate_unary(compiler: &mut Compiler, op: UnaryOperator, expr: Box<Expr>) {
    generate_expr(compiler, *expr);

    match op {
        UnaryOperator::Negate => {
            compiler.current.add_statement(Statement::String("f64.neg".to_string()));
        },
        _ => todo!(),
    }
}

fn generate_fun(compiler: &mut Compiler, ident: Identifier, decl: FunDecl) {
    let main_clone = compiler.current.clone();

    let f = Function::new(ident, decl.args, Some(ValueType::F64), vec![], FunctionType::Function);
    compiler.current = f;

    // Compile function expressions.
    for expr in decl.body {
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
    match l {
        LiteralExpr::Number(n) => {
            compiler.current.add_statement(Statement::Const(n));
        }
        _ => todo!()
    }
}

#[derive(Clone)]
struct Compiler {
    module: Module,
    globals: Vec<Identifier>,
    current: Function,
}

impl Compiler {
    pub fn new() -> Self {
        let main_fun = Function::new("main".to_string(), vec![], None, vec![], FunctionType::Script);
        Self {
            module: Module::new(),
            globals: vec![],
            current: main_fun,
        }
    }

    pub fn is_local(&self, local: &Identifier) -> bool {
        if self.current.params.contains(local) {
            return true;
        }

        self.current.locals.contains(local)
    }

    pub fn to_wat(mut self) -> String {
        self.module.to_wat()
    }
}
