
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::all)]
#![allow(clippy::pedantic)]

pub mod FRC_FPGA_ChipObject {
    include!("chipobject_bindings.rs");
}

pub mod FRC_NetworkCommunication {
    include!("netcomm_bindings.rs");
}

pub mod visa {
    include!("visa_bindings.rs");
}

pub mod shims {
    // embcanshim
    #[no_mangle]
    extern "C" fn niEmbCANCloseSession() {}
    #[no_mangle]
    extern "C" fn niEmbCANWrite() {}
    #[no_mangle]
    extern "C" fn niEmbCANOpenSession() {}
    #[no_mangle]
    extern "C" fn niEmbCANStart() {}
    #[no_mangle]
    extern "C" fn niEmbCANGetProperty() {}
    #[no_mangle]
    extern "C" fn niEmbCANStop() {}
    #[no_mangle]
    extern "C" fn niEmbCANRead() {}

    // fpgalvshim
    #[no_mangle]
    extern "C" fn NiFpgaLv_Open() {}
}