mod syntax;
mod codegen;

use anyhow::Result;
use teal::run;

fn main() -> Result<()> {
    let code = r#"
    print true != false;
    "#;
    run(code)
}
