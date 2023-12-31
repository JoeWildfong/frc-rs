use std::{error::Error, path::{Path, PathBuf}, io::{Cursor, Read}};

mod wpilib_hal_ffi;

pub fn generate_bindings(crate_name: Option<String>) -> Result<(), Box<dyn Error>> {
    std::fs::remove_dir_all(header_folder()).unwrap_or_else(|err| match err.kind() {
        std::io::ErrorKind::NotFound => {},
        _ => panic!("failed to remove headers folder"),
    });
    std::fs::create_dir(header_folder()).expect("failed to create headers folder");
    match crate_name {
        Some(t) => {
            match t.as_str() {
                "wpilib_hal_ffi" => wpilib_hal_ffi::generate_bindings()?,
                invalid => return Err(format!("Invalid crate name: {invalid}").into()),
            }
        },
        None => {
            wpilib_hal_ffi::generate_bindings()?;
        },
    }
    Ok(())
}

pub fn frc_maven_url() -> String {
    "https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first".to_owned()
}

pub fn header_folder() -> PathBuf {
    super::project_root().join("xtask/target/headers")
}

pub fn download_and_extract_zip(url: &str, output_dir: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    let resp = ureq::get(url).call()?;
    let length = resp.header("Content-Length").ok_or("No Content-Length header")?.parse()?;
    let mut buf = Vec::with_capacity(length);
    resp.into_reader().take(100_000_000).read_to_end(&mut buf)?;
    zip::ZipArchive::new(Cursor::new(buf))?.extract(output_dir)?;
    Ok(())
}

pub fn find_wpilib_toolchain() -> PathBuf {
    if let Some(path) = std::env::var_os("WPILIB_TOOLCHAIN").map(PathBuf::from) {
        assert!(path.exists(), "WPILIB_TOOLCHAIN environment variable set to {}, but the path doesn't exist", path.display());
        return path;
    };
    let default_location = if cfg!(windows) {
        PathBuf::from("C:/Users/Public/wpilib/2023")
    } else if cfg!(unix) {
        let Some(home) = dirs::home_dir() else {
            panic!("Could not get default toolchain location because home directory could not be determined. Try setting the WPILIB_TOOLCHAIN environment variable.");
        };
        home.join("wpilib").join("2023")
    } else {
        panic!("Your OS does not have a default WPILib toolchain location. Try setting the WPILIB_TOOLCHAIN environment variable.")
    };
    assert!(
        default_location.exists(),
        "Could not find WPILib toolchain at default location {}. Make sure the toolchain is installed, or try setting the WPILIB_TOOLCHAIN environment variable.",
        default_location.display()
    );
    default_location
}

pub fn clang_args_for_toolchain(toolchain_path: &Path) -> impl Iterator<Item=String> {
    let sysroot_path = toolchain_path.join(PathBuf::from(
        "roborio-academic/arm-nilrt-linux-gnueabi/sysroot",
    ));
    [
        format!("-isysroot{}", sysroot_path.display()),
        "-iwithsysroot/usr/lib/gcc/arm-nilrt-linux-gnueabi/12/include".to_owned(),
        "-iwithsysroot/usr/lib/gcc/arm-nilrt-linux-gnueabi/12/include-fixed".to_owned(),
        "-iwithsysroot/usr/include/c++/12".to_owned(),
        "-iwithsysroot/usr/include/c++/12/arm-nilrt-linux-gnueabi".to_owned(),
        "-iwithsysroot/usr/include/c++/12/backward".to_owned(),
        "-iwithsysroot/usr/include".to_owned(),
    ].into_iter()
}

#[allow(dead_code)]
pub fn wrap_all_headers(dir: impl AsRef<Path>) -> String {
    walkdir::WalkDir::new(dir).into_iter()
        .filter_entry(|e| e.file_name().to_str() != Some("cpp"))
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file() && entry.file_name().to_string_lossy().ends_with(".h"))
        .map(|file| format!("#include \"{}\"", file.into_path().to_string_lossy()))
        .collect::<Vec<_>>()
        .join("\n")
}
