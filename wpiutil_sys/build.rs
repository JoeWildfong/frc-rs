fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut build = cc::Build::new();
    build
        .files(glob::glob("wpiutil/sources/**/*.cpp")?.map(Result::unwrap))
        .cpp(true)
        .warnings(false)
        .flag_if_supported("-w") // clang, gcc
        .flag_if_supported("/w") // msvc
        .flag_if_supported("-Wno-psabi") // gcc
        .flag_if_supported("-std=c++20") // clang, gcc
        .flag_if_supported("/std:c++20") // msvc
        .include("wpiutil/headers");
    if let Some(ni_headers) = std::env::var_os("DEP_NI_FRC_INCLUDE") {
        build.include(ni_headers);
    }
    build.compile("wpiutil");
    println!("cargo:rerun-if-changed=wpiutil/");
    println!(
        "cargo:include={}/wpiutil/headers",
        std::env::current_dir()?.display()
    );
    Ok(())
}
