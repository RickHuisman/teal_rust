mod syntax;
mod codegen;

use std::sync::{Arc, Mutex};
use anyhow::Result;
use wasmer::{Function, ImportObject, Instance, Memory, MemoryType, MemoryView, Pages, Store};
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

    // let memory = Memory::new(&store, MemoryType::new(1, None, false)).unwrap();
    let log_func = Function::new_native(&store, log);

    let import_object = imports! {
        "env" => {
            "log" => log_func,
            // "mem" => memory.clone(),
        }
    };

    // let import_object = get_import_object(store);

    let instance = Instance::new(&module, &import_object)?;

    let main = instance.exports.get_function("main")?;
    main.call(&[])?;

    // println!("{:?}", memory);

    // Without synchronization.
    // let view: MemoryView<u8> = memory.view();
    // for byte in view[0x1000 .. 0x1010].iter().map(Cell::get) {
    //     println!("byte: {}", byte);
    // }

// With synchronization.
//     let atomic_view = view.atomically();
//     for byte in atomic_view[0x1000 .. 0x1010].iter().map(|atom| atom.load(Ordering::SeqCst)) {
//         println!("byte: {}", byte);
//     }

    let memory = instance.exports.get_memory("memory")?;

    // println!("Querying memory size... {:?}", memory.size());
    // assert_eq!(memory.size().bytes(), Bytes::from(65536 as usize));
    // assert_eq!(memory.data_size(), 65536);

    Ok(())
}

fn get_import_object(store: Store) -> ImportObject {
    let memory = Memory::new(&store, MemoryType::new(1, None, false)).unwrap();
    let log_func = Function::new_native(&store, log);

    return imports! {
        "env" => {
            "log" => log_func,
            "mem" => memory,
        }
    }
}

fn log(n: i32) {
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

    fn log(env: &Env, n: i32) {
        let mut output_ref = env.output.lock().unwrap();
        output_ref.push(n.to_string());
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