use std::sync::Once;

use cargo_metadata::camino::Utf8PathBuf;

static GOT_WPIHAL: Once = Once::new();
static GOT_WPIUTIL: Once = Once::new();

pub fn get_wpihal() -> Result<Utf8PathBuf, &'static str> {
    let wpihal_folder = crate::project_root().join("wpihal_ffi");
    let sources_folder = wpihal_folder.join("wpihal/sources");
    let headers_folder = wpihal_folder.join("wpihal/headers");

    GOT_WPIHAL.call_once(|| {
        std::fs::create_dir_all(&sources_folder).unwrap();
        std::fs::create_dir_all(&headers_folder).unwrap();
        let base_url = super::frc_maven_url();
        use super::WPILIB_VERSION;
        super::download_and_extract_zip(
            &format!(
                "{base_url}/hal/hal-cpp/{WPILIB_VERSION}/hal-cpp-{WPILIB_VERSION}-headers.zip"
            ),
            &headers_folder,
        )
        .unwrap();
        super::download_and_extract_zip(
            &format!(
                "{base_url}/hal/hal-cpp/{WPILIB_VERSION}/hal-cpp-{WPILIB_VERSION}-sources.zip"
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
    let wpiutil_folder = crate::project_root().join("wpiutil_ffi");
    let sources_folder = wpiutil_folder.join("wpiutil/sources");
    let headers_folder = wpiutil_folder.join("wpiutil/headers");

    GOT_WPIUTIL.call_once(|| {
        std::fs::create_dir_all(&sources_folder).unwrap();
        std::fs::create_dir_all(&headers_folder).unwrap();
        let base_url = super::frc_maven_url();
        use super::WPILIB_VERSION;
        super::download_and_extract_zip(&format!("{base_url}/wpiutil/wpiutil-cpp/{WPILIB_VERSION}/wpiutil-cpp-{WPILIB_VERSION}-headers.zip"), &headers_folder).unwrap();
        super::download_and_extract_zip(&format!("{base_url}/wpiutil/wpiutil-cpp/{WPILIB_VERSION}/wpiutil-cpp-{WPILIB_VERSION}-sources.zip"), &sources_folder).unwrap();
        std::fs::remove_dir_all(sources_folder.join("jni")).ok();
    });

    if GOT_WPIUTIL.is_completed() {
        Ok(headers_folder)
    } else {
        Err("failed to get wpiutil")
    }
}
