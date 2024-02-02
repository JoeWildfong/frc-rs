use std::{
    error::Error,
    io::{Cursor, Read},
    path::Path,
};

use crate::camino::{Utf8Path, Utf8PathBuf};

mod libraries;
mod ni_frc_sys;
mod wpihal_sys;
mod wpiutil_sys;

const WPILIB_YEAR: &str = "2024";
const WPILIB_VERSION: &str = "2024.2.1";

const FRC_MAVEN_URL: &str = "https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first";

pub fn generate_bindings(crate_name: Option<String>) -> Result<(), Box<dyn Error>> {
    match crate_name {
        Some(t) => match t.as_str() {
            "wpihal_sys" => wpihal_sys::generate_bindings()?,
            "wpiutil_sys" => wpiutil_sys::generate_bindings()?,
            "ni_frc_sys" => ni_frc_sys::generate_bindings()?,
            invalid => return Err(format!("Invalid crate name: {invalid}").into()),
        },
        None => {
            wpihal_sys::generate_bindings()?;
            wpiutil_sys::generate_bindings()?;
            ni_frc_sys::generate_bindings()?;
        }
    }
    Ok(())
}

pub fn download_and_extract_zip(
    url: &str,
    output_dir: impl AsRef<Path>,
) -> Result<(), Box<dyn Error>> {
    let resp = ureq::get(url).call()?;
    let length = match resp.header("Content-Length") {
        Some(size) => size.parse::<usize>()?.max(100_000_000),
        None => 1_000_000,
    };
    let mut buf = Vec::with_capacity(length);
    resp.into_reader().take(100_000_000).read_to_end(&mut buf)?;
    zip::ZipArchive::new(Cursor::new(buf))?.extract(output_dir)?;
    Ok(())
}

pub fn find_wpilib_toolchain_root() -> Utf8PathBuf {
    if let Ok(path) = std::env::var("WPILIB_TOOLCHAIN").map(Utf8PathBuf::from) {
        assert!(
            path.exists(),
            "WPILIB_TOOLCHAIN environment variable set to {}, but the path doesn't exist",
            path
        );
        return path;
    };
    let default_location = if cfg!(windows) {
        Utf8PathBuf::from(format!("C:/Users/Public/wpilib/{WPILIB_YEAR}"))
    } else if cfg!(unix) {
        let Some(home) = dirs::home_dir() else {
            panic!("Could not get default toolchain location because home directory could not be determined. Try setting the WPILIB_TOOLCHAIN environment variable.");
        };
        Utf8PathBuf::try_from(home.join("wpilib").join(WPILIB_YEAR))
            .expect("home dir is valid UTF-8")
    } else {
        panic!("Your OS does not have a default WPILib toolchain location. Try setting the WPILIB_TOOLCHAIN environment variable.")
    };
    assert!(
        default_location.exists(),
        "Could not find WPILib toolchain at default location {}. Make sure the toolchain is installed, or try setting the WPILIB_TOOLCHAIN environment variable.",
        default_location
    );
    default_location
}

pub fn find_wpilib_gcc() -> Utf8PathBuf {
    find_wpilib_toolchain_root().join(format!(
        "roborio-academic/bin/arm-frc{}-linux-gnueabi-gcc",
        WPILIB_YEAR
    ))
}

pub fn clang_args_for_toolchain(toolchain_path: &Utf8Path) -> impl Iterator<Item = String> {
    let sysroot_path = toolchain_path.join(Utf8PathBuf::from(
        "roborio-academic/arm-nilrt-linux-gnueabi/sysroot",
    ));
    [
        "--target=armv7-unknown-linux-gnueabi".to_owned(),
        format!("-isysroot{}", sysroot_path),
        format!("--sysroot={}", sysroot_path),
        "-iwithsysroot/usr/lib/gcc/arm-nilrt-linux-gnueabi/12/include".to_owned(),
        "-iwithsysroot/usr/lib/gcc/arm-nilrt-linux-gnueabi/12/include-fixed".to_owned(),
        "-iwithsysroot/usr/include/c++/12".to_owned(),
        "-iwithsysroot/usr/include/c++/12/arm-nilrt-linux-gnueabi".to_owned(),
        "-iwithsysroot/usr/include/c++/12/backward".to_owned(),
        "-iwithsysroot/usr/include".to_owned(),
    ]
    .into_iter()
}
