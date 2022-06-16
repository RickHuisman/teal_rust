mod watwriter;

use crate::codegen::watwriter::{Function, FunctionType, Global, Module, Statement, ValueType};
use crate::syntax::ast::{BinaryOperator, Expr, Identifier, LiteralExpr, Program};

pub fn generate_assembly(program: Program) -> String {
    let mut compiler = Compiler::new();

    for expr in program {
        generate_expr(&mut compiler, expr);
    }

    // Add init function.
    compiler.module.add_function(compiler.current.clone());

    compiler.to_wat()
}

fn generate_expr(compiler: &mut Compiler, expr: Expr) {
    match expr {
        Expr::Block(_) => todo!(),
        Expr::Binary { left, op, right } => {
            generate_expr(compiler, *left);
            generate_expr(compiler, *right);

            generate_binary_op(compiler, op);
        }
        Expr::Unary { .. } => todo!(),
        Expr::LetAssign { ident, initializer } => {
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
        Expr::LetGet { ident } => {
            let s = if compiler.is_local(&ident) {
                // Local var.
                Statement::String(format!("local.get ${}", ident.clone()))
            } else {
                // Global var.
                Statement::String(format!("global.get ${}", ident.clone()))
            };

            compiler.current.add_statement(s);
        }
        Expr::LetSet { ident, expr } => {
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
        Expr::Puts { .. } => todo!(),
        Expr::IfElse { .. } => todo!(),
        Expr::Def { ident, decl } => {
            let init_clone = compiler.current.clone();

            let f = Function::new(ident, decl.args, Some(ValueType::F64), vec![], FunctionType::Function);
            compiler.current = f;

            // Compile function expressions.
            for expr in decl.body {
                generate_expr(compiler, expr);
            }

            compiler.module.add_function(compiler.current.clone());

            compiler.current = init_clone;
        }
        Expr::Call { callee, args } => {
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
        Expr::Literal(l) => {
            match l {
                LiteralExpr::Number(n) => {
                    compiler.current.add_statement(Statement::Const(n));
                }
                _ => todo!()
            }
        }
    }
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

#[derive(Clone)]
struct Compiler {
    module: Module,
    globals: Vec<Identifier>,
    current: Function,
}

impl Compiler {
    pub fn new() -> Self {
        let init_fun = Function::new("init".to_string(), vec![], Some(ValueType::F64), vec![], FunctionType::Script);
        Self {
            module: Module::new(),
            globals: vec![],
            current: init_fun,
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
