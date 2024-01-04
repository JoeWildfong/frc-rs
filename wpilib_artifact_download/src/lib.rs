use std::{
    io::{Cursor, Read},
    path::Path,
};
use walkdir::WalkDir;

pub struct WpilibArtifactInfo {
    pub platform_name: &'static str,
    pub object_path_in_zip: &'static Path,
}

impl WpilibArtifactInfo {
    pub fn from_target(target: &str) -> Self {
        match target {
            "aarch64-unknown-linux-gnu" => Self {
                platform_name: "linuxarm64",
                object_path_in_zip: &Path::new("linux/arm64"),
            },
            #[cfg(sim)]
            "arm-unknown-linux-gnueabi" => Self {
                platform_name: "linuxarm32",
                object_path_in_zip: &Path::new("linux/arm32"),
            },
            #[cfg(not(sim))]
            "arm-unknown-linux-gnueabi" => Self {
                platform_name: "linuxathena",
                object_path_in_zip: &Path::new("linux/athena"),
            },
            "x86_64-unknown-linux-gnu" => Self {
                platform_name: "linuxx86-64",
                object_path_in_zip: &Path::new("linux/x86-64"),
            },
            "aarch64-apple-darwin" | "x86_64-apple-darwin" => Self {
                platform_name: "osxuniversal",
                object_path_in_zip: &Path::new("osx/universal"),
            },
            "x86_64-pc-windows-msvc" => Self {
                platform_name: "windowsx86-64",
                object_path_in_zip: &Path::new("windows/x86-64"),
            },
            _ => panic!("unsupported OS/architecture"),
        }
    }
}

pub const WPILIB_VERSION: &str = "2023.4.3";
pub const NI_VERSION: &str = "2023.3.0";

fn clean_objects_in_dir(dir: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let file_path = entry.path();
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
    Ok(())
}

pub fn download_and_extract_zip(
    url: &str,
    output_dir: impl AsRef<Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let resp = ureq::get(url).call()?;
    let length = resp
        .header("Content-Length")
        .ok_or("No Content-Length header")?
        .parse()?;
    let mut buf = Vec::with_capacity(length);
    resp.into_reader().take(100_000_000).read_to_end(&mut buf)?;
    zip::ZipArchive::new(Cursor::new(buf))?.extract(output_dir.as_ref())?;
    clean_objects_in_dir(output_dir)?;
    Ok(())
}
