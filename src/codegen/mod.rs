mod watwriter;

use crate::codegen::watwriter::{Function, Global, Module, Statement, ValueType};
use crate::syntax::ast::{BinaryOperator, Expr, LiteralExpr, Program};

pub fn generate_assembly(program: Program) -> String {
    let mut compiler = Compiler {
        module: Module::new(),
        current_function: Function::new("init", Some(ValueType::F64), vec![]),
    };

    for expr in program {
        generate_expr(&mut compiler, expr);
    }

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
            compiler.current_function.add_statement(s);
            // let s2 = Statement::String(format!("global.get ${}", ident.clone()));
            // compiler.current_function.add_statement(s2);
        },
        Expr::LetGet { ident } => {
            let s2 = Statement::String(format!("global.get ${}", ident.clone()));
            compiler.current_function.add_statement(s2);

        },
        Expr::LetSet { .. } => todo!(),
        Expr::Puts { .. } => todo!(),
        Expr::IfElse { .. } => todo!(),
        Expr::Def { .. } => todo!(),
        Expr::Call { .. } => todo!(),
        Expr::Literal(l) => {
            match l {
                LiteralExpr::Number(n) => {
                    compiler.current_function.add_statement(Statement::Const(n));
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

    compiler.current_function.add_statement(Statement::String(operator));
}

struct Compiler {
    module: Module,
    current_function: Function,
}

impl Compiler {
    pub fn to_wat(mut self) -> String {
        self.module.add_function(self.current_function);

        self.module.to_wat()
    }
}
