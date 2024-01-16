use std::error::Error;

use super::libraries;

pub fn generate_bindings() -> Result<(), Box<dyn Error>> {
    let headers_folder = libraries::get_wpihal()?;
    let wpiutil_headers = libraries::get_wpiutil()?;
    let ni_frc_headers = crate::project_root().join("ni_frc_sys/ni-libraries/src/include");
    let wrappers_folder = crate::project_root().join("xtask/src/codegen/wrappers");

    let bindings = bindgen::Builder::default()
        .clang_args(super::clang_args_for_toolchain(
            &super::find_wpilib_toolchain_root(),
        ))
        .clang_args([
            format!("-isystem{}", headers_folder),
            format!("-isystem{}", ni_frc_headers),
            format!("-isystem{}", wpiutil_headers),
        ])
        .header(headers_folder.join("hal/HAL.h"))
        .header(wrappers_folder.join("REVPH.h"))
        .allowlist_function("HAL_.*")
        .allowlist_type("HAL_.*")
        .allowlist_var("HAL_.*")
        .raw_line("use wpiutil_sys::WPI_EventHandle;")
        .blocklist_type("WPI_EventHandle")
        .blocklist_type("WPI_Handle") // wpihal never uses this in the public API
        .generate()
        .expect("failed to generate bindings");

    bindings
        .write_to_file(crate::project_root().join("wpihal_sys/src/bindings.rs"))
        .expect("failed to write to file");
    Ok(())
}
