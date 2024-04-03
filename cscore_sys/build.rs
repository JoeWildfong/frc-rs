use std::path::PathBuf;

fn unwrap_all_glob(pattern: &str) -> impl Iterator<Item = PathBuf> {
    glob::glob(pattern).unwrap().map(|path| path.unwrap())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut build = cc::Build::new();
    build
        .files(unwrap_all_glob("cscore/sources/**/*.cpp"))
        .cpp(true)
        .warnings(false)
        .flag_if_supported("-w") // clang, gcc
        .flag_if_supported("/w") // msvc
        .flag_if_supported("-Wno-psabi") // gcc
        .flag_if_supported("-std=c++20") // clang, gcc
        .flag_if_supported("/std:c++20") // msvc
        .include("cscore/include");
    if let Some(wpiutil_headers) = std::env::var_os("DEP_WPIUTIL_INCLUDE") {
        build.include(wpiutil_headers);
    }
    if let Some(wpinet_headers) = std::env::var_os("DEP_WPINET_INCLUDE") {
        build.include(wpinet_headers);
    }
    build.compile("cscore");
    println!("cargo:rerun-if-changed=cscore/");
    println!(
        "cargo:include={}/cscore/include",
        std::env::current_dir()?.display()
    );
    Ok(())
}
