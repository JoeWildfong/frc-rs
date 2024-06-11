// wpihal DIO channels:
//  channels 0-9: regular DIO pins
//  channels 10-25: MXP (expansion port) pins
//      all channels can be used for either DIO or another function
//      10..=13 - PWM 0..=3
//      14 - SPI CS
//      15 - SPI CLK
//      16 - SPI MISO
//      17 - SPI MOSI
//      18..=23 - PWM 4..=9
//      24 - I2C SCL
//      25 - I2C SDA
//      note: the MXP also has analog pins, which don't appear here (they are analog channels)
//  channels 26-30: SPI port (each pin can also be used for normal DIO)
//      CS0 does not use a channel, writes to /dev/spidev0.0 instead (wtf?)
//      26 - CS1
//      27 - CS2
//      28 - CS3
//      29 - MOSI
//      30 - MISO
//  channel 31: unused
// wpihal has a comment saying MXP is 10-26 and SPI is 27-31, these appear to be incorrect
// ni's chipobject doesn't use this layout - it uses a c++ bitfield
// wpihal has functions to convert a channel to a bit in ni's bitfield

use std::{
    marker::PhantomData,
    sync::atomic::{AtomicBool, Ordering},
};

use embedded_hal::digital::{ErrorType, InputPin, OutputPin, PinState};
use embedded_hal_async::digital::Wait;
use wpihal_sys::panic_on_hal_error;

use crate::reactor::dio::EdgeType;

pub trait PinMode: crate::Sealed {}

impl crate::Sealed for Input {}
impl PinMode for Input {}

impl crate::Sealed for Output {}
impl PinMode for Output {}

impl crate::Sealed for Uninitialized {}
impl PinMode for Uninitialized {}

/// Input pin mode (typestate)
pub struct Input;

/// Output pin mode (typestate)
pub struct Output;

/// Uninitialized pin mode (typestate)
pub struct Uninitialized;

/// Represents a DIO pin on the built-in port
/// N is the pin number from 0..=9
pub struct Dio<const N: u8, MODE: PinMode = Uninitialized> {
    // sigh... no ZST for me
    handle: wpihal_sys::HAL_DigitalHandle,
    _mode: PhantomData<MODE>,
}

impl<const N: u8, MODE: PinMode> ErrorType for Dio<N, MODE> {
    type Error = std::convert::Infallible;
}

impl<const N: u8> Dio<N, Uninitialized> {
    /// assert that N is in 0..=9
    const _VALID: () = assert!(N < 10);

    /// Creates a new digital pin. It is not recommended to use this function
    /// unless you can be sure that no other object representing this pin exists.
    /// Violating this will not result in undefined behaviour, but can lead to
    /// races if both objects attempt to change the pin state.
    pub const fn new_uninit() -> Self {
        Self {
            handle: 0,
            _mode: PhantomData,
        }
    }

    pub fn into_input(self) -> Dio<N, Input> {
        let handle = unsafe {
            panic_on_hal_error(|status| {
                wpihal_sys::HAL_InitializeDIOPort(
                    wpihal_sys::HAL_GetPort(N.into()),
                    1,
                    c"".as_ptr(),
                    status,
                )
            })
        };
        Dio {
            handle,
            _mode: PhantomData,
        }
    }

    pub fn into_output(self) -> Dio<N, Output> {
        let handle = unsafe {
            panic_on_hal_error(|status| {
                wpihal_sys::HAL_InitializeDIOPort(
                    wpihal_sys::HAL_GetPort(N.into()),
                    0,
                    c"".as_ptr(),
                    status,
                )
            })
        };
        Dio {
            handle,
            _mode: PhantomData,
        }
    }
}

impl<const N: u8> Dio<N, Input> {
    fn read(&self) -> PinState {
        let val =
            unsafe { panic_on_hal_error(|status| wpihal_sys::HAL_GetDIO(self.handle, status)) };
        PinState::from(val != 0)
    }

