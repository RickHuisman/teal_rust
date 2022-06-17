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
    fun double(x) {
        x * 2;
    }

    if 3 < 2 {
        print double(4);
    }
    "#;
    run(code)
}