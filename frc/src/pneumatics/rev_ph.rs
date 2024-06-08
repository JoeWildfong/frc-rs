use std::sync::Arc;

use uom::si::f64::{ElectricCurrent, ElectricPotential, Pressure};
use wpihal_sys::HAL_REVPHCompressorConfigType;

use super::{
    AnalogCompressor, Compressor, DigitalCompressor, HybridCompressor, SolenoidChannel,
    SolenoidController,
};

pub struct RevPh {
    handle: wpihal_sys::HAL_REVPHHandle,
}

impl RevPh {
    #[must_use]
    pub fn new(can_id: i32) -> Self {
        let handle = unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_InitializeREVPH(can_id, c"".as_ptr(), status)
            })
        };
        Self { handle }
    }

    #[must_use]
    pub fn into_parts(self) -> (RevCompressor, RevPneumatics) {
        let arc = Arc::new(self);
        (
            RevCompressor { ph: Arc::clone(&arc) },
            RevPneumatics {
                channel0: RevChannel0 { ph: Arc::clone(&arc) },
                channel1: RevChannel1 { ph: Arc::clone(&arc) },
                channel2: RevChannel2 { ph: Arc::clone(&arc) },
                channel3: RevChannel3 { ph: Arc::clone(&arc) },
                channel4: RevChannel4 { ph: Arc::clone(&arc) },
                channel5: RevChannel5 { ph: Arc::clone(&arc) },
                channel6: RevChannel6 { ph: Arc::clone(&arc) },
                channel7: RevChannel7 { ph: Arc::clone(&arc) },
                channel8: RevChannel8 { ph: Arc::clone(&arc) },
                channel9: RevChannel9 { ph: Arc::clone(&arc) },
                channel10: RevChannel10 { ph: Arc::clone(&arc) },
                channel11: RevChannel11 { ph: Arc::clone(&arc) },
                channel12: RevChannel12 { ph: Arc::clone(&arc) },
                channel13: RevChannel13 { ph: Arc::clone(&arc) },
                channel14: RevChannel14 { ph: Arc::clone(&arc) },
                channel15: RevChannel15 { ph: arc },
            },
        )
    }
}

impl SolenoidController for RevPh {
    fn get_solenoid_bitset(&self) -> u32 {
        let solenoids = unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_GetREVPHSolenoids(self.handle, status)
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

#[repr(i32)]
#[derive(PartialEq, Eq)]
pub enum RevCompressorMode {
    /// Compressor is always off
    Disabled = wpihal_sys::HAL_REVPHCompressorConfigType_HAL_REVPHCompressorConfigType_kDisabled,
    /// Compressor runs when the connected digital pressure switch is not activated by high pressure
    Digital = wpihal_sys::HAL_REVPHCompressorConfigType_HAL_REVPHCompressorConfigType_kDigital,
    /// Compressor runs when the connected analog pressure sensor reports a value in a specified range
    Analog = wpihal_sys::HAL_REVPHCompressorConfigType_HAL_REVPHCompressorConfigType_kAnalog,
    /// Compressor runs when the requirements under the `Digital` and `Analog` modes are both met
    Hybrid = wpihal_sys::HAL_REVPHCompressorConfigType_HAL_REVPHCompressorConfigType_kHybrid,
}

impl TryFrom<HAL_REVPHCompressorConfigType> for RevCompressorMode {
    type Error = ();
    fn try_from(value: HAL_REVPHCompressorConfigType) -> Result<Self, Self::Error> {
        match value {
            wpihal_sys::HAL_REVPHCompressorConfigType_HAL_REVPHCompressorConfigType_kDisabled => {
                Ok(RevCompressorMode::Disabled)
            }
            wpihal_sys::HAL_REVPHCompressorConfigType_HAL_REVPHCompressorConfigType_kDigital => {
                Ok(RevCompressorMode::Digital)
            }
            wpihal_sys::HAL_REVPHCompressorConfigType_HAL_REVPHCompressorConfigType_kAnalog => {
                Ok(RevCompressorMode::Analog)
            }
            wpihal_sys::HAL_REVPHCompressorConfigType_HAL_REVPHCompressorConfigType_kHybrid => {
                Ok(RevCompressorMode::Hybrid)
            }
            _ => Err(()),
        }
    }
}

pub struct RevCompressor {
    ph: Arc<RevPh>,
}

impl RevCompressor {
    /// Implementes formula from the REV Analog Pressure Sensor datasheet
    /// to convert voltage readings to pressure.
    fn voltage_to_pressure(v_out: ElectricPotential, v_cc: ElectricPotential) -> Pressure {
        // datasheet found at https://www.revrobotics.com/content/docs/REV-11-1107-DS.pdf
        use uom::si::{electric_potential::volt, pressure::psi};
        let p = 250.0 * (v_out.get::<volt>() / v_cc.get::<volt>()) - 25.0;
        Pressure::new::<psi>(p)
    }

    /// Implementes formula from the REV Analog Pressure Sensor datasheet
    // to convert pressure and source voltage to expected output voltage
    fn pressure_to_voltage(pressure: Pressure, v_cc: ElectricPotential) -> ElectricPotential {
        // datasheet found at https://www.revrobotics.com/content/docs/REV-11-1107-DS.pdf
        use uom::si::{electric_potential::volt, pressure::psi};
        let v_out = v_cc.get::<volt>() * (0.004 * pressure.get::<psi>() + 0.1);
        ElectricPotential::new::<volt>(v_out)
    }

    /// Gets the compressor's currently configured mode of operation
    /// # Panics
    /// Panics if the firmware returns an invalid mode of operation
    #[must_use]
    pub fn mode(&self) -> RevCompressorMode {
        let config = unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_GetREVPHCompressorConfig(self.ph.handle, status)
            })
        };
        RevCompressorMode::try_from(config).unwrap()
    }
}

