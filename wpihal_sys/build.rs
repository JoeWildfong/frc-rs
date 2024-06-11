fn main() {
    let is_roborio = std::env::var("TARGET").unwrap().as_str() == "armv7-unknown-linux-gnueabi";
    let build = wpilib_build::Build {
        maven_name: "hal",
        version: include_str!("version.txt"),
        base_name: "wpihal",
        srcs: vec![
            "wpihal/sources/*.cpp",
            "wpihal/sources/cpp/**/*.cpp",
            "wpihal/sources/handles/**/*.cpp",
            if is_roborio { "wpihal/sources/athena/**/*.cpp" } else { "wpihal/sources/sim/**/*.cpp" }
        ],
        include: "wpihal/headers",
        include_env_vars: &["DEP_NI_FRC_INCLUDE", "DEP_WPIUTIL_INCLUDE"],
    };
    build.build(wpilib_build::ArtifactType::Static);

    // relink all dependencies of wpihal
    println!("cargo:rustc-link-lib=wpiutil");
    if is_roborio {
        println!("cargo:rustc-link-lib=visa");
        println!("cargo:rustc-link-lib=RoboRIO_FRC_ChipObject");
        println!("cargo:rustc-link-lib=FRC_NetworkCommunication");
        println!("cargo:rustc-link-lib=dylib:+verbatim=libNiFpgaLv.so.13");
        println!("cargo:rustc-link-lib=dylib:+verbatim=libnirio_emb_can.so.23");
    }
}
