pub use cc;

use std::path::PathBuf;

fn glob_all(pattern: &str) -> impl Iterator<Item = PathBuf> {
    glob::glob(pattern).unwrap().map(Result::unwrap)
}

pub fn build(b: &super::Build) {
    let mut cc = cc::Build::new();
    cc.cpp(true)
        .warnings(false)
        .flag_if_supported("-w") // clang, gcc
        .flag_if_supported("/w") // msvc
        .flag_if_supported("-Wno-psabi") // gcc
        .flag_if_supported("-std=c++20") // clang, gcc
        .flag_if_supported("/std:c++20") // msvc
        .include(b.include);
    for src in b.srcs.iter() {
        cc.files(glob_all(src));
    }
    for var in b.include_env_vars {
        if let Some(p) = std::env::var_os(var) {
            cc.include(p);
        }
    }
    cc.compile(b.base_name);
    println!("cargo:rerun-if-changed={}/", b.base_name);
    println!(
        "cargo:include={}/{}",
        std::env::current_dir().unwrap().display(),
        b.include
    );
}
