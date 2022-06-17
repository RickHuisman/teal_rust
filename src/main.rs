mod syntax;
mod codegen;

use anyhow::Result;
use wasmer::{Exports, Function, ImportObject, Instance, Store};
use wasmer::Module;
use wasmer::imports;
use wasmer::internals::WithoutEnv;
use wasmer::Value;
use teal::run;
use crate::codegen::generate_assembly;
use crate::syntax::{lex, parse};

fn main() -> Result<()> {
    let code = r#"
    fun negate(x) {
        -x;
    }

    puts negate(3);
    "#;
    run(code)
}