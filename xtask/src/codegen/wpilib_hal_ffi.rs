use std::{error::Error, path::Path};

pub fn generate_bindings() -> Result<(), Box<dyn Error>> {
    let base_url = super::frc_maven_url();
    let wpilib_version = "2023.4.3";
    let hal_header_url = format!("{base_url}/hal/hal-cpp/{wpilib_version}/hal-cpp-{wpilib_version}-headers.zip");
    let hal_header_folder = super::header_folder().join("hal");
    super::download_and_extract_zip(&hal_header_url, &hal_header_folder)?;
    let ni_version = "2023.3.0";
    let chipobject_header_url = format!("{base_url}/ni-libraries/chipobject/{ni_version}/chipobject-{ni_version}-headers.zip");
    let chipobject_header_folder = super::header_folder().join("chipobject");
    super::download_and_extract_zip(&chipobject_header_url, &chipobject_header_folder)?;
    let wpiutil_header_url = format!("{base_url}/wpiutil/wpiutil-cpp/{wpilib_version}/wpiutil-cpp-{wpilib_version}-headers.zip");
    let wpiutil_header_folder = super::header_folder().join("wpiutil");
    super::download_and_extract_zip(&wpiutil_header_url, &wpiutil_header_folder)?;

    let wrappers_folder = Path::new(file!()).parent().unwrap().join("wrappers");
    let bindings = bindgen::Builder::default()
        .clang_args(super::clang_args_for_toolchain(&super::find_wpilib_toolchain()))
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
    bindings.write_to_file(crate::project_root().join("wpilib_hal_ffi/src/bindings.rs")).expect("failed to write to file");
    Ok(())
}
