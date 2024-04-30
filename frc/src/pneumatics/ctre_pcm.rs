use super::{Compressor, DigitalCompressor, SolenoidChannel, SolenoidController};

pub struct CtrePcm {
    handle: wpihal_sys::HAL_CTREPCMHandle,
}

impl CtrePcm {
    #[must_use]
    pub fn new(can_id: i32) -> Self {
        let handle = unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_InitializeCTREPCM(can_id, c"".as_ptr(), status)
            })
        };
        Self { handle }
    }

    #[must_use]
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
            },
        )
    }
}

impl SolenoidController for CtrePcm {
    fn get_solenoid_bitset(&self) -> u32 {
        let solenoids = unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_GetCTREPCMSolenoids(self.handle, status)
            })
        };
        #[allow(clippy::cast_sign_loss)]
        {
            solenoids as u32
        }
    }

    fn set_solenoid_bitset(&self, mask: u32, values: u32) {
        unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                #[allow(clippy::cast_possible_wrap)]
                wpihal_sys::HAL_SetCTREPCMSolenoids(
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
        unsafe { wpihal_sys::HAL_FreeCTREPCM(self.handle) }
    }
}

pub struct CtreCompressor<'a> {
    pcm: &'a CtrePcm,
}

impl<'a> Compressor for CtreCompressor<'a> {
    fn running(&self) -> bool {
        unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_GetCTREPCMCompressor(self.pcm.handle, status)
            }) != 0
        }
    }

    fn current_draw(&self) -> uom::si::f64::ElectricCurrent {
        let amps = unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_GetCTREPCMCompressorCurrent(self.pcm.handle, status)
            })
        };
        uom::si::f64::ElectricCurrent::new::<uom::si::electric_current::ampere>(amps)
    }

    fn disable(&mut self) {
        unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_SetCTREPCMClosedLoopControl(self.pcm.handle, 0, status);
            });
        }
    }
}

impl<'a> DigitalCompressor for CtreCompressor<'a> {
    fn get_pressure_switch(&self) -> bool {
        unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_GetCTREPCMPressureSwitch(self.pcm.handle, status)
            }) != 0
        }
    }

    fn enable_digital(&mut self) {
        unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_SetCTREPCMClosedLoopControl(self.pcm.handle, 1, status);
            });
        }
    }

    fn in_digital_mode(&self) -> bool {
        unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_GetCTREPCMClosedLoopControl(self.pcm.handle, status)
            }) != 0
        }
    }
}

macro_rules! ctre_channel {
    ($name:ident, $num:expr) => {
        pub struct $name<'a> {
            pcm: &'a CtrePcm,
        }

        impl<'a> SolenoidChannel<'a> for $name<'a> {
            const CHANNEL: u32 = $num;
            type Controller = CtrePcm;

            fn get_controller(&self) -> &'a Self::Controller {
                self.pcm
            }

            unsafe fn new(controller: &'a Self::Controller) -> Self {
                Self { pcm: controller }
            }
        }
    };
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