    pub fn into_output(self) -> Dio<N, Output> {
        unsafe {
            panic_on_hal_error(|status| {
                wpihal_sys::HAL_SetDIODirection(self.handle, 0, status);
            });
        }
        Dio {
            handle: self.handle,
            _mode: PhantomData,
        }
    }
}

impl<const N: u8> InputPin for Dio<N, Input> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.read() == PinState::High)
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.read() == PinState::Low)
    }
}

impl<const N: u8> Dio<N, Output> {
    fn write(&mut self, state: PinState) {
        unsafe {
            panic_on_hal_error(|status| {
                wpihal_sys::HAL_SetDIO(self.handle, i32::from(bool::from(state)), status);
            });
        }
    }

    pub fn into_uninit(self) -> Dio<N, Uninitialized> {
        unsafe { wpihal_sys::HAL_FreeDIOPort(self.handle) };
        Dio {
            handle: 0,
            _mode: PhantomData,
        }
    }

    pub fn into_input(self) -> Dio<N, Input> {
        unsafe {
            panic_on_hal_error(|status| {
                wpihal_sys::HAL_SetDIODirection(self.handle, 1, status);
            });
        }
        Dio {
            handle: self.handle,
            _mode: PhantomData,
        }
    }
}

impl<const N: u8> OutputPin for Dio<N, Output> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.write(PinState::High);
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.write(PinState::Low);
        Ok(())
    }

    fn set_state(&mut self, state: PinState) -> Result<(), Self::Error> {
        self.write(state);
        Ok(())
    }
}

impl<const N: u8> Wait for Dio<N, Input> {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        if self.is_high().unwrap() {
            return Ok(());
        }
        self.wait_for_rising_edge().await
    }

    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        if self.is_low().unwrap() {
            return Ok(());
        }
        self.wait_for_falling_edge().await
    }

    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        crate::reactor::dio::wait_for_edge(self.handle, EdgeType::Rising)
            .await
            .unwrap_or_else(|e| {
                eprintln!("error waiting for pin {N}: {e}");
            });
        Ok(())
    }

    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        crate::reactor::dio::wait_for_edge(self.handle, EdgeType::Falling)
            .await
            .unwrap_or_else(|e| {
                eprintln!("error waiting for pin {N}: {e}");
            });
        Ok(())
    }

    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        crate::reactor::dio::wait_for_edge(self.handle, EdgeType::Either)
            .await
            .unwrap_or_else(|e| {
                eprintln!("error waiting for pin {N}: {e}");
            });
        Ok(())
    }
}

impl<const N: u8, MODE: PinMode> Drop for Dio<N, MODE> {
    fn drop(&mut self) {
        if self.handle != 0 {
            unsafe { wpihal_sys::HAL_FreeDIOPort(self.handle) }
        }
    }
}

pub type Dio1 = Dio<1>;
pub type Dio2 = Dio<2>;
pub type Dio3 = Dio<3>;
pub type Dio4 = Dio<4>;
pub type Dio5 = Dio<5>;
pub type Dio6 = Dio<6>;
pub type Dio7 = Dio<7>;
pub type Dio8 = Dio<8>;
pub type Dio9 = Dio<9>;

pub struct DioPort {
    pub dio1: Dio1,
    pub dio2: Dio2,
    pub dio3: Dio3,
    pub dio4: Dio4,
    pub dio5: Dio5,
    pub dio6: Dio6,
    pub dio7: Dio7,
    pub dio8: Dio8,
    pub dio9: Dio9,
}

static PORT_TAKEN: AtomicBool = AtomicBool::new(false);

impl DioPort {
    pub fn take() -> Option<Self> {
        let previously_taken = PORT_TAKEN.swap(true, Ordering::Relaxed);
        if previously_taken {
            None
        } else {
            Some(Self {
                dio1: Dio1::new_uninit(),
                dio2: Dio2::new_uninit(),
                dio3: Dio3::new_uninit(),
                dio4: Dio4::new_uninit(),
                dio5: Dio5::new_uninit(),
                dio6: Dio6::new_uninit(),
                dio7: Dio7::new_uninit(),
                dio8: Dio8::new_uninit(),
                dio9: Dio9::new_uninit(),
            })
        }
    }
}
