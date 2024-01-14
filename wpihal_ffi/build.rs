use std::path::PathBuf;

fn unwrap_all_glob(pattern: &str) -> impl Iterator<Item = PathBuf> {
    glob::glob(pattern).unwrap().map(|path| path.unwrap())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_roborio = std::env::var("TARGET").unwrap().as_str() == "armv7-unknown-linux-gnueabi";
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .warnings(false)
        .flag_if_supported("-w") // clang, gcc
        .flag_if_supported("/w") // msvc
        .flag_if_supported("-Wno-psabi") // gcc
        .flag_if_supported("-std=c++20") // clang, gcc
        .flag_if_supported("/std:c++20") // msvc
        .files(unwrap_all_glob("wpihal/sources/*.cpp"))
        .files(unwrap_all_glob("wpihal/sources/cpp/**/*.cpp"))
        .files(unwrap_all_glob("wpihal/sources/handles/**/*.cpp"))
        .include("wpihal/headers");
    if let Some(ni_headers) = std::env::var_os("DEP_NI_FRC_INCLUDE") {
        build.include(ni_headers);
    }
    if let Some(wpiutil_headers) = std::env::var_os("DEP_WPIUTIL_INCLUDE") {
        build.include(wpiutil_headers);
    }

    if is_roborio {
        build.files(unwrap_all_glob("wpihal/sources/athena/**/*.cpp"));
    } else {
        build.files(unwrap_all_glob("wpihal/sources/sim/**/*.cpp"));
    }

    build.compile("wpihal");
    println!("cargo:rustc-link-lib=wpihal");

    // relink all dependencies of wpihal
    println!("cargo:rustc-link-lib=wpiutil");
    if is_roborio {
        println!("cargo:rustc-link-lib=visa");
        println!("cargo:rustc-link-lib=RoboRIO_FRC_ChipObject");
        println!("cargo:rustc-link-lib=FRC_NetworkCommunication");
        println!("cargo:rustc-link-lib=dylib:+verbatim=libNiFpgaLv.so.13");
        println!("cargo:rustc-link-lib=dylib:+verbatim=libnirio_emb_can.so.23");
    }

    println!(
        "cargo:include={}/wpihal/headers",
        std::env::current_dir()?.display()
    );
    Ok(())
}
