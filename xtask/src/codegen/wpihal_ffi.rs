use std::{error::Error, path::Path};

use super::libraries;

pub fn generate_bindings() -> Result<(), Box<dyn Error>> {
    let hal_header_folder = libraries::get_wpilib_hal();
    let chipobject_header_folder = libraries::get_ni_chipobject();
    let wpiutil_header_folder = libraries::get_wpilib_wpiutil();

    let wrappers_folder = Path::new(file!()).parent().unwrap().join("wrappers");
    let bindings = bindgen::Builder::default()
        .clang_args(super::clang_args_for_toolchain(
            &super::find_wpilib_toolchain(),
        ))
        .clang_args([
            format!("-isystem{}", hal_header_folder.display()),
            format!("-isystem{}", chipobject_header_folder.display()),
            format!("-isystem{}", wpiutil_header_folder.display()),
        ])
        .header(hal_header_folder.join("hal/HAL.h").to_string_lossy())
        .header(wrappers_folder.join("REVPH.h").to_string_lossy())
        .allowlist_function("HAL_.*")
        .allowlist_type("HAL_.*")
        .allowlist_var("HAL_.*")
        .generate()
        .expect("failed to generate bindings");
    bindings
        .write_to_file(crate::project_root().join("wpihal_ffi/src/bindings.rs"))
        .expect("failed to write to file");
    Ok(())
}
