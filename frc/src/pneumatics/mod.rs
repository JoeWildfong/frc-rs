//! # Pneumatics Control
//!
//! This module defines common interfaces for access to pneumatic hardware.
//! For concrete implementations, see vendor-specific submodules.
//! - Support for CTRE's Pneumatics Control Module is contained in the [ctre] module, via the [CtrePcm](ctre::CtrePcm) type.
//! - Support for REV's Pneumatics Hub is contained in the [rev] module, via the [RevPh](rev::RevPh) type.
//!
//! Each concrete implementation provides a way to obtain types implementing [Solenoid] and [DoubleSolenoid].
//! These can be used to actuate single- and double-ended solenoids, respectively.
//! Similarly to [gpio pins] in many embedded crates, different types may be used for each solenoid channel.
//! If needed, the channel and controller type can be erased into this module's
//! [AnySolenoid] and [AnyDoubleSolenoid] types.
//! Additionally, just the channel can be erased into a vendor-specific type that stores the channel at runtime.
//!
//! [gpio pins]: https://doc.rust-lang.org/stable/embedded-book/design-patterns/hal/gpio.html

pub mod ctre;
pub mod rev;

/// A solenoid, which can be actuated in one direction or released.
pub trait Solenoid {
    /// Gets the state of the solenoid.
    /// True indicates the solenoid is currently actuated, false indicates released.
    fn get(&self) -> bool;

    /// Sets the state of the solenoid.
    /// True indicates to actuate the solenoid, false indicates to release.
    fn set(&mut self, state: bool);
}

/// Enumeration of all possible states for a [DoubleSolenoid].
pub enum DoubleSolenoidState {
    Forward,
    Backward,
    Off,
}

/// Represents the state of a double solenoid actuated on both ends.
/// This is invalid but possible, so this type is returned from [DoubleSolenoid::get] in this case.
pub struct InvalidDoubleSolenoidState;

/// A double solenoid, which can be actuated in either direction or released.
pub trait DoubleSolenoid {
    /// Gets the state of the double solenoid. If the solenoid is actuated on both ends, [InvalidDoubleSolenoidState] is returned.
    fn get(&self) -> Result<DoubleSolenoidState, InvalidDoubleSolenoidState>;

    /// Sets the state of the double solenoid.
    fn set(&mut self, state: DoubleSolenoidState);
}

enum GenericSolenoid<'a> {
    Rev(rev::AnyRevSolenoid<'a>),
    Ctre(ctre::AnyCtreSolenoid<'a>),
}

/// A wrapper around any kind of solenoid, storing all information needed for its operation
/// (e.g. controller, channel) at runtime.
pub struct AnySolenoid<'a> {
    solenoid: GenericSolenoid<'a>,
}

impl<'a> Solenoid for AnySolenoid<'a> {
    fn get(&self) -> bool {
        match self.solenoid {
            GenericSolenoid::Rev(ref s) => s.get(),
            GenericSolenoid::Ctre(ref s) => s.get(),
        }
    }

    fn set(&mut self, state: bool) {
        match self.solenoid {
            GenericSolenoid::Rev(ref mut s) => s.set(state),
            GenericSolenoid::Ctre(ref mut s) => s.set(state),
        }
    }
}

enum GenericDoubleSolenoid<'a> {
    Rev(rev::AnyRevDoubleSolenoid<'a>),
    Ctre(ctre::AnyCtreDoubleSolenoid<'a>),
}

/// A wrapper around any kind of double solenoid, storing all information needed for its operation
/// (e.g. controller, channel) at runtime.
pub struct AnyDoubleSolenoid<'a> {
    solenoid: GenericDoubleSolenoid<'a>,
}

impl<'a> DoubleSolenoid for AnyDoubleSolenoid<'a> {
    fn get(&self) -> Result<DoubleSolenoidState, InvalidDoubleSolenoidState> {
        match self.solenoid {
            GenericDoubleSolenoid::Rev(ref s) => s.get(),
            GenericDoubleSolenoid::Ctre(ref s) => s.get(),
        }
    }

    fn set(&mut self, state: DoubleSolenoidState) {
        match self.solenoid {
            GenericDoubleSolenoid::Rev(ref mut s) => s.set(state),
            GenericDoubleSolenoid::Ctre(ref mut s) => s.set(state),
        }
    }
}

pub trait Compressor {
    fn get_enabled(&self) -> bool;
    fn get_pressure_switch(&self) -> bool;
}
