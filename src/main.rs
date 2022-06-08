mod syntax;
mod codegen;

use anyhow::Result;
use wasmer::{Instance, Store};
use wasmer::Module;
use wasmer::imports;
use wasmer::Value;
use crate::codegen::generate_assembly;
use crate::syntax::{lex, parse};

fn main() -> Result<()> {
    let code = r#"
    let x = 10;
    x * x + 2;
    "#;
    run(code)
}

fn run(source: &str) -> Result<()> {
    let mut tokens = lex(source).unwrap();
    let ast = parse(&mut tokens).unwrap();

    println!("{:?}", ast);

    let module_wat = generate_assembly(ast);
    println!("{}", module_wat);

    // Run wasmer.
    let store = Store::default();
    let module = Module::new(&store, &module_wat)?;

    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object)?;

    let add_one = instance.exports.get_function("init")?;
    let result = add_one.call(&[])?;
    println!("{:?}", result);

    Ok(())
}