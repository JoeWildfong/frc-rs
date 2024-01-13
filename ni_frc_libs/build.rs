use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ni_src = std::env::current_dir()?.join("ni-libraries/src");
    let object_folder = PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("objects");
    std::fs::create_dir_all(&object_folder)?;

    std::fs::copy(ni_src.join("lib/chipobject/libRoboRIO_FRC_ChipObject.so.24.0.0"), object_folder.join("libRoboRIO_FRC_ChipObject.so"))?;
    std::fs::copy(ni_src.join("lib/netcomm/libFRC_NetworkCommunication.so.24.0.0"), object_folder.join("libFRC_NetworkCommunication.so"))?;
    std::fs::copy(ni_src.join("lib/visa/libvisa.so.23.3.0"), object_folder.join("libvisa.so"))?;

    println!("cargo:rustc-link-search={}", object_folder.display());
    println!("cargo:rustc-link-search={}", std::env::current_dir()?.join("built-shims").display());
    println!("cargo:rustc-link-lib=RoboRIO_FRC_ChipObject");
    println!("cargo:rustc-link-lib=FRC_NetworkCommunication");
    println!("cargo:rustc-link-lib=visa");
    println!("cargo:rustc-link-lib=dylib:+verbatim=libNiFpgaLv.so.13");
    println!("cargo:rustc-link-lib=dylib:+verbatim=libnirio_emb_can.so.23");

    println!("cargo:include={}/include", ni_src.display());
    Ok(())
}