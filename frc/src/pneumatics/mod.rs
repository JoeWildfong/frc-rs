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

use std::marker::PhantomData;

pub mod ctre_pcm;
pub mod rev_ph;

/// Common functionality between solenoid controllers.
/// Assumes that solenoid states are represented as a bitset, with each bit storing
/// the state of an individual solenoid.
pub trait SolenoidController {
    /// Gets the current state of all solenoids as a bitset (i.e. if bit 0 is set,
    /// the solenoid in channel 0 is currently actuated)
    fn get_solenoid_bitset(&self) -> u32;

    /// Sets the solenoid bitset through a mask. For each solenoid, this sets its
    /// state to the value in `values` if its bit in `mask` is set.
    /// ```rust, no_run
    /// controller.set_solenoid_bitset(u32::MAX, 0);
    /// assert_eq!(controller.get_solenoid_bitset(), 0);
    /// controller.set_solenoid_bitset(0xF000), u32::MAX);
    /// assert_eq!(controller.get_solenoid_bitset(), 0xF000);
    /// controller.set_solenoid_bitset(0x00FF, 0x0F0F);
    /// assert_eq!(controller.get_solenoid_bitset(), 0xF00F);
    /// ```
    fn set_solenoid_bitset(&self, mask: u32, values: u32);

    /// Gets the state of a single solenoid based on [SolenoidController::get_solenoid_bitset].
    fn get_solenoid(&self, channel: u32) -> bool {
        self.get_solenoid_bitset() & (1 << channel) != 0
    }

    /// Sets the state of a single solenoid based on [SolenoidController::set_solenoid_bitset].
    fn set_solenoid(&self, channel: u32, state: bool) {
        self.set_solenoid_bitset(1 << channel, if state { u32::MAX } else { 0 });
    }

    /// Gets the state of a double solenoid based on [SolenoidController::get_solenoid_bitset].
    /// Returns an [InvalidDoubleSolenoidState] if both channels of the double solenoid
    /// are actuated (i.e. both ends of the cylinder are firing), otherwise a [DoubleSolenoidState].
    fn get_double_solenoid(
        &self,
        forward_channel: u32,
        backward_channel: u32,
    ) -> Result<DoubleSolenoidState, InvalidDoubleSolenoidState> {
        let masked = self.get_solenoid_bitset() & (1 << forward_channel) | (1 << backward_channel);
        match masked {
            _ if masked == (1 << forward_channel) => Ok(DoubleSolenoidState::Forward),
            _ if masked == (1 << backward_channel) => Ok(DoubleSolenoidState::Backward),
            0 => Ok(DoubleSolenoidState::Off),
            _ => Err(InvalidDoubleSolenoidState),
        }
    }

    /// Sets the state of a double solenoid based on [SolenoidController::set_solenoid_bitset].
    fn set_double_solenoid(
        &self,
        forward_channel: u32,
        backward_channel: u32,
        state: DoubleSolenoidState,
    ) {
        let value = match state {
            DoubleSolenoidState::Forward => 1 << forward_channel,
            DoubleSolenoidState::Backward => 1 << backward_channel,
            DoubleSolenoidState::Off => 0,
        };
        self.set_solenoid_bitset((1 << forward_channel) | (1 << backward_channel), value)
    }
}

/// A channel from a solenoid controller. Each channel can be actuated and released
/// independently by the controller. This trait allows each channel to be used
/// essentially as a reference to its controller, but with a type-level constant
/// for the channel number.
pub trait SolenoidChannel<'a> {
    /// The channel number for this channel
    const CHANNEL: u32;

    /// The type of the controller that controls this channel
    type Controller: SolenoidController + 'a;

    /// Gets a reference to this channel's controller
    fn get_controller(&self) -> &'a Self::Controller;

    /// Creates a new channel from a controller reference
    /// # Safety
    /// Must not be called while another channel with the same channel number and
    /// controller exists. This would cause two objects to control the same
    /// pneumatic channel, violating assumptions about aliasing.
    unsafe fn new(controller: &'a Self::Controller) -> Self;
}

