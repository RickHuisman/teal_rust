mod syntax;
mod codegen;

use anyhow::Result;
use wasmer::{Function, Instance, Store};
use wasmer::Module;
use wasmer::imports;
use wasmer::Value;
use crate::codegen::generate_assembly;
use crate::syntax::{lex, parse};

fn main() -> Result<()> {
    let code = r#"
    fun negate(x) {
        -x;
    }

    negate(3);
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

    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object)?;

    let main = instance.exports.get_function("main")?;
    let result = main.call(&[])?;
    println!("{:?}", result);

    Ok(())
}