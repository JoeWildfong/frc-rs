use std::path::PathBuf;

fn unwrap_all_glob(pattern: &str) -> impl Iterator<Item = PathBuf> {
    glob::glob(pattern).unwrap().map(|path| path.unwrap())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut build = cxx_build::bridge("src/lib.rs");
    build
        .files(unwrap_all_glob("wpinet/sources/**/*.cpp"))
        .warnings(false)
        .flag_if_supported("-w") // clang, gcc
        .flag_if_supported("/w") // msvc
        .flag_if_supported("-std=c++20") // clang, gcc
        .flag_if_supported("/std:c++20") // msvc
        .include("wpinet/include");
    if let Some(wpiutil_headers) = std::env::var_os("DEP_WPIUTIL_INCLUDE") {
        build.include(wpiutil_headers);
    }
    build.compile("wpinet");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=wpinet/");
    println!(
        "cargo:include={}/wpinet/include",
        std::env::current_dir()?.display()
    );
    Ok(())
}
