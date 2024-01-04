use std::path::PathBuf;
use wpilib_artifact_download::{WpilibArtifactInfo, NI_VERSION, WPILIB_VERSION};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let object_dir =
        PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR not set")).join("objects");
    let artifact_info = WpilibArtifactInfo::from_target(std::env::var("TARGET")?.as_str());

    wpilib_artifact_download::download_and_extract_zip(
        &format!("https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first/hal/hal-cpp/{WPILIB_VERSION}/hal-cpp-{WPILIB_VERSION}-{}.zip", artifact_info.platform_name), 
        &object_dir
    )?;
    println!("cargo:rustc-link-lib=wpiHal");
    println!("cargo:rustc-link-lib=wpiutil");

    if artifact_info.platform_name == "linuxathena" {
        wpilib_artifact_download::download_and_extract_zip(
            &format!("https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first/ni-libraries/visa/{NI_VERSION}/visa-{NI_VERSION}-linuxathena.zip"), 
            &object_dir
        )?;
        println!("cargo:rustc-link-lib=visa");
        wpilib_artifact_download::download_and_extract_zip(
            &format!("https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first/ni-libraries/chipobject/{NI_VERSION}/chipobject-{NI_VERSION}-linuxathena.zip"), 
            &object_dir
        )?;
        println!("cargo:rustc-link-lib=RoboRIO_FRC_ChipObject");
        wpilib_artifact_download::download_and_extract_zip(
            &format!("https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first/ni-libraries/netcomm/{NI_VERSION}/netcomm-{NI_VERSION}-linuxathena.zip"), 
            &object_dir
        )?;
        println!("cargo:rustc-link-lib=FRC_NetworkCommunication");
        wpilib_artifact_download::download_and_extract_zip(
            &format!("https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first/ni-libraries/runtime/{NI_VERSION}/runtime-{NI_VERSION}-linuxathena.zip"), 
            &object_dir
        )?;
        println!("cargo:rustc-link-lib=fpgalvshim");
        println!("cargo:rustc-link-lib=embcanshim");
    }
    let link_dir = object_dir
        .join(artifact_info.object_path_in_zip)
        .join("shared");
    println!("cargo:rustc-link-search={}", link_dir.display());

    Ok(())
}
