#![allow(clippy::module_name_repetitions)]

use uom::si::f64::{ElectricCurrent, Pressure};

/// Common functionality expected in all pneumatic compressors.
/// This trait is not very useful on its own - compressors will generally implement
/// at least one of [`DigitalCompressor`], [`AnalogCompressor`] and [`HybridCompressor`]
/// to expose more functionality.
/// Compressors may also expose specific functionality not included in these traits.
pub trait Compressor {
    /// Gets whether the compressor is currently compressing air.
    #[must_use]
    fn running(&self) -> bool;

    /// Gets the current drawn by the compressor at this moment.
    #[must_use]
    fn current_draw(&self) -> ElectricCurrent;

    /// Disables the compressor, turning it off and preventing it from running.
    fn disable(&mut self);
}

/// For compressors with a connected digital pressure switch and digital operation mode.
///
/// The pressure switch is externally configured to a target pressure and can
/// report whether or not the actual pressure exceeds the specified pressure.
/// When the compressor is in digital mode, it will run only when the pressure switch
/// reports the pressure is below the target pressure.
pub trait DigitalCompressor: Compressor {
    /// Gets the state of the pressure switch connected to the compressor.
    /// TODO: is the following true? underdocumented in wpilib, needs testing
    /// A true value indicates the pressure exceeds the switch's configured pressure.
    #[must_use]
    fn get_pressure_switch(&self) -> bool;

    /// Enables the compressor in digital mode. It will run while the pressure is
    /// below the limit imposed by the connected pressure switch and turn off when
    /// the pressure exceeds the limit.
    fn enable_digital(&mut self);

    /// Returns whether the compressor is currently configured in digital mode.
    #[must_use]
    fn in_digital_mode(&self) -> bool;
}

/// For compressors with a connected analog pressure sensor and analog operation mode.
///
/// The pressure sensor can report the current pressure, and a pressure range can
/// be given to the compressor. The compressor will run when the pressure, as
/// reported by the sensor, is within the specified range.
pub trait AnalogCompressor: Compressor {
    /// Gets the pressure in the system, as reported by the pressure sensor.
    #[must_use]
    fn pressure(&self) -> Pressure;

    /// Enables the compressor in analog mode. It will run while the pressure sensor
    /// reports a pressure between `min_pressure` and `max_pressure`.
    fn enable_analog(&mut self, min_pressure: Pressure, max_pressure: Pressure);

    /// Returns whether the compressor is currently configured in analog mode.
    #[must_use]
    fn in_analog_mode(&self) -> bool;
}

/// For compressors with both a digital pressure switch and analog pressure sensor,
/// and a hybrid operation mode.
///
/// The compressor will run when the pressure is within the specified range, and
/// below the digital switch's externally configured target pressure.
pub trait HybridCompressor: DigitalCompressor + AnalogCompressor {
    /// Enables the compressor in hybrid mode. It will run while the pressure sensor
    /// reports a pressure between `min_pressure` and `max_pressure`, and the
    /// digital switch's pressure target is not exceeded.
    fn enable_hybrid(&mut self, min_pressure: Pressure, max_pressure: Pressure);

    /// Returns whether the compressor is currently configured in hybrid mode.
    #[must_use]
    fn in_hybrid_mode(&self) -> bool;
}
