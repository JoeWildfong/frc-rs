use std::{env, error::Error, path::{PathBuf, Path}};

mod codegen;

pub fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn print_help() {
    println!(
        "
Usage: Run with `cargo xtask <task>`, eg. `cargo xtask codegen`.
Tasks:
    codegen <crate>: Run bindgen for <crate> or all crates if none is specified.
"
    );
}


fn main() -> Result<(), Box<dyn Error>> {
    let task = env::args().nth(1);
    match task {
        None => print_help(),
        Some(t) => match t.as_str() {
            "--help" => print_help(),
            "codegen" => codegen::generate_bindings(env::args().nth(2))?,
            invalid => return Err(format!("Invalid task name: {invalid}").into()),
        },
    };
    Ok(())
}
