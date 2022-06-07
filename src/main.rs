use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;

use anyhow::Result;
use wasmer::{Instance, Store};
use wasmer::Module;
use wasmer::imports;
use wasmer::Value;

fn main() -> Result<()> {
    let module_wat = r#"
    (module
    (type $t0 (func (param i32) (result i32)))
    (func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
        get_local $p0
        i32.const 1
        i32.add))
    "#;

    let store = Store::default();
    let module = Module::new(&store, &module_wat)?;

    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object)?;

    let add_one = instance.exports.get_function("add_one")?;
    let result = add_one.call(&[Value::I32(42)])?;
    println!("{:?}", result);

    Ok(())
}