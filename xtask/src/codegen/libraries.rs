use std::sync::Once;

use crate::camino::Utf8PathBuf;

static GOT_WPIHAL: Once = Once::new();
static GOT_WPIUTIL: Once = Once::new();

pub fn get_wpihal() -> Result<Utf8PathBuf, &'static str> {
    let wpihal_folder = crate::project_root().join("wpihal_sys");
    let sources_folder = wpihal_folder.join("wpihal/sources");
    let headers_folder = wpihal_folder.join("wpihal/headers");

    GOT_WPIHAL.call_once(|| {
        use super::{FRC_MAVEN_URL, WPILIB_VERSION};
        std::fs::create_dir_all(&sources_folder).unwrap();
        std::fs::create_dir_all(&headers_folder).unwrap();
        super::download_and_extract_zip(
            &format!(
                "{FRC_MAVEN_URL}/hal/hal-cpp/{WPILIB_VERSION}/hal-cpp-{WPILIB_VERSION}-headers.zip"
            ),
            &headers_folder,
        )
        .unwrap();
        super::download_and_extract_zip(
            &format!(
                "{FRC_MAVEN_URL}/hal/hal-cpp/{WPILIB_VERSION}/hal-cpp-{WPILIB_VERSION}-sources.zip"
            ),
            &sources_folder,
        )
        .unwrap();
        std::fs::remove_dir_all(sources_folder.join("jni")).ok();
    });

    if GOT_WPIHAL.is_completed() {
        Ok(headers_folder)
    } else {
        Err("failed to get wpihal")
    }
}

pub fn get_wpiutil() -> Result<Utf8PathBuf, &'static str> {
    let wpiutil_folder = crate::project_root().join("wpiutil_sys");
    let sources_folder = wpiutil_folder.join("wpiutil/sources");
    let headers_folder = wpiutil_folder.join("wpiutil/headers");

    GOT_WPIUTIL.call_once(|| {
        use super::{FRC_MAVEN_URL, WPILIB_VERSION};
        std::fs::create_dir_all(&sources_folder).unwrap();
        std::fs::create_dir_all(&headers_folder).unwrap();
        super::download_and_extract_zip(&format!("{FRC_MAVEN_URL}/wpiutil/wpiutil-cpp/{WPILIB_VERSION}/wpiutil-cpp-{WPILIB_VERSION}-headers.zip"), &headers_folder).unwrap();
        super::download_and_extract_zip(&format!("{FRC_MAVEN_URL}/wpiutil/wpiutil-cpp/{WPILIB_VERSION}/wpiutil-cpp-{WPILIB_VERSION}-sources.zip"), &sources_folder).unwrap();
        std::fs::remove_dir_all(sources_folder.join("jni")).ok();
    });

    if GOT_WPIUTIL.is_completed() {
        Ok(headers_folder)
    } else {
        Err("failed to get wpiutil")
    }
}
