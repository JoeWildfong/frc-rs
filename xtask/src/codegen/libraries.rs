use std::{path::PathBuf, sync::Once};

static GOT_WPILIB_HAL: Once = Once::new();
static GOT_NI_CHIPOBJECT: Once = Once::new();
static GOT_WPILIB_WPIUTIL: Once = Once::new();

pub fn get_wpilib_hal() -> PathBuf {
    use super::WPILIB_VERSION;
    let base_url = super::frc_maven_url();
    let hal_header_url =
        format!("{base_url}/hal/hal-cpp/{WPILIB_VERSION}/hal-cpp-{WPILIB_VERSION}-headers.zip");
    let hal_header_folder = super::header_folder().join("hal");
    GOT_WPILIB_HAL.call_once(|| {
        super::download_and_extract_zip(&hal_header_url, &hal_header_folder).unwrap();
    });
    hal_header_folder
}

pub fn get_ni_chipobject() -> PathBuf {
    use super::NI_VERSION;
    let base_url = super::frc_maven_url();
    let chipobject_header_url = format!(
        "{base_url}/ni-libraries/chipobject/{NI_VERSION}/chipobject-{NI_VERSION}-headers.zip"
    );
    let chipobject_header_folder = super::header_folder().join("chipobject");
    GOT_NI_CHIPOBJECT.call_once(|| {
        super::download_and_extract_zip(&chipobject_header_url, &chipobject_header_folder).unwrap();
    });
    chipobject_header_folder
}

pub fn get_wpilib_wpiutil() -> PathBuf {
    use super::WPILIB_VERSION;
    let base_url = super::frc_maven_url();
    let wpiutil_header_url = format!(
        "{base_url}/wpiutil/wpiutil-cpp/{WPILIB_VERSION}/wpiutil-cpp-{WPILIB_VERSION}-headers.zip"
    );
    let wpiutil_header_folder = super::header_folder().join("wpiutil");
    GOT_WPILIB_WPIUTIL.call_once(|| {
        super::download_and_extract_zip(&wpiutil_header_url, &wpiutil_header_folder).unwrap();
    });
    wpiutil_header_folder
}
