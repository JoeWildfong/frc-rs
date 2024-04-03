use std::error::Error;

use super::libraries;

pub fn generate_bindings() -> Result<(), Box<dyn Error>> {
    let headers_folder = libraries::get_cscore()?;
    let wpiutil_headers = libraries::get_wpiutil()?;

    let bindings = bindgen::Builder::default()
        .clang_args(super::clang_args_for_toolchain(
            &super::find_wpilib_toolchain_root(),
        ))
        .clang_args([
            format!("-isystem{}", headers_folder),
            format!("-isystem{}", wpiutil_headers),
        ])
        .header(headers_folder.join("cscore_c.h"))
        .allowlist_function("CS_.*")
        .allowlist_type("CS_.*")
        .allowlist_var("CS_.*")
        .generate()
        .expect("failed to generate bindings");
    bindings
        .write_to_file(crate::project_root().join("cscore_sys/src/bindings.rs"))
        .expect("failed to write to file");
    Ok(())
}