/// A solenoid, which can be actuated in one direction or released.
/// Implemented by all solenoid types in this module. Generic code should prefer
/// to use either this trait or the [AnySolenoid] struct.
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
    /// Actuated in the forwards direction
    Forward,
    /// Actuated in the backwards direction
    Backward,
    /// Not actuated in any direction
    Off,
}

/// Represents the state of a double solenoid actuated on both ends.
/// This is invalid but possible, so this type is returned from [DoubleSolenoid::get]
/// in this case.
pub struct InvalidDoubleSolenoidState;

/// A double solenoid, which can be actuated in either direction or released.
/// Implemented by all double solenoid types in this module. Generic code
/// should prefer to use either this trait or the [AnyDoubleSolenoid] struct.
pub trait DoubleSolenoid {
    /// Gets the state of the double solenoid. If the solenoid is actuated on both ends, [InvalidDoubleSolenoidState] is returned.
    fn get(&self) -> Result<DoubleSolenoidState, InvalidDoubleSolenoidState>;

    /// Sets the state of the double solenoid.
    fn set(&mut self, state: DoubleSolenoidState);
}

/// A solenoid, with channels and controller type encoded in the type system.
/// This allows for a minimal memory footprint, as well as allowing the channel
/// to be released for reuse via [TypedSolenoid::into_channel]. This comes
/// at the cost of a complex generic type, and the inability to store several
/// solenoids homogeneously, e.g. in a slice or array.
/// To type-erase some of this information and store it at runtime, use
/// [TypedSolenoid::erase_channel] to type-erase the channel
/// or [TypedSolenoid::erase_all] to type-erase the channel and controller.
pub struct TypedSolenoid<'a, Channel: SolenoidChannel<'a>> {
    controller: &'a Channel::Controller,
}

impl<'a, Channel: SolenoidChannel<'a> + 'a> TypedSolenoid<'a, Channel> {
    /// Creates a [TypedSolenoid] from a channel.
    pub fn new(channel: Channel) -> Self {
        Self {
            controller: channel.get_controller(),
        }
    }

    /// Releases the channel used by this solenoid for reuse.
    pub fn into_channel(self) -> Channel {
        // SAFETY: this object has taken ownership of the channel for its lifetime.
        // No other object can safely alias this channel, so it is safe to re-materialize.
        unsafe { Channel::new(self.controller) }
    }

    /// Type-erases the channel used by this solenoid, storing it at runtime.
    pub fn erase_channel(self) -> ChannelErasedSolenoid<'a, Channel::Controller> {
        ChannelErasedSolenoid {
            controller: self.controller,
            channel: Channel::CHANNEL,
        }
    }

    /// Type-erases the channel and controller used by this solenoid, storing both at runtime.
    pub fn erase_all(self) -> AnySolenoid<'a> {
        AnySolenoid {
            controller: self.controller,
            channel: Channel::CHANNEL,
        }
    }
}

impl<'a, Channel: SolenoidChannel<'a> + 'a> Solenoid for TypedSolenoid<'a, Channel> {
    fn get(&self) -> bool {
        self.controller.get_solenoid(Channel::CHANNEL)
    }

    fn set(&mut self, state: bool) {
        self.controller.set_solenoid(Channel::CHANNEL, state)
    }
}

/// A solenoid with its channel stored at runtime, but controller type
/// still in the type system. Created by [TypedSolenoid::erase_channel].
/// Useful to store several solenoids from the same controller homogeneously, e.g.
/// in a slice or array. For a fully type-erased solenoid, see [AnySolenoid].
/// For a fully typed solenoid, see [TypedSolenoid].
pub struct ChannelErasedSolenoid<'a, Controller> {
    controller: &'a Controller,
    channel: u32,
}

impl<'a, Controller: SolenoidController> ChannelErasedSolenoid<'a, Controller> {
    /// Type-erases the controller used by this solenoid, storing it at runtime
    /// in addition to the channel already stored at runtime by this type.
    pub fn erase_all(self) -> AnySolenoid<'a> {
        AnySolenoid {
            controller: self.controller,
            channel: self.channel,
        }
    }
}

impl<'a, Controller: SolenoidController> Solenoid for ChannelErasedSolenoid<'a, Controller> {
    fn get(&self) -> bool {
        self.controller.get_solenoid(self.channel)
    }

