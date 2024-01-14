use std::{env, error::Error, sync::OnceLock};

use cargo_metadata::camino::{Utf8Path, Utf8PathBuf};

mod codegen;

static PROJECT_ROOT: OnceLock<Utf8PathBuf> = OnceLock::new();

pub fn project_root() -> &'static Utf8Path {
    PROJECT_ROOT
        .get_or_init(|| {
            cargo_metadata::MetadataCommand::new()
                .exec()
                .unwrap()
                .workspace_root
        })
        .as_path()
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
