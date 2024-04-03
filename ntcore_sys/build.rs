fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut build = cc::Build::new();
    build
        .file("ntcore/sources/ntcore_c.cpp")
        .cpp(true)
        .warnings(false)
        .flag_if_supported("-w") // clang, gcc
        .flag_if_supported("/w") // msvc
        .flag_if_supported("-Wno-psabi") // gcc
        .flag_if_supported("-std=c++20") // clang, gcc
        .flag_if_supported("/std:c++20") // msvc
        .include("ntcore/include");
    if let Some(wpiutil_headers) = std::env::var_os("DEP_WPIUTIL_INCLUDE") {
        build.include(wpiutil_headers);
    }
    build.compile("ntcore");
    println!("cargo:rerun-if-changed=ntcore/");
    println!(
        "cargo:include={}/ntcore/include",
        std::env::current_dir()?.display()
    );
    Ok(())
}
