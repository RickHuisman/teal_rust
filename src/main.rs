mod syntax;
mod codegen;

use anyhow::Result;
use wasmer::{Exports, Function, ImportObject, Instance, Store};
use wasmer::Module;
use wasmer::imports;
use wasmer::internals::WithoutEnv;
use wasmer::Value;
use crate::codegen::generate_assembly;
use crate::syntax::{lex, parse};

fn main() -> Result<()> {
    let code = r#"
    fun negate(x) {
        -x;
    }

    puts negate(3);
    5;
    "#;
    run(code)
}

fn run(source: &str) -> Result<()> {
    // Compile program.
    let mut tokens = lex(source).unwrap();
    let ast = parse(&mut tokens).unwrap();

    println!("{:?}", ast);

    // Generate wasm.
    let module_wat = generate_assembly(ast);
    println!("{}", module_wat);

    // Run wasm.
    let store = Store::default();
    let module = Module::new(&store, &module_wat)?;

    let log_func = Function::new_native(&store, log);

    let import_object = imports! {
        "env" => {
            "log" => log_func
        }
    };

    let instance = Instance::new(&module, &import_object)?;

    let main = instance.exports.get_function("main")?;
    let result = main.call(&[])?;
    println!("{:?}", result);

    Ok(())
}

fn log(n: f64) {
    println!("{}", n);
}