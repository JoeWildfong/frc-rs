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

pub struct RevCompressor<'a> {
    ph: &'a RevPh,
}

impl<'a> RevCompressor<'a> {
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

impl<'a> Compressor for RevCompressor<'a> {
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

impl<'a> DigitalCompressor for RevCompressor<'a> {
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

impl<'a> AnalogCompressor for RevCompressor<'a> {
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

impl<'a> HybridCompressor for RevCompressor<'a> {
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
        pub struct $name<'a> {
            ph: &'a RevPh,
        }

        impl<'a> SolenoidChannel<'a> for $name<'a> {
            const CHANNEL: u32 = $num;
            type Controller = RevPh;

            fn get_controller(&self) -> &'a Self::Controller {
                self.ph
            }

            unsafe fn new(controller: &'a Self::Controller) -> Self {
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
