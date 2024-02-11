use std::path::PathBuf;

fn unwrap_all_glob(pattern: &str) -> impl Iterator<Item = PathBuf> {
    glob::glob(pattern).unwrap().map(|path| path.unwrap())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut build = cxx_build::bridge("src/lib.rs");
    build
        .files(unwrap_all_glob("wpimath/sources/**/*.cpp"))
        .warnings(false)
        .flag_if_supported("-w") // clang, gcc
        .flag_if_supported("/w") // msvc
        .flag_if_supported("-std=c++20") // clang, gcc
        .flag_if_supported("/std:c++20") // msvc
        .include("wpimath/include")
        .include("wpimath/include/wpimath/protobuf");
    if let Some(wpiutil_headers) = std::env::var_os("DEP_WPIUTIL_INCLUDE") {
        build.include(wpiutil_headers);
    }
    build.compile("wpimath");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=wpimath/");
    println!(
        "cargo:include={}/wpimath/include",
        std::env::current_dir()?.display()
    );
    Ok(())
}
