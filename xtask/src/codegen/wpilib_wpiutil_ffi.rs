use std::error::Error;

use super::libraries;

pub fn generate_bindings() -> Result<(), Box<dyn Error>> {
    let wpiutil_header_folder = libraries::get_wpilib_wpiutil();

    let bindings = bindgen::Builder::default()
        .clang_args(super::clang_args_for_toolchain(&super::find_wpilib_toolchain()))
        .clang_args([
            format!("-isystem{}", wpiutil_header_folder.display()),
        ])
        .header(wpiutil_header_folder.join("wpi/Synchronization.h").to_string_lossy())
        .allowlist_function("WPI_.*")
        .allowlist_type("WPI_.*")
        .allowlist_var("WPI_.*")
        .generate()
        .expect("failed to generate bindings");
    bindings.write_to_file(crate::project_root().join("wpilib_wpiutil_ffi/src/bindings.rs")).expect("failed to write to file");
    Ok(())
}
