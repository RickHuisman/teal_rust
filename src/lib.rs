mod syntax;
mod codegen;

use std::io::{Cursor, Write};
use std::sync::{Arc, Mutex};
use anyhow::Result;
use wasmer::{Function, Instance, Store};
use wasmer::Module;
use wasmer::imports;
use wasmer::WasmerEnv;
use crate::codegen::generate_assembly;
use crate::syntax::{lex, parse};

#[derive(WasmerEnv, Clone)]
struct Env {
    output: Arc<Mutex<Vec<String>>>,
}

pub fn run(source: &str) -> Result<()> {
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
    main.call(&[])?;

    Ok(())
}

fn log(n: f64) {
    println!("{}", n);
}

pub fn run_with_output(source: &str) -> Result<Vec<String>> {
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

    let shared_counter2: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

    let log_func = Function::new_native_with_env(&store, Env { output: shared_counter2.clone() }, log);

    fn log(env: &Env, n: f64) {
        let mut output_ref = env.output.lock().unwrap();
        output_ref.push(n.to_string());
        // println!("{}", n);
    }

    let import_object = imports! {
        "env" => {
            "log" => log_func
        }
    };

    let instance = Instance::new(&module, &import_object)?;

    let main = instance.exports.get_function("main")?;
    main.call(&[])?;

    let foobar = &shared_counter2.lock().unwrap();
    Ok((*foobar).to_vec())
}