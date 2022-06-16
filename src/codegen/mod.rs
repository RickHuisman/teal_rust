mod watwriter;

use crate::codegen::watwriter::{Function, Global, Module, Statement, ValueType};
use crate::syntax::ast::{BinaryOperator, Expr, LiteralExpr, Program};

pub fn generate_assembly(program: Program) -> String {
    let init_fun = Function::new("init".to_string(), Some(ValueType::F64), vec![]);

    let mut compiler = Compiler {
        module: Module::new(),
        current: init_fun,
    };

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
                value_type: ValueType::F64
            };

            compiler.module.add_global(global);

            // Generate initializer.
            generate_expr(compiler, *initializer);

            let s = Statement::String(format!("global.set ${}", ident.clone()));
            compiler.current.add_statement(s);
        },
        Expr::LetGet { ident } => {
            let s = Statement::String(format!("global.get ${}", ident.clone()));
            compiler.current.add_statement(s);
        },
        Expr::LetSet { ident, expr } => {
            generate_expr(compiler, *expr);
            let s = Statement::String(format!("global.set ${}", ident));
            compiler.current.add_statement(s);
        },
        Expr::Puts { .. } => todo!(),
        Expr::IfElse { .. } => todo!(),
        Expr::Def { ident, decl } => {
            let init_clone = compiler.current.clone();

            let f = Function::new(ident, Some(ValueType::F64), vec![]);
            compiler.current = f;

            // Compile function expressions.
            for expr in decl.body {
                generate_expr(compiler, expr);
            }

            compiler.module.add_function(compiler.current.clone());

            compiler.current = init_clone;
        },
        Expr::Call { callee, args } => {

        },
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
    current: Function,
}

impl Compiler {
    pub fn to_wat(mut self) -> String {
        // self.module.add_function(self.current_function);
        self.module.to_wat()
    }
}
