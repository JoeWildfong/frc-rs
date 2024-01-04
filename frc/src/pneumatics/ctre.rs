use std::ffi::CStr;

use super::{PneumaticsController, Compressor};

pub struct CtrePcm {
    handle: wpihal_ffi::HAL_CTREPCMHandle,
}

impl CtrePcm {
    pub fn new(can_id: i32) -> Self {
        let handle = unsafe {
            wpihal_ffi::panic_on_hal_error(|status| {
                wpihal_ffi::HAL_InitializeCTREPCM(
                    can_id,
                    CStr::from_bytes_with_nul(b"\0").unwrap().as_ptr(),
                    status,
                )
            })
        };
        Self {
            handle,
        }
    }

    pub fn as_parts(&mut self) -> (CtreCompressor<'_>, CtrePneumatics<'_>) {
        (
            CtreCompressor { pcm: self },
            CtrePneumatics {
                channel0: CtreChannel0 { pcm: self },
                channel1: CtreChannel1 { pcm: self },
                channel2: CtreChannel2 { pcm: self },
                channel3: CtreChannel3 { pcm: self },
                channel4: CtreChannel4 { pcm: self },
                channel5: CtreChannel5 { pcm: self },
                channel6: CtreChannel6 { pcm: self },
                channel7: CtreChannel7 { pcm: self },
            }
        )
    }
}

impl PneumaticsController for CtrePcm {
    fn get_solenoids(&self) -> u32 {
        let solenoids = unsafe {
            wpihal_ffi::panic_on_hal_error(|status| {
                wpihal_ffi::HAL_GetCTREPCMSolenoids(self.handle, status)
            })
        };
        solenoids as u32
    }

    fn set_solenoids(&self, mask: u32, values: u32) {
        unsafe {
            wpihal_ffi::panic_on_hal_error(|status| {
                wpihal_ffi::HAL_SetCTREPCMSolenoids(
                    self.handle,
                    mask as i32,
                    values as i32,
                    status,
                );
            });
        };
    }
}

impl Default for CtrePcm {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Drop for CtrePcm {
    fn drop(&mut self) {
        unsafe { wpihal_ffi::HAL_FreeCTREPCM(self.handle) }
    }
}

pub struct CtreCompressor<'a> {
    pcm: &'a CtrePcm,
}

impl<'a> Compressor for CtreCompressor<'a> {
    fn get_enabled(&self) -> bool {
        unsafe {
            wpihal_ffi::panic_on_hal_error(|status| {
                wpihal_ffi::HAL_GetCTREPCMCompressor(self.pcm.handle, status)
            }) != 0
        }
    }

    fn get_pressure_switch(&self) -> bool {
        unsafe {
            wpihal_ffi::panic_on_hal_error(|status| {
                wpihal_ffi::HAL_GetCTREPCMPressureSwitch(self.pcm.handle, status)
            }) != 0
        }
    }
}

macro_rules! ctre_channel {
    ($name:ident, $num:expr) => {
        pub struct $name<'a> {
            pcm: &'a CtrePcm,
        }

        impl<'a> super::MaskBasedChannel for $name<'a> {
            const MASK: u32 = 1 << $num;
            type Controller = CtrePcm;

            fn get_controller(&self) -> &Self::Controller {
                self.pcm
            }
        }
    }
}

ctre_channel!(CtreChannel0, 0);
ctre_channel!(CtreChannel1, 1);
ctre_channel!(CtreChannel2, 2);
ctre_channel!(CtreChannel3, 3);
ctre_channel!(CtreChannel4, 4);
ctre_channel!(CtreChannel5, 5);
ctre_channel!(CtreChannel6, 6);
ctre_channel!(CtreChannel7, 7);

pub struct CtrePneumatics<'a> {
    pub channel0: CtreChannel0<'a>,
    pub channel1: CtreChannel1<'a>,
    pub channel2: CtreChannel2<'a>,
    pub channel3: CtreChannel3<'a>,
    pub channel4: CtreChannel4<'a>,
    pub channel5: CtreChannel5<'a>,
    pub channel6: CtreChannel6<'a>,
    pub channel7: CtreChannel7<'a>,
}
