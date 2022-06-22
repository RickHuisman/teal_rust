use crate::syntax::ast::Identifier;

#[derive(Clone)]
pub struct Module {
    pub globals: Vec<Global>,
    pub data: Vec<String>,
    pub functions: Vec<Function>,
}

impl Module {
    pub fn new() -> Self {
        Self { globals: vec![], data: vec![], functions: vec![] }
    }

    pub fn add_global(&mut self, global: Global) {
        self.globals.push(global);
    }

    pub fn add_data(&mut self, str: String) {
        self.data.push(str);
    }

    pub fn add_function(&mut self, fun: Function) {
        self.functions.push(fun);
    }

    pub fn to_wat(self) -> String {
        let mut prefix = "(module\n".to_string();

        // Print function.
        prefix += "(import \"env\" \"log\" (func $log (param i32)))\n";

        // Memory.
        prefix += "(memory $mem 1)\n";

        // Globals.
        for g in self.globals {
            prefix += &g.to_wat();
        }

        // Data.
        for d in self.data {
            prefix += &format!("(data (i32.const 0) \"{}\")\n", d);
        }

        // Functions.
        for f in self.functions {
            prefix += &f.to_wat();
        }

        // Export main function.
        prefix += &format!("(export \"main\" (func $main))\n");

        // Export memory.
        prefix += &format!("(export \"memory\" (memory $mem))\n");

        prefix + ")"
    }
}

#[derive(Clone)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Clone)]
pub struct Global {
    pub name: String,
    pub mutable: bool,
    pub value_type: ValueType,
}

impl Global {
    pub fn to_wat(self) -> String {
        format!("(global ${} (mut i32) (i32.const 0))\n", self.name)
    }
}

pub type FunctionName = String;

#[derive(Clone)]
pub enum Statement {
    Const(i32),
    Call(FunctionName),
    String(String),
}

impl Statement {
    fn to_wat(self) -> String {
        return match self {
            Statement::Const(c) => {
                format!("i32.const {}\n", c)
            }
            Statement::String(s) => {
                format!("{}\n", s)
            }
            Statement::Call(f) => {
                format!("call ${}\n", f)
            }
        };
    }
}

#[derive(Clone, PartialEq)]
pub enum FunctionType {
    Function,
    Script,
}

#[derive(Clone)]
pub struct Function {
    name: String,
    pub params: Vec<Identifier>,
    return_type: Option<ValueType>,
    pub locals: Vec<Identifier>,
    statements: Vec<Statement>,
    pub function_type: FunctionType,
}

impl Function {
    pub fn new(
        name: String,
        params: Vec<Identifier>,
        return_type: Option<ValueType>,
        statements: Vec<Statement>,
        function_type: FunctionType) -> Self {
        Self { name, params, return_type, locals: vec![], statements, function_type }
    }

    pub fn add_local(&mut self, local: Identifier) {
        self.locals.push(local);
    }

    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }

    pub fn to_wat(self) -> String {
        let mut prefix = "(func ".to_string();

        prefix += &format!("${} ", self.name);

        // Params.
        for p in self.params {
            prefix += &format!("(param ${} i32) ", p);
        }
        prefix += "\n";

        // Return type.
        if self.return_type.is_some() {
            prefix += "(result i32)\n";
        }

        // Local declarations.
        for l in self.locals {
            prefix += &format!("(local ${} i32)\n", l);
        }

        // Statements.
        for s in self.statements {
            prefix += &s.to_wat();
        }

        prefix += ")\n";

        prefix.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_to_wat() {
        let m = Module::new();
        assert_eq!(m.to_wat(), "(module)");
    }

    // #[test]
    // fn fun_to_wat() {
    //     let f = Function::new_empty("foobar");
    //     assert_eq!(f.to_wat(), "(func $foobar)");
    // }
}