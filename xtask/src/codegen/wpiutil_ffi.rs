use std::error::Error;

use super::libraries;

pub fn generate_bindings() -> Result<(), Box<dyn Error>> {
    let headers_folder = libraries::get_wpiutil()?;

    let bindings = bindgen::Builder::default()
        .clang_args(super::clang_args_for_toolchain(
            &super::find_wpilib_toolchain_root(),
        ))
        .clang_args([format!(
            "-isystem{}",
            headers_folder.join("wpiutil/headers").display()
        )])
        .header(
            headers_folder
                .join("wpi/Synchronization.h")
                .to_string_lossy(),
        )
        .allowlist_function("WPI_.*")
        .allowlist_type("WPI_.*")
        .allowlist_var("WPI_.*")
        .generate()
        .expect("failed to generate bindings");
    bindings
        .write_to_file(crate::project_root().join("wpiutil_ffi/src/bindings.rs"))
        .expect("failed to write to file");
    Ok(())
}
