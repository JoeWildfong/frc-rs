fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut build = cc::Build::new();
    build
        .files(glob::glob("wpiutil/sources/**/*.cpp")?.into_iter().map(|a| a.unwrap()))
        .cpp(true)
        .flag("-std=c++20")
        .flag("-w") // disable warnings
        .flag_if_supported("-Wno-psabi")
        .include("wpiutil/headers");
    if let Some(ni_headers) = std::env::var_os("DEP_NI_FRC_INCLUDE") {
        build.include(ni_headers);
    }
    build.compile("wpiutil");
    println!("cargo:rustc-link-lib=wpiutil");
    println!("cargo:include={}/wpiutil/headers", std::env::current_dir()?.display());
    Ok(())
}