    fn set(&mut self, state: bool) {
        self.controller.set_solenoid(self.channel, state)
    }
}

/// A solenoid with all type information (controller type and channels)
/// type-erased and stored at runtime. Created by [TypedSolenoid::erase_all]
/// or [ChannelErasedSolenoid::erase_all]. Useful to store any type of double
/// solenoid homogeneously, e.g. in a slice or array. For a solenoid with
/// channels type-erased but not controller type, see [ChannelErasedSolenoid].
/// For a fully-typed solenoid, see [TypedSolenoid].
pub struct AnySolenoid<'a> {
    controller: &'a dyn SolenoidController,
    channel: u32,
}

impl<'a> Solenoid for AnySolenoid<'a> {
    fn get(&self) -> bool {
        self.controller.get_solenoid(self.channel)
    }

    fn set(&mut self, state: bool) {
        self.controller.set_solenoid(self.channel, state)
    }
}

/// A double solenoid, with channels and controller type encoded in the type system.
/// This allows for a minimal memory footprint, as well as allowing channels to be
/// released for reuse via [TypedDoubleSolenoid::into_channels]. This comes
/// at the cost of a complex generic type, and the inability to store several
/// double solenoids homogeneously, e.g. in a slice or array.
/// To type-erase some of this information and store it at runtime, use
/// [TypedDoubleSolenoid::erase_channels] to type-erase the channels
/// or [TypedDoubleSolenoid::erase_all] to type-erase the channels and controller.
pub struct TypedDoubleSolenoid<'a, Controller, ForwardChannel, BackwardChannel> {
    controller: &'a Controller,
    _phantom: PhantomData<(ForwardChannel, BackwardChannel)>,
}

impl<'a, Controller, ForwardChannel, BackwardChannel>
    TypedDoubleSolenoid<'a, Controller, ForwardChannel, BackwardChannel>
where
    Controller: SolenoidController,
    ForwardChannel: SolenoidChannel<'a, Controller = Controller> + 'a,
    BackwardChannel: SolenoidChannel<'a, Controller = Controller> + 'a,
{
    /// Creates a new [TypedDoubleSolenoid] from two solenoid channels. These channels
    /// must have the same controller. The type system enforces that both have the
    /// same controller type, but it is possible to create two controllers of the
    /// same type and pass one channel from each into this function. This will
    /// cause a runtime panic.
    /// # Panics
    /// Panics if `forward_channel` and `backward_channel` do not have the same
    /// pneumatics controller. For example, the following code will panic:
    /// ```rust, no_run
    /// let pcm1 = CtrePcm::new(1);
    /// let (_, CtrePneumatics{ channel0, .. }) = pcm1.as_parts();
    /// let pcm2 = CtrePcm::new(2);
    /// let (_, CtrePneumatics{ channel1, .. }) = pcm2.as_parts();
    /// let will_panic = TypedDoubleSolenoid::new(channel0, channel1);
    /// ```
    /// If your code does not use more than one pneumatics controller, this function
    /// cannot panic.
    pub fn new(forward_channel: ForwardChannel, backward_channel: BackwardChannel) -> Self {
        // We enforce that both channels of a double solenoid have the same controller.
        // If the channels are controlled by different controllers, we can't set
        // the double solenoid state in one write, potentially causing timing issues.
        assert!(std::ptr::eq(
            forward_channel.get_controller(),
            backward_channel.get_controller()
        ));
        Self {
            controller: forward_channel.get_controller(),
            _phantom: PhantomData,
        }
    }

    /// Releases the channels used by this double solenoid for reuse.
    pub fn into_channels(self) -> (ForwardChannel, BackwardChannel) {
        // SAFETY: this object has taken ownership of both channels for its lifetime.
        // No other object can safely alias these channels, so they are safe to re-materialize.
        unsafe {
            (
                ForwardChannel::new(self.controller),
                BackwardChannel::new(self.controller),
            )
        }
    }

    /// Type-erases the channels used by this double solenoid, storing them at runtime.
    pub fn erase_channels(self) -> ChannelErasedDoubleSolenoid<'a, Controller> {
        ChannelErasedDoubleSolenoid {
            controller: self.controller,
            forward_channel: ForwardChannel::CHANNEL,
            backward_channel: BackwardChannel::CHANNEL,
        }
    }

    /// Type-erases the channels and controller used by this double solenoid, storing them at runtime.
    pub fn erase_all(self) -> AnyDoubleSolenoid<'a> {
        AnyDoubleSolenoid {
            controller: self.controller,
            forward_channel: ForwardChannel::CHANNEL,
            backward_channel: BackwardChannel::CHANNEL,
        }
    }
}

