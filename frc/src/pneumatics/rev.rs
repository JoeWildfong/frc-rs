use std::ffi::CStr;

use super::{Compressor, PneumaticsController};

pub struct RevPh {
    handle: wpihal_sys::HAL_REVPHHandle,
}

impl RevPh {
    pub fn new(can_id: i32) -> Self {
        let handle = unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_InitializeREVPH(
                    can_id,
                    CStr::from_bytes_with_nul(b"\0").unwrap().as_ptr(),
                    status,
                )
            })
        };
        Self { handle }
    }

    pub fn as_parts(&mut self) -> (RevCompressor<'_>, RevPneumatics<'_>) {
        (
            RevCompressor { ph: self },
            RevPneumatics {
                channel0: RevChannel0 { ph: self },
                channel1: RevChannel1 { ph: self },
                channel2: RevChannel2 { ph: self },
                channel3: RevChannel3 { ph: self },
                channel4: RevChannel4 { ph: self },
                channel5: RevChannel5 { ph: self },
                channel6: RevChannel6 { ph: self },
                channel7: RevChannel7 { ph: self },
                channel8: RevChannel8 { ph: self },
                channel9: RevChannel9 { ph: self },
                channel10: RevChannel10 { ph: self },
                channel11: RevChannel11 { ph: self },
                channel12: RevChannel12 { ph: self },
                channel13: RevChannel13 { ph: self },
                channel14: RevChannel14 { ph: self },
                channel15: RevChannel15 { ph: self },
            },
        )
    }
}

impl PneumaticsController for RevPh {
    fn get_solenoids(&self) -> u32 {
        let solenoids = unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_GetREVPHSolenoids(self.handle, status)
            })
        };
        solenoids as u32
    }

    fn set_solenoids(&self, mask: u32, values: u32) {
        unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_SetREVPHSolenoids(self.handle, mask as i32, values as i32, status);
            });
        };
    }
}

impl Default for RevPh {
    fn default() -> Self {
        Self::new(1)
    }
}

impl Drop for RevPh {
    fn drop(&mut self) {
        unsafe { wpihal_sys::HAL_FreeREVPH(self.handle) }
    }
}

pub struct RevCompressor<'a> {
    ph: &'a RevPh,
}

impl<'a> Compressor for RevCompressor<'a> {
    fn get_enabled(&self) -> bool {
        unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_GetREVPHCompressor(self.ph.handle, status)
            }) != 0
        }
    }

    fn get_pressure_switch(&self) -> bool {
        unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_GetREVPHPressureSwitch(self.ph.handle, status)
            }) != 0
        }
    }
}

macro_rules! rev_channel {
    ($name:ident, $num:expr) => {
        pub struct $name<'a> {
            ph: &'a RevPh,
        }

        impl<'a> super::MaskBasedChannel for $name<'a> {
            const MASK: u32 = 1 << $num;
            type Controller = RevPh;

            fn get_controller(&self) -> &Self::Controller {
                self.ph
            }
        }
    };
}

rev_channel!(RevChannel0, 0);
rev_channel!(RevChannel1, 1);
rev_channel!(RevChannel2, 2);
rev_channel!(RevChannel3, 3);
rev_channel!(RevChannel4, 4);
rev_channel!(RevChannel5, 5);
rev_channel!(RevChannel6, 6);
rev_channel!(RevChannel7, 7);
rev_channel!(RevChannel8, 8);
rev_channel!(RevChannel9, 9);
rev_channel!(RevChannel10, 10);
rev_channel!(RevChannel11, 11);
rev_channel!(RevChannel12, 12);
rev_channel!(RevChannel13, 13);
rev_channel!(RevChannel14, 14);
rev_channel!(RevChannel15, 15);

pub struct RevPneumatics<'a> {
    pub channel0: RevChannel0<'a>,
    pub channel1: RevChannel1<'a>,
    pub channel2: RevChannel2<'a>,
    pub channel3: RevChannel3<'a>,
    pub channel4: RevChannel4<'a>,
    pub channel5: RevChannel5<'a>,
    pub channel6: RevChannel6<'a>,
    pub channel7: RevChannel7<'a>,
    pub channel8: RevChannel8<'a>,
    pub channel9: RevChannel9<'a>,
    pub channel10: RevChannel10<'a>,
    pub channel11: RevChannel11<'a>,
    pub channel12: RevChannel12<'a>,
    pub channel13: RevChannel13<'a>,
    pub channel14: RevChannel14<'a>,
    pub channel15: RevChannel15<'a>,
}
