use std::path::PathBuf;

fn unwrap_all_glob(pattern: &str) -> impl Iterator<Item = PathBuf> {
    glob::glob(pattern).unwrap().map(|path| path.unwrap())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cxx_build::bridge("src/lib.rs")
        .files(unwrap_all_glob("wpilibc/sources/**/*.cpp"))
        .flag_if_supported("-std=c++20") // clang, gcc
        .flag_if_supported("/std:c++20") // msvc
        .include("wpilibc/headers")
        .include(std::env::var("DEP_WPIHAL_INCLUDE")?)
        .compile("wpilib");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=wpilibc/");
    println!(
        "cargo:include={}/wpilibc/headers",
        std::env::current_dir()?.display()
    );
    Ok(())
}
