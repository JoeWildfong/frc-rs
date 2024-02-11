use std::path::PathBuf;

fn unwrap_all_glob(pattern: &str) -> impl Iterator<Item = PathBuf> {
    glob::glob(pattern).unwrap().map(|path| path.unwrap())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cxx_build::bridge("src/lib.rs")
        .files(unwrap_all_glob("wpinet/sources/**/*.cpp"))
        .flag_if_supported("-std=c++20") // clang, gcc
        .flag_if_supported("/std:c++20") // msvc
        .include("wpinet/headers")
        .compile("wpinet");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=wpinet/");
    println!(
        "cargo:include={}/wpinet/headers",
        std::env::current_dir()?.display()
    );
    Ok(())
}
