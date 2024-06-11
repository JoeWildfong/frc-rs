use std::sync::Arc;

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
    pub fn into_parts(self) -> (CtreCompressor, CtrePneumatics) {
        let arc = Arc::new(self);
        (
            CtreCompressor {
                pcm: Arc::clone(&arc),
            },
            CtrePneumatics {
                channel0: CtreChannel0 {
                    pcm: Arc::clone(&arc),
                },
                channel1: CtreChannel1 {
                    pcm: Arc::clone(&arc),
                },
                channel2: CtreChannel2 {
                    pcm: Arc::clone(&arc),
                },
                channel3: CtreChannel3 {
                    pcm: Arc::clone(&arc),
                },
                channel4: CtreChannel4 {
                    pcm: Arc::clone(&arc),
                },
                channel5: CtreChannel5 {
                    pcm: Arc::clone(&arc),
                },
                channel6: CtreChannel6 {
                    pcm: Arc::clone(&arc),
                },
                channel7: CtreChannel7 { pcm: arc },
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

pub struct CtreCompressor {
    pcm: Arc<CtrePcm>,
}

impl Compressor for CtreCompressor {
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

impl DigitalCompressor for CtreCompressor {
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
        pub struct $name {
            pcm: Arc<CtrePcm>,
        }

        impl SolenoidChannel for $name {
            const CHANNEL: u32 = $num;
            type Controller = CtrePcm;

            fn into_controller(self) -> Arc<Self::Controller> {
                self.pcm
            }

            unsafe fn new(controller: Arc<Self::Controller>) -> Self {
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

pub struct CtrePneumatics {
    pub channel0: CtreChannel0,
    pub channel1: CtreChannel1,
    pub channel2: CtreChannel2,
    pub channel3: CtreChannel3,
    pub channel4: CtreChannel4,
    pub channel5: CtreChannel5,
    pub channel6: CtreChannel6,
    pub channel7: CtreChannel7,
}
