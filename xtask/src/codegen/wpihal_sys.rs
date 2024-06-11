use std::{error::Error, fs::File, io::Write};

use bindgen::callbacks::{IntKind, ParseCallbacks};

use super::libraries;

pub fn generate_bindings() -> Result<(), Box<dyn Error>> {
    let headers_folder = libraries::get_wpihal()?;
    let wpiutil_headers = libraries::get_wpiutil()?;
    let ni_frc_headers = crate::project_root().join("ni_frc_sys/ni-libraries/src/include");
    let wrappers_folder = crate::project_root().join("xtask/src/codegen/wrappers");

    let mut bindings = bindgen::Builder::default()
        .clang_args(super::clang_args_for_toolchain(
            &super::find_wpilib_toolchain_root(),
        ))
        .clang_args([
            format!("-isystem{headers_folder}"),
            format!("-isystem{ni_frc_headers}"),
            format!("-isystem{wpiutil_headers}"),
        ])
        .header(headers_folder.join("hal/HAL.h"))
        .header(wrappers_folder.join("REVPH.h"))
        .allowlist_function("HAL_.*")
        .allowlist_type("HAL_.*")
        .allowlist_var("HAL_.*")
        .raw_line("use wpiutil_sys::WPI_EventHandle;")
        .blocklist_type("WPI_EventHandle")
        .blocklist_type("WPI_Handle") // wpihal never uses this in the public API
        .parse_callbacks(Box::new(Callbacks));
    for name in Callbacks::ERROR_MACRO_CONSTANTS {
        bindings = bindings.allowlist_var(name);
    }
    let bindings = bindings.generate().expect("failed to generate bindings");

    bindings
        .write_to_file(crate::project_root().join("wpihal_sys/src/bindings.rs"))
        .expect("failed to write to file");

    let mut version_file = File::create(crate::project_root().join("wpihal_sys/version.txt"))?;
    write!(version_file, "{}", super::WPILIB_VERSION)?;
    Ok(())
}

#[derive(Debug)]
struct Callbacks;

impl Callbacks {
    // hal/src/main/native/include/hal/Errors.h
    const ERROR_MACRO_CONSTANTS: &'static [&'static str] = &[
        "HAL_SUCCESS",
        "SAMPLE_RATE_TOO_HIGH",
        "VOLTAGE_OUT_OF_RANGE",
        "LOOP_TIMING_ERROR",
        "SPI_WRITE_NO_MOSI",
        "SPI_READ_NO_MISO",
        "SPI_READ_NO_DATA",
        "INCOMPATIBLE_STATE",
        "NO_AVAILABLE_RESOURCES",
        "NULL_PARAMETER",
        "ANALOG_TRIGGER_LIMIT_ORDER_ERROR",
        "ANALOG_TRIGGER_PULSE_OUTPUT_ERROR",
        "PARAMETER_OUT_OF_RANGE",
        "RESOURCE_IS_ALLOCATED",
        "RESOURCE_OUT_OF_RANGE",
        "HAL_INVALID_ACCUMULATOR_CHANNEL",
        "HAL_COUNTER_NOT_SUPPORTED",
        "HAL_PWM_SCALE_ERROR",
        "HAL_HANDLE_ERROR",
        "HAL_LED_CHANNEL_ERROR",
        "HAL_INVALID_DMA_ADDITION",
        "HAL_INVALID_DMA_STATE",
        "HAL_SERIAL_PORT_NOT_FOUND",
        "HAL_SERIAL_PORT_OPEN_ERROR",
        "HAL_SERIAL_PORT_ERROR",
        "HAL_THREAD_PRIORITY_ERROR",
        "HAL_THREAD_PRIORITY_RANGE_ERROR",
        "HAL_CAN_TIMEOUT",
        "HAL_SIM_NOT_SUPPORTED",
        "HAL_USE_LAST_ERROR",
        "HAL_CONSOLE_OUT_ENABLED_ERROR",
        "HAL_CAN_BUFFER_OVERRUN",
    ];
}

impl ParseCallbacks for Callbacks {
    fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
        if Self::ERROR_MACRO_CONSTANTS.contains(&name) {
            // apis treat errors as i32s
            Some(IntKind::I32)
        } else {
            None
        }
    }
}
