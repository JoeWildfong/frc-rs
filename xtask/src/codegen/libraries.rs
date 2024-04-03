use std::sync::Once;

use crate::camino::Utf8PathBuf;

static GOT_NTCORE: Once = Once::new();
static GOT_CSCORE: Once = Once::new();
static GOT_WPIHAL: Once = Once::new();
static GOT_WPILIB: Once = Once::new();
static GOT_WPIMATH: Once = Once::new();
static GOT_WPINET: Once = Once::new();
static GOT_WPIUTIL: Once = Once::new();
static GOT_WPI_OPENCV: Once = Once::new();

pub fn get_ntcore() -> Result<Utf8PathBuf, &'static str> {
    let ntcore_folder = crate::project_root().join("ntcore_sys");
    let sources_folder = ntcore_folder.join("ntcore/sources");
    let headers_folder = ntcore_folder.join("ntcore/include");

    GOT_NTCORE.call_once(|| {
        std::fs::create_dir_all(&sources_folder).unwrap();
        std::fs::create_dir_all(&headers_folder).unwrap();
        use super::{FRC_MAVEN_URL, WPILIB_VERSION};
        super::download_and_extract_zip(&format!("{FRC_MAVEN_URL}/ntcore/ntcore-cpp/{WPILIB_VERSION}/ntcore-cpp-{WPILIB_VERSION}-headers.zip"), &headers_folder).unwrap();
        super::download_and_extract_zip(&format!("{FRC_MAVEN_URL}/ntcore/ntcore-cpp/{WPILIB_VERSION}/ntcore-cpp-{WPILIB_VERSION}-sources.zip"), &sources_folder).unwrap();
        std::fs::remove_dir_all(sources_folder.join("jni")).ok();
    });

    if GOT_NTCORE.is_completed() {
        Ok(headers_folder)
    } else {
        Err("failed to get ntcore")
    }
}

pub fn get_cscore() -> Result<Utf8PathBuf, &'static str> {
    let cscore_folder = crate::project_root().join("cscore_sys");
    let sources_folder = cscore_folder.join("cscore/sources");
    let headers_folder = cscore_folder.join("cscore/include");

    GOT_CSCORE.call_once(|| {
        std::fs::create_dir_all(&sources_folder).unwrap();
        std::fs::create_dir_all(&headers_folder).unwrap();
        use super::{FRC_MAVEN_URL, WPILIB_VERSION};
        super::download_and_extract_zip(&format!("{FRC_MAVEN_URL}/cscore/cscore-cpp/{WPILIB_VERSION}/cscore-cpp-{WPILIB_VERSION}-headers.zip"), &headers_folder).unwrap();
        super::download_and_extract_zip(&format!("{FRC_MAVEN_URL}/cscore/cscore-cpp/{WPILIB_VERSION}/cscore-cpp-{WPILIB_VERSION}-sources.zip"), &sources_folder).unwrap();
        std::fs::remove_dir_all(sources_folder.join("jni")).ok();
    });

    if GOT_CSCORE.is_completed() {
        Ok(headers_folder)
    } else {
        Err("failed to get cscore")
    }
}

pub fn get_wpihal() -> Result<Utf8PathBuf, &'static str> {
    let wpihal_folder = crate::project_root().join("wpihal_sys");
    let sources_folder = wpihal_folder.join("wpihal/sources");
    let headers_folder = wpihal_folder.join("wpihal/include");

    GOT_WPIHAL.call_once(|| {
        std::fs::create_dir_all(&sources_folder).unwrap();
        std::fs::create_dir_all(&headers_folder).unwrap();
        use super::{FRC_MAVEN_URL, WPILIB_VERSION};
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

pub fn get_wpilib() -> Result<Utf8PathBuf, &'static str> {
    let wpilib_folder = crate::project_root().join("wpilib_cxx");
    let sources_folder = wpilib_folder.join("wpilibc/sources");
    let headers_folder = wpilib_folder.join("wpilibc/include");

    GOT_WPILIB.call_once(|| {
        std::fs::create_dir_all(&sources_folder).unwrap();
        std::fs::create_dir_all(&headers_folder).unwrap();
        use super::{FRC_MAVEN_URL, WPILIB_VERSION};
        super::download_and_extract_zip(&format!("{FRC_MAVEN_URL}/wpilibc/wpilibc-cpp/{WPILIB_VERSION}/wpilibc-cpp-{WPILIB_VERSION}-headers.zip"), &headers_folder).unwrap();
        super::download_and_extract_zip(&format!("{FRC_MAVEN_URL}/wpilibc/wpilibc-cpp/{WPILIB_VERSION}/wpilibc-cpp-{WPILIB_VERSION}-sources.zip"), &sources_folder).unwrap();
    });

    if GOT_WPILIB.is_completed() {
        Ok(headers_folder)
    } else {
        Err("failed to get wpilib")
    }
}

pub fn get_wpimath() -> Result<Utf8PathBuf, &'static str> {
    let wpimath_folder = crate::project_root().join("wpimath_cxx");
    let sources_folder = wpimath_folder.join("wpimath/sources");
    let headers_folder = wpimath_folder.join("wpimath/include");

    GOT_WPIMATH.call_once(|| {
        std::fs::create_dir_all(&sources_folder).unwrap();
        std::fs::create_dir_all(&headers_folder).unwrap();
        use super::{FRC_MAVEN_URL, WPILIB_VERSION};
        super::download_and_extract_zip(&format!("{FRC_MAVEN_URL}/wpimath/wpimath-cpp/{WPILIB_VERSION}/wpimath-cpp-{WPILIB_VERSION}-headers.zip"), &headers_folder).unwrap();
        super::download_and_extract_zip(&format!("{FRC_MAVEN_URL}/wpimath/wpimath-cpp/{WPILIB_VERSION}/wpimath-cpp-{WPILIB_VERSION}-sources.zip"), &sources_folder).unwrap();
        std::fs::remove_dir_all(sources_folder.join("jni")).ok();
    });

    if GOT_WPIMATH.is_completed() {
        Ok(headers_folder)
    } else {
        Err("failed to get wpimath")
    }
}

