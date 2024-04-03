fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut build = cxx_build::bridge("src/lib.rs");
    build
        .files(glob::glob("wpilibc/sources/**/*.cpp").unwrap().filter_map(Result::ok))
        .flag_if_supported("-std=c++20") // clang, gcc
        .flag_if_supported("/std:c++20") // msvc
        .include("wpilibc/include");
    if let Some(wpihal_headers) = std::env::var_os("DEP_WPIHAL_INCLUDE") {
        build.include(wpihal_headers);
    }
    if let Some(wpiutil_headers) = std::env::var_os("DEP_WPIUTIL_INCLUDE") {
        build.include(wpiutil_headers);
    }
    if let Some(wpinet_headers) = std::env::var_os("DEP_WPINET_INCLUDE") {
        build.include(wpinet_headers);
    }
    if let Some(wpimath_headers) = std::env::var_os("DEP_WPIMATH_INCLUDE") {
        build.include(wpimath_headers);
    }
    if let Some(ntcore_headers) = std::env::var_os("DEP_NTCORE_INCLUDE") {
        build.include(ntcore_headers);
    }
    build.compile("wpilib");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=wpilibc/");
    println!(
        "cargo:include={}/wpilibc/include",
        std::env::current_dir()?.display()
    );
    Ok(())
}
