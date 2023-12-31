use std::{path::{Path, PathBuf}, io::{Cursor, Read}};

struct HalArtifactInfo {
    platform_name: &'static str,
    object_path_in_zip: &'static Path
}

impl HalArtifactInfo {
    fn from_target(target: &str) -> Self {
        match target {
            "aarch64-unknown-linux-gnu" => Self { platform_name: "linuxarm64", object_path_in_zip: &Path::new("linux/arm64") },
            #[cfg(sim)]
            "arm-unknown-linux-gnueabi" => Self { platform_name: "linuxarm32", object_path_in_zip: &Path::new("linux/arm32") },
            #[cfg(not(sim))]
            "arm-unknown-linux-gnueabi" => Self { platform_name: "linuxathena", object_path_in_zip: &Path::new("linux/athena") },
            "x86_64-unknown-linux-gnu" => Self { platform_name: "linuxx86-64", object_path_in_zip: &Path::new("linux/x86-64") },
            "aarch64-apple-darwin" | "x86_64-apple-darwin" => Self { platform_name: "osxuniversal", object_path_in_zip: &Path::new("osx/universal") },
            "x86_64-pc-windows-msvc" => Self { platform_name: "windowsx86-64", object_path_in_zip: &Path::new("windows/x86-64") },
            _ => panic!("unsupported OS/architecture"),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let object_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set")).join("objects");
    let artifact_info = HalArtifactInfo::from_target(std::env::var("TARGET")?.as_str());
    let wpilib_version = "2023.4.3";
    let ni_version = "2023.3.0";

    download_and_extract_zip(
        &format!("https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first/hal/hal-cpp/{wpilib_version}/hal-cpp-{wpilib_version}-{}.zip", artifact_info.platform_name), 
        &object_dir
    )?;
    println!("cargo:rustc-link-lib=wpiHal");
    download_and_extract_zip(
        &format!("https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first/wpiutil/wpiutil-cpp/{wpilib_version}/wpiutil-cpp-{wpilib_version}-{}.zip", artifact_info.platform_name), 
        &object_dir
    )?;
    println!("cargo:rustc-link-lib=wpiUtil");

    if artifact_info.platform_name == "linuxathena" {
        download_and_extract_zip(
            &format!("https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first/ni-libraries/visa/{ni_version}/visa-{ni_version}-linuxathena.zip"), 
            &object_dir
        )?;
        println!("cargo:rustc-link-lib=visa");
        download_and_extract_zip(
            &format!("https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first/ni-libraries/chipobject/{ni_version}/chipobject-{ni_version}-linuxathena.zip"), 
            &object_dir
        )?;
        println!("cargo:rustc-link-lib=RoboRIO_FRC_ChipObject");
        download_and_extract_zip(
            &format!("https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first/ni-libraries/netcomm/{ni_version}/netcomm-{ni_version}-linuxathena.zip"), 
            &object_dir
        )?;
        println!("cargo:rustc-link-lib=FRC_NetworkCommunication");
        download_and_extract_zip(
            &format!("https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first/ni-libraries/runtime/{ni_version}/runtime-{ni_version}-linuxathena.zip"), 
            &object_dir
        )?;
        println!("cargo:rustc-link-lib=fpgalvshim");
        println!("cargo:rustc-link-lib=embcanshim");
    }
    let link_dir = object_dir.join(artifact_info.object_path_in_zip).join("shared");
    for file in std::fs::read_dir(&link_dir)? {
        let file_path = file?.path();
        if file_path.ends_with(".debug") {
            std::fs::remove_file(file_path)?;
            continue;
        }
        let Some(Some(file_name)) = file_path.file_name().map(|name| name.to_str()) else {
            continue
        };
        if let Some(index) = file_name.rfind(".so") {
            let end_index = index + ".so".len();
            if end_index == file_name.len() {
                continue;
            }
            let new_name = &file_name[..index + ".so".len()];
            std::fs::rename(&file_path, file_path.with_file_name(&new_name))?;
        }
    }
    println!("cargo:rustc-link-search={}", link_dir.display());

    Ok(())
}

fn download_and_extract_zip(url: &str, output_dir: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let resp = ureq::get(url).call()?;
    let length = resp.header("Content-Length").ok_or("No Content-Length header")?.parse()?;
    let mut buf = Vec::with_capacity(length);
    resp.into_reader().take(100_000_000).read_to_end(&mut buf)?;
    zip::ZipArchive::new(Cursor::new(buf))?.extract(output_dir)?;
    Ok(())
}
