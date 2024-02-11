use std::error::Error;

use super::libraries;

pub fn generate_bindings() -> Result<(), Box<dyn Error>> {
    let headers_folder = libraries::get_wpiutil()?;

    let bindings = bindgen::Builder::default()
        .clang_args(super::clang_args_for_toolchain(
            &super::find_wpilib_toolchain_root(),
        ))
        .clang_arg(format!(
            "-isystem{}",
            headers_folder
        ))
        .clang_args(["-std=c++20"])
        .header_contents(
            "wpiutil.hpp",
            r#"
#include "wpi/Synchronization.h"
#include "wpi/DataLog.h"
            "#
        )
        .allowlist_function("WPI_.*")
        .allowlist_type("WPI_.*")
        .allowlist_var("WPI_.*")
        .generate()
        .expect("failed to generate bindings");
    bindings
        .write_to_file(crate::project_root().join("wpiutil_sys/src/bindings.rs"))
        .expect("failed to write to file");
    Ok(())
}