impl Compressor for RevCompressor {
    fn running(&self) -> bool {
        unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_GetREVPHCompressor(self.ph.handle, status)
            }) != 0
        }
    }

    fn current_draw(&self) -> ElectricCurrent {
        let amps = unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_GetREVPHCompressorCurrent(self.ph.handle, status)
            })
        };
        ElectricCurrent::new::<uom::si::electric_current::ampere>(amps)
    }

    fn disable(&mut self) {
        unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_SetREVPHClosedLoopControlDisabled(self.ph.handle, status);
            });
        }
    }
}

impl DigitalCompressor for RevCompressor {
    fn get_pressure_switch(&self) -> bool {
        unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_GetREVPHPressureSwitch(self.ph.handle, status)
            }) != 0
        }
    }

    fn enable_digital(&mut self) {
        unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_SetREVPHClosedLoopControlDigital(self.ph.handle, status);
            });
        }
    }

    fn in_digital_mode(&self) -> bool {
        self.mode() == RevCompressorMode::Digital
    }
}

impl AnalogCompressor for RevCompressor {
    fn pressure(&self) -> Pressure {
        use uom::si::electric_potential::volt;
        // calculate the pressure in psi per the REV Analog Pressure Sensor datasheet
        // datasheet found at https://www.revrobotics.com/content/docs/REV-11-1107-DS.pdf
        // the datasheet doesn't say it's in psi, it's assumed from wpihal's implementation
        let v_out = unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_GetREVPHAnalogVoltage(self.ph.handle, 0, status)
            })
        };
        let v_cc = unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_GetREVPH5VVoltage(self.ph.handle, status)
            })
        };
        Self::voltage_to_pressure(
            ElectricPotential::new::<volt>(v_out),
            ElectricPotential::new::<volt>(v_cc),
        )
    }

    fn enable_analog(&mut self, min_pressure: Pressure, max_pressure: Pressure) {
        use uom::si::electric_potential::volt;
        let min_voltage =
            Self::pressure_to_voltage(min_pressure, ElectricPotential::new::<volt>(5.0));
        let max_voltage =
            Self::pressure_to_voltage(max_pressure, ElectricPotential::new::<volt>(5.0));
        unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_SetREVPHClosedLoopControlAnalog(
                    self.ph.handle,
                    min_voltage.get::<volt>(),
                    max_voltage.get::<volt>(),
                    status,
                );
            });
        }
    }

    fn in_analog_mode(&self) -> bool {
        self.mode() == RevCompressorMode::Analog
    }
}

impl HybridCompressor for RevCompressor {
    fn enable_hybrid(&mut self, min_pressure: Pressure, max_pressure: Pressure) {
        use uom::si::electric_potential::volt;
        let min_voltage =
            Self::pressure_to_voltage(min_pressure, ElectricPotential::new::<volt>(5.0));
        let max_voltage =
            Self::pressure_to_voltage(max_pressure, ElectricPotential::new::<volt>(5.0));
        unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_SetREVPHClosedLoopControlHybrid(
                    self.ph.handle,
                    min_voltage.get::<volt>(),
                    max_voltage.get::<volt>(),
                    status,
                );
            });
        }
    }

    fn in_hybrid_mode(&self) -> bool {
        self.mode() == RevCompressorMode::Hybrid
    }
}

macro_rules! rev_channel {
    ($name:ident, $num:expr) => {
        pub struct $name {
            ph: Arc<RevPh>,
        }

        impl SolenoidChannel for $name {
            const CHANNEL: u32 = $num;
            type Controller = RevPh;

            fn into_controller(self) -> Arc<Self::Controller> {
                self.ph
            }

            unsafe fn new(controller: Arc<Self::Controller>) -> Self {
                Self { ph: controller }
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

pub struct RevPneumatics {
    pub channel0: RevChannel0,
    pub channel1: RevChannel1,
    pub channel2: RevChannel2,
    pub channel3: RevChannel3,
    pub channel4: RevChannel4,
    pub channel5: RevChannel5,
    pub channel6: RevChannel6,
    pub channel7: RevChannel7,
    pub channel8: RevChannel8,
    pub channel9: RevChannel9,
    pub channel10: RevChannel10,
    pub channel11: RevChannel11,
    pub channel12: RevChannel12,
    pub channel13: RevChannel13,
    pub channel14: RevChannel14,
    pub channel15: RevChannel15,
}
