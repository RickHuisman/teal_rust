#[derive(Clone)]
pub struct Module {
    pub globals: Vec<Global>,
    pub functions: Vec<Function>,
}

impl Module {
    pub fn new() -> Self {
        Self { globals: vec![], functions: vec![] }
    }

    pub fn add_global(&mut self, global: Global) {
        self.globals.push(global);
    }

    pub fn add_function(&mut self, fun: Function) {
        self.functions.push(fun);
    }

    pub fn to_wat(self) -> String {
        let mut prefix = "(module ".to_string();

        // Globals.
        for g in self.globals {
            prefix += &g.to_wat();
        }

        // Functions.
        for f in self.functions {
            prefix += &f.to_wat();
        }

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
        format!("(global ${} (mut f64) (f64.const 0))\n", self.name)
    }
}

#[derive(Clone)]
pub enum Statement {
    Const(f64),
    String(String),
}

impl Statement {
    fn to_wat(self) -> String {
        return match self {
            Statement::Const(c) => {
                format!("f64.const {}\n", c)
            }
            Statement::String(s) => {
                format!("{}\n", s)
            }
        }
    }
}

#[derive(Clone)]
pub struct Function {
    name: String,
    return_type: Option<ValueType>,
    statements: Vec<Statement>,
}

impl Function {
    pub fn new(name: String, return_type: Option<ValueType>, statements: Vec<Statement>) -> Self {
        Self { name, return_type, statements }
    }

    pub fn new_empty(name: &str) -> Self {
        Self { name: name.to_string(), return_type: None, statements: vec![] }
    }

    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }

    pub fn to_wat(self) -> String {
        let mut prefix = "(func ".to_string();

        // Export.
        prefix += &format!("${}\n", self.name);

        // Return type.
        if self.return_type.is_some() {
            prefix += " (result f64)\n";
        }

        // Statements.
        for s in self.statements {
            prefix += &s.to_wat();
        }

        prefix += ")";

        prefix += &format!("(export \"{}\" (func ${}))", self.name, self.name);

        prefix.to_string()
    }
}