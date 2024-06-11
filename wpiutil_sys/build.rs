fn main() {
    let build = wpilib_build::Build {
        maven_name: "wpiutil",
        version: include_str!("version.txt"),
        base_name: "wpiutil",
        srcs: vec!["wpiutil/sources/**/*.cpp"],
        include: "wpiutil/headers",
        include_env_vars: &["DEP_NI_FRC_INCLUDE"],
    };
    build.build(wpilib_build::ArtifactType::Static);
}