impl<'a, Controller, ForwardChannel, BackwardChannel> DoubleSolenoid
    for TypedDoubleSolenoid<'a, Controller, ForwardChannel, BackwardChannel>
where
    Controller: SolenoidController,
    ForwardChannel: SolenoidChannel<'a, Controller = Controller>,
    BackwardChannel: SolenoidChannel<'a, Controller = Controller>,
{
    fn get(&self) -> Result<DoubleSolenoidState, InvalidDoubleSolenoidState> {
        self.controller
            .get_double_solenoid(ForwardChannel::CHANNEL, BackwardChannel::CHANNEL)
    }

    fn set(&mut self, state: DoubleSolenoidState) {
        self.controller.set_double_solenoid(
            ForwardChannel::CHANNEL,
            BackwardChannel::CHANNEL,
            state,
        )
    }
}

/// A double solenoid with its channels stored at runtime, but controller type
/// still in the type system. Created by [TypedDoubleSolenoid::erase_channels].
/// Useful to store several solenoids from the same controller homogeneously, e.g.
/// in a slice or array. For a fully type-erased double solenoid, see [AnyDoubleSolenoid].
/// For a fully typed double solenoid, see [TypedDoubleSolenoid].
pub struct ChannelErasedDoubleSolenoid<'a, Controller> {
    controller: &'a Controller,
    forward_channel: u32,
    backward_channel: u32,
}

impl<'a, Controller: SolenoidController> ChannelErasedDoubleSolenoid<'a, Controller> {
    /// Type-erases the controller used by this double solenoid, storing it at runtime
    /// in addition to the channels already stored at runtime by this type.
    pub fn erase_all(self) -> AnyDoubleSolenoid<'a> {
        AnyDoubleSolenoid {
            controller: self.controller,
            forward_channel: self.forward_channel,
            backward_channel: self.backward_channel,
        }
    }
}

impl<'a, Controller: SolenoidController> DoubleSolenoid
    for ChannelErasedDoubleSolenoid<'a, Controller>
{
    fn get(&self) -> Result<DoubleSolenoidState, InvalidDoubleSolenoidState> {
        self.controller
            .get_double_solenoid(self.forward_channel, self.backward_channel)
    }

    fn set(&mut self, state: DoubleSolenoidState) {
        self.controller
            .set_double_solenoid(self.forward_channel, self.backward_channel, state)
    }
}

/// A double solenoid with all type information (controller type and channels)
/// type-erased and stored at runtime. Created by [TypedDoubleSolenoid::erase_all]
/// or [ChannelErasedDoubleSolenoid::erase_all]. Useful to store any type of double
/// solenoid homogeneously, e.g. in a slice or array. For a double solenoid with
/// channels type-erased but not controller type, see [ChannelErasedDoubleSolenoid].
/// For a fully-typed double solenoid, see [TypedDoubleSolenoid].
pub struct AnyDoubleSolenoid<'a> {
    controller: &'a dyn SolenoidController,
    forward_channel: u32,
    backward_channel: u32,
}

impl<'a> DoubleSolenoid for AnyDoubleSolenoid<'a> {
    fn get(&self) -> Result<DoubleSolenoidState, InvalidDoubleSolenoidState> {
        self.controller
            .get_double_solenoid(self.forward_channel, self.backward_channel)
    }

    fn set(&mut self, state: DoubleSolenoidState) {
        self.controller
            .set_double_solenoid(self.forward_channel, self.backward_channel, state)
    }
}

/// Common functionality between all pneumatic compressors.
/// Unfinished, does not currently contain all common functionality.
/// Some compressors may expose specific functionality not included in this trait.
pub trait Compressor {
    fn get_enabled(&self) -> bool;
    fn get_pressure_switch(&self) -> bool;
}
