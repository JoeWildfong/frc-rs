use std::error::Error;

use super::libraries;

pub fn generate_bindings() -> Result<(), Box<dyn Error>> {
    let headers_folder = libraries::get_ntcore()?;

    let bindings = bindgen::Builder::default()
        .clang_args(super::clang_args_for_toolchain(
            &super::find_wpilib_toolchain_root(),
        ))
        .clang_arg(format!(
            "-isystem{}",
            headers_folder
        ))
        .header(headers_folder.join("ntcore_c.h"))
        .allowlist_function("NT_.*")
        .allowlist_type("NT_.*")
        .allowlist_var("NT_.*")
        .raw_line("use wpiutil_sys::WPI_DataLog;")
        .blocklist_type("WPI_DataLog")
        .generate()
        .expect("failed to generate bindings");
    bindings
        .write_to_file(crate::project_root().join("ntcore_sys/src/bindings.rs"))
        .expect("failed to write to file");
    Ok(())
}
