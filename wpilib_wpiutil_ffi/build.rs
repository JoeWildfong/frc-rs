use std::path::PathBuf;
use wpilib_artifact_download::{WPILIB_VERSION, WpilibArtifactInfo};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let object_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set")).join("objects");
    let artifact_info = WpilibArtifactInfo::from_target(std::env::var("TARGET")?.as_str());

    wpilib_artifact_download::download_and_extract_zip(
        &format!("https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first/wpiutil/wpiutil-cpp/{WPILIB_VERSION}/wpiutil-cpp-{WPILIB_VERSION}-{}.zip", artifact_info.platform_name), 
        &object_dir
    )?;
    println!("cargo:rustc-link-lib=wpiutil");

    let link_dir = object_dir.join(artifact_info.object_path_in_zip).join("shared");
    println!("cargo:rustc-link-search={}", link_dir.display());

    Ok(())
}
