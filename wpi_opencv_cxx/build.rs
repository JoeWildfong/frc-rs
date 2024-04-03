use std::path::PathBuf;

fn unwrap_all_glob(pattern: &str) -> impl Iterator<Item = PathBuf> {
    glob::glob(pattern).unwrap().map(|path| path.unwrap())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut build = cxx_build::bridge("src/lib.rs");
    build
        .files(unwrap_all_glob("opencv/sources/**/*.cpp"))
        .warnings(false)
        .flag_if_supported("-w") // clang, gcc
        .flag_if_supported("/w") // msvc
        .flag_if_supported("-std=c++20") // clang, gcc
        .flag_if_supported("/std:c++20") // msvc
        .define("__OPENCV_BUILD", "")
        .include("opencv/include")
        .include("opencv/include/opencv2");
    build.compile("opencv");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=opencv/");
    println!(
        "cargo:include={}/opencv/include",
        std::env::current_dir()?.display()
    );
    Ok(())
}
