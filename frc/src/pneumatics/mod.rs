//! # Pneumatics Control
//!
//! This module defines common interfaces for access to pneumatic hardware.
//! For concrete implementations, see vendor-specific submodules.
//! - Support for CTRE's Pneumatics Control Module is contained in the [`ctre_pcm`] module, via the [`CtrePcm`](ctre_pcm::CtrePcm) type.
//! - Support for REV's Pneumatics Hub is contained in the [`rev_ph`] module, via the [`RevPh`](rev_ph::RevPh) type.
//!
//! Each concrete implementation provides a way to obtain solenoid channels implementing [`SolenoidChannel`].
//! These can be used to create single- and double-ended solenoids, types for which are provided in this module.
//! The basic version of each is [`TypedSolenoid`] and [`TypedDoubleSolenoid`].
//!
//! Similarly to [GPIO pins] in many embedded crates, a different type may be provided for each solenoid channel.
//! If needed, this channel information can be erased in a [`ChannelErasedSolenoid`] or [`ChannelErasedDoubleSolenoid`],
//! e.g. for homogeneous storage of several solenoids in a slice or array.
//! The controller type can also be erased into the [`AnySolenoid`] and [`AnyDoubleSolenoid`] types.
//!
//! [GPIO pins]: https://doc.rust-lang.org/stable/embedded-book/design-patterns/hal/gpio.html

pub mod compressor;
pub use compressor::*;
pub mod solenoid;
pub use solenoid::*;

pub mod ctre_pcm;
pub mod rev_ph;
