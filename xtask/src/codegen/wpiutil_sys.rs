use std::{error::Error, fs::File, io::Write};

use super::libraries;

pub fn generate_bindings() -> Result<(), Box<dyn Error>> {
    let headers_folder = libraries::get_wpiutil()?;

    let bindings = bindgen::Builder::default()
        .clang_args(super::clang_args_for_toolchain(
            &super::find_wpilib_toolchain_root(),
        ))
        .clang_arg(format!(
            "-isystem{}",
            headers_folder.join("wpiutil/headers")
        ))
        .header(headers_folder.join("wpi/Synchronization.h"))
        .allowlist_function("WPI_.*")
        .allowlist_type("WPI_.*")
        .allowlist_var("WPI_.*")
        .generate()
        .expect("failed to generate bindings");
    bindings
        .write_to_file(crate::project_root().join("wpiutil_sys/src/bindings.rs"))
        .expect("failed to write to file");

    let mut version_file = File::create(crate::project_root().join("wpiutil_sys/version.txt"))?;
    write!(version_file, "{}", super::WPILIB_VERSION)?;

    Ok(())
}
