//! # Pneumatics Control
//!
//! Support for CTRE's Pneumatics Control Module is contained in the [ctre] module, via the [CtrePcm](ctre::CtrePcm) type.
//! Support for REV's Pneumatics Hub is contained in the [rev] module, via the [RevPh](rev::RevPh) type.
//!
//! This module also provides controller-independent [Solenoid] and [DoubleSolenoid] types.

pub mod ctre;
pub mod rev;

pub trait PneumaticsController {
    fn set_solenoids(&self, mask: u32, values: u32);
    fn get_solenoids(&self) -> u32;
}

pub struct Solenoid<Channel> {
    channel: Channel,
}

impl<Channel> Solenoid<Channel>
where
    Channel: MaskBasedChannel,
{
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }

    pub fn set(&mut self, state: bool) {
        self.channel
            .get_controller()
            .set_solenoids(Channel::MASK, if state { u32::MAX } else { 0 });
    }

    pub fn get(&self) -> bool {
        self.channel.get_controller().get_solenoids() & Channel::MASK != 0
    }

    pub fn into_channel(self) -> Channel {
        self.channel
    }
}

pub struct DoubleSolenoid<ForwardChannel, BackwardChannel> {
    forward_channel: ForwardChannel,
    backward_channel: BackwardChannel,
}

pub enum DoubleSolenoidState {
    Forward,
    Backward,
    Off,
}

pub struct InvalidDoubleSolenoidState;

impl<ForwardChannel, BackwardChannel> DoubleSolenoid<ForwardChannel, BackwardChannel>
where
    ForwardChannel: MaskBasedChannel,
    BackwardChannel: MaskBasedChannel,
{
    const FORWARD_MASK: u32 = ForwardChannel::MASK;
    const BACKWARD_MASK: u32 = BackwardChannel::MASK;
    const TOTAL_MASK: u32 = Self::FORWARD_MASK | Self::BACKWARD_MASK;

    pub fn new(forward_channel: ForwardChannel, backward_channel: BackwardChannel) -> Self {
        Self {
            forward_channel,
            backward_channel,
        }
    }

    pub fn set(&mut self, state: DoubleSolenoidState) {
        let value = match state {
            DoubleSolenoidState::Forward => Self::FORWARD_MASK,
            DoubleSolenoidState::Backward => Self::BACKWARD_MASK,
            DoubleSolenoidState::Off => 0,
        };
        self.forward_channel
            .get_controller()
            .set_solenoids(Self::TOTAL_MASK, value)
    }

    pub fn get(&self) -> Result<DoubleSolenoidState, InvalidDoubleSolenoidState> {
        let masked = self.forward_channel.get_controller().get_solenoids() & Self::TOTAL_MASK;
        match masked {
            _ if masked == Self::FORWARD_MASK => Ok(DoubleSolenoidState::Forward),
            _ if masked == Self::BACKWARD_MASK => Ok(DoubleSolenoidState::Backward),
            0 => Ok(DoubleSolenoidState::Off),
            _ => Err(InvalidDoubleSolenoidState),
        }
    }

    pub fn into_channels(self) -> (ForwardChannel, BackwardChannel) {
        (self.forward_channel, self.backward_channel)
    }
}

pub trait MaskBasedChannel {
    const MASK: u32;
    type Controller: PneumaticsController;

    fn get_controller(&self) -> &Self::Controller;
}

pub trait Compressor {
    fn get_enabled(&self) -> bool;
    fn get_pressure_switch(&self) -> bool;
}