pub fn get_wpinet() -> Result<Utf8PathBuf, &'static str> {
    let wpinet_folder = crate::project_root().join("wpinet_cxx");
    let sources_folder = wpinet_folder.join("wpinet/sources");
    let headers_folder = wpinet_folder.join("wpinet/include");

    GOT_WPINET.call_once(|| {
        std::fs::create_dir_all(&sources_folder).unwrap();
        std::fs::create_dir_all(&headers_folder).unwrap();
        use super::{FRC_MAVEN_URL, WPILIB_VERSION};
        super::download_and_extract_zip(&format!("{FRC_MAVEN_URL}/wpinet/wpinet-cpp/{WPILIB_VERSION}/wpinet-cpp-{WPILIB_VERSION}-headers.zip"), &headers_folder).unwrap();
        super::download_and_extract_zip(&format!("{FRC_MAVEN_URL}/wpinet/wpinet-cpp/{WPILIB_VERSION}/wpinet-cpp-{WPILIB_VERSION}-sources.zip"), &sources_folder).unwrap();
        std::fs::remove_dir_all(sources_folder.join("jni")).ok();
    });

    if GOT_WPINET.is_completed() {
        Ok(headers_folder)
    } else {
        Err("failed to get wpinet")
    }
}

pub fn get_wpiutil() -> Result<Utf8PathBuf, &'static str> {
    let wpiutil_folder = crate::project_root().join("wpiutil_sys");
    let sources_folder = wpiutil_folder.join("wpiutil/sources");
    let headers_folder = wpiutil_folder.join("wpiutil/include");

    GOT_WPIUTIL.call_once(|| {
        std::fs::create_dir_all(&sources_folder).unwrap();
        std::fs::create_dir_all(&headers_folder).unwrap();
        use super::{FRC_MAVEN_URL, WPILIB_VERSION};
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

pub fn get_wpi_opencv() -> Result<Utf8PathBuf, &'static str> {
    let wpiutil_folder = crate::project_root().join("wpi_opencv_cxx");
    let sources_folder = wpiutil_folder.join("opencv/sources");
    let headers_folder = wpiutil_folder.join("opencv/include");

    GOT_WPI_OPENCV.call_once(|| {
        std::fs::create_dir_all(&sources_folder).unwrap();
        std::fs::create_dir_all(&headers_folder).unwrap();
        use super::{FRC_MAVEN_URL, OPENCV_VERSION, WPILIB_YEAR};
        super::download_and_extract_zip(&format!("{FRC_MAVEN_URL}/thirdparty/frc{WPILIB_YEAR}/opencv/opencv-cpp/{OPENCV_VERSION}/opencv-cpp-{OPENCV_VERSION}-headers.zip"), &headers_folder).unwrap();
        super::download_and_extract_zip(&format!("{FRC_MAVEN_URL}/thirdparty/frc{WPILIB_YEAR}/opencv/opencv-cpp/{OPENCV_VERSION}/opencv-cpp-{OPENCV_VERSION}-sources.zip"), &sources_folder).unwrap();
        // std::fs::remove_dir_all(sources_folder.join("jni")).ok();
    });

    if GOT_WPI_OPENCV.is_completed() {
        Ok(headers_folder)
    } else {
        Err("failed to get opencv")
    }
}
