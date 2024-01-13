use std::{error::Error, process::Command};

pub fn generate_bindings() -> Result<(), Box<dyn Error>> {
    let ni_frc_libs_path = crate::project_root().join("ni_frc_libs");
    let include_folder = ni_frc_libs_path.join("ni-libraries/src/include");

    let binding_gen = bindgen::Builder::default()
    .clang_args(super::clang_args_for_toolchain(
        &super::find_wpilib_toolchain_root(),
    ));

    let chipobject_header = include_folder.join("FRC_FPGA_ChipObject/fpgainterfacecapi/NiFpga.h");
    let chipobject_bindings = binding_gen.clone()
        .header(
                chipobject_header.to_string_lossy(),
        )
        .allowlist_file(chipobject_header.to_string_lossy())
        .generate()
        .expect("failed to generate bindings");
    chipobject_bindings
        .write_to_file(ni_frc_libs_path.join("src/chipobject_bindings.rs"))
        .expect("failed to write to file");

    let netcomm_bindings = binding_gen.clone()
        .clang_args(["-x", "c++"])
        .clang_arg(format!("-I{}", include_folder.join("FRC_NetworkCommunication").to_string_lossy()))
        .header_contents("allnetcomm.h", r#"
#include "AICalibration.h"
#include "CANInterfacePlugin.h"
#include "CANSessionMux.h"
#include "FRCComm.h"
#include "LoadOut.h"
#include "NetCommRPCProxy_Occur.h"
#include "UsageReporting.h"
        "#)
        .allowlist_file(format!("{}.*", include_folder.join("FRC_NetworkCommunication").to_string_lossy()))
        .size_t_is_usize(false)
        .generate()
        .expect("failed to generate bindings");
    netcomm_bindings
        .write_to_file(ni_frc_libs_path.join("src/netcomm_bindings.rs"))
        .expect("failed to write to file");

    let visa_bindings = binding_gen.clone()
        .clang_arg(format!("-I{}", include_folder.join("visa").to_string_lossy()))
        .header_contents("allvisa.h", r#"
#include "visa.h"
#include "visatype.h"
        "#)
        .allowlist_file(format!("{}.*", include_folder.join("visa").to_string_lossy()))
        .size_t_is_usize(false)
        .generate()
        .expect("failed to generate bindings");
    visa_bindings
        .write_to_file(ni_frc_libs_path.join("src/visa_bindings.rs"))
        .expect("failed to write to file");

    // compile shims into built-shims folder
    let compiler_path = super::find_wpilib_gcc();
    let shims_folder = ni_frc_libs_path.join("ni-libraries/src/shims");
    let shims_output = ni_frc_libs_path.join("built-shims");
    std::fs::remove_dir_all(&shims_output).ok();
    std::fs::create_dir_all(&shims_output)?;
    Command::new(&compiler_path)
        .args([
            "-shared",
            &format!("{}/embcan/main.c", shims_folder.display()),
            "-o",
            &format!("{}/libnirio_emb_can.so.23", shims_output.display()),
        ])
        .spawn()?
        .wait()?;
    Command::new(&compiler_path)
        .args([
            "-shared",
            &format!("{}/fpgalv/main.c", shims_folder.display()),
            "-o",
            &format!("{}/libNiFpgaLv.so.13", shims_output.display()),
        ])
        .spawn()?
        .wait()?;

    Ok(())
}
