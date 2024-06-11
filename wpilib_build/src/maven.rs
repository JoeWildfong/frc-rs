use std::{io::Cursor, path::{Path, PathBuf}};

use crate::Linkage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MavenTarget {
    LinuxArm32,
    LinuxArm64,
    LinuxAthena,
    LinuxX86_64,
    OsXUniversal,
    WindowsArm64,
    WindowsX86_64,
}

impl MavenTarget {
    pub fn from_rustc_target(target: &str) -> Option<Self> {
        // TODO: test the rest of these
        // also, maybe match something to LinuxArm32?
        // not done currently because non-rio LinuxArm32 is ambiguous and also rare
        match target {
            // untested
            "aarch64-unknown-linux-gnu" => Some(Self::LinuxArm64),
            // roborio
            "armv7-unknown-linux-gnueabi" => Some(Self::LinuxAthena),
            "x86_64-unknown-linux-gnu" => Some(Self::LinuxX86_64),
            // untested
            "x86_64-apple-darwin" | "aarch64-apple-darwin" => Some(Self::OsXUniversal),
            // untested
            "aarch64-pc-windows-msvc" => Some(Self::WindowsArm64),
            // untested
            "x64_64-pc-windows-msvc" => Some(Self::WindowsX86_64),
            _ => None
        }
    }

    fn as_maven_fragment(&self) -> &'static str {
        match self {
            Self::LinuxArm32 => "linuxarm32",
            Self::LinuxArm64 => "linuxarm64",
            Self::LinuxAthena => "linuxathena",
            Self::LinuxX86_64 => "linuxx86-64",
            Self::OsXUniversal => "osxuniversal",
            Self::WindowsArm64 => "windowsarm64",
            Self::WindowsX86_64 => "windowsx86-64",
        }
    }

    fn as_path_fragment(&self) -> &'static str {
        match self {
            Self::LinuxArm32 => "linux/arm32",
            Self::LinuxArm64 => "linux/arm64",
            Self::LinuxAthena => "linux/athena",
            Self::LinuxX86_64 => "linux/x86-64",
            Self::OsXUniversal => "osx/universal",
            Self::WindowsArm64 => "windows/arm64",
            Self::WindowsX86_64 => "windows/x86-64",
        }
    }

    fn stdcpp_link(&self) -> Option<&'static str> {
        match self {
            Self::WindowsArm64 | Self::WindowsX86_64 => None,
            _ => Some("stdc++"),
        }
    }
}

pub fn download_and_extract_zip(
    url: &str,
    output_dir: impl AsRef<Path>,
) {
    let resp = reqwest::blocking::get(url).unwrap().bytes().unwrap();
    println!("maven downloader: got {} bytes from {url}", resp.len());
    zip::ZipArchive::new(Cursor::new(resp)).unwrap().extract(output_dir).unwrap();
}

pub fn build(b: &super::Build, linkage: Linkage, target: MavenTarget) {
    let name = b.maven_url_name;
    let version = b.version;
    let t = target.as_maven_fragment();
    let l = match linkage {
        Linkage::Shared => "",
        Linkage::Static => "static",
    };
    let debug = std::env::var("DEBUG").unwrap().as_str() == "none";
    let d = if debug { "debug" } else { "" };
    println!("cargo::rerun-if-env-changed=DEBUG");
    let url = format!(
        "https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first/{name}/{name}-cpp/{version}/{name}-cpp-{version}-{t}{l}{d}.zip"
    );
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    download_and_extract_zip(&url, &out_dir);
    let linkage_path_fragment = match linkage {
        Linkage::Shared => "shared",
        Linkage::Static => "static",
    };
    let extracted_dir = out_dir.join(target.as_path_fragment()).join(linkage_path_fragment);
    println!("cargo::rustc-link-search={}", extracted_dir.display());
    let debug_suffix = if debug { "d" } else { "" };
    println!("cargo::rustc-link-lib={}{debug_suffix}", b.maven_link_name);
    if let Some(cpp) = target.stdcpp_link() {
        println!("cargo::rustc-link-lib={cpp}");
    }
}
