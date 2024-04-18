use super::{
    AnyDoubleSolenoid, AnySolenoid, Compressor, DoubleSolenoid, DoubleSolenoidState,
    GenericDoubleSolenoid, GenericSolenoid, InvalidDoubleSolenoidState, Solenoid,
};

pub struct CtrePcm {
    handle: wpihal_sys::HAL_CTREPCMHandle,
}

impl CtrePcm {
    pub fn new(can_id: i32) -> Self {
        let handle = unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_InitializeCTREPCM(
                    can_id,
                    c"".as_ptr(),
                    status,
                )
            })
        };
        Self { handle }
    }

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

    fn get_solenoids(&self) -> u32 {
        let solenoids = unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_GetCTREPCMSolenoids(self.handle, status)
            })
        };
        solenoids as u32
    }

    fn set_solenoids(&self, mask: u32, values: u32) {
        unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
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
    fn get_enabled(&self) -> bool {
        unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_GetCTREPCMCompressor(self.pcm.handle, status)
            }) != 0
        }
    }

    fn get_pressure_switch(&self) -> bool {
        unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_GetCTREPCMPressureSwitch(self.pcm.handle, status)
            }) != 0
        }
    }
}

pub trait CtreChannel {
    const MASK: u32;
    fn get_controller(&self) -> &CtrePcm;
}

macro_rules! ctre_channel {
    ($name:ident, $num:expr) => {
        pub struct $name<'a> {
            pcm: &'a CtrePcm,
        }

        impl<'a> CtreChannel for $name<'a> {
            const MASK: u32 = 1 << $num;

            fn get_controller(&self) -> &CtrePcm {
                self.pcm
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

pub struct CtreSolenoid<Channel: CtreChannel> {
    channel: Channel,
}

impl<Channel: CtreChannel> CtreSolenoid<Channel> {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }

    pub fn into_channel(self) -> Channel {
        self.channel
    }

    pub fn erase_channel(&mut self) -> AnyCtreSolenoid<'_> {
        AnyCtreSolenoid {
            pcm: self.channel.get_controller(),
            mask: Channel::MASK,
        }
    }

    pub fn erase_all(&mut self) -> AnySolenoid<'_> {
        AnySolenoid {
            solenoid: GenericSolenoid::Ctre(self.erase_channel()),
        }
    }
}

impl<Channel: CtreChannel> Solenoid for CtreSolenoid<Channel> {
    fn get(&self) -> bool {
        self.channel.get_controller().get_solenoids() & Channel::MASK != 0
    }

    fn set(&mut self, state: bool) {
        self.channel
            .get_controller()
            .set_solenoids(Channel::MASK, if state { u32::MAX } else { 0 });
    }
}

pub struct CtreDoubleSolenoid<ForwardChannel: CtreChannel, BackwardChannel: CtreChannel> {
    forward_channel: ForwardChannel,
    backward_channel: BackwardChannel,
}

impl<ForwardChannel: CtreChannel, BackwardChannel: CtreChannel>
    CtreDoubleSolenoid<ForwardChannel, BackwardChannel>
{
    const FORWARD_MASK: u32 = ForwardChannel::MASK;
    const BACKWARD_MASK: u32 = BackwardChannel::MASK;
    const BOTH_MASK: u32 = ForwardChannel::MASK | BackwardChannel::MASK;

    pub fn new(forward_channel: ForwardChannel, backward_channel: BackwardChannel) -> Self {
        assert!(std::ptr::eq(
            forward_channel.get_controller(),
            backward_channel.get_controller()
        ));
        Self {
            forward_channel,
            backward_channel,
        }
    }

    pub fn into_channels(self) -> (ForwardChannel, BackwardChannel) {
        (self.forward_channel, self.backward_channel)
    }

    pub fn erase_channel(&mut self) -> AnyCtreDoubleSolenoid<'_> {
        AnyCtreDoubleSolenoid {
            pcm: self.forward_channel.get_controller(),
            forward_mask: ForwardChannel::MASK,
            backward_mask: BackwardChannel::MASK,
        }
    }

    pub fn erase_all(&mut self) -> AnyDoubleSolenoid<'_> {
        AnyDoubleSolenoid {
            solenoid: GenericDoubleSolenoid::Ctre(self.erase_channel()),
        }
    }
}

impl<ForwardChannel: CtreChannel, BackwardChannel: CtreChannel> DoubleSolenoid
    for CtreDoubleSolenoid<ForwardChannel, BackwardChannel>
{
    fn get(&self) -> Result<DoubleSolenoidState, InvalidDoubleSolenoidState> {
        let masked = self.forward_channel.get_controller().get_solenoids() & Self::BOTH_MASK;
        match masked {
            _ if masked == Self::FORWARD_MASK => Ok(DoubleSolenoidState::Forward),
            _ if masked == Self::BACKWARD_MASK => Ok(DoubleSolenoidState::Backward),
            0 => Ok(DoubleSolenoidState::Off),
            _ => Err(InvalidDoubleSolenoidState),
        }
    }

    fn set(&mut self, state: DoubleSolenoidState) {
        let value = match state {
            DoubleSolenoidState::Forward => Self::FORWARD_MASK,
            DoubleSolenoidState::Backward => Self::BACKWARD_MASK,
            DoubleSolenoidState::Off => 0,
        };
        self.forward_channel
            .get_controller()
            .set_solenoids(Self::BOTH_MASK, value)
    }
}

pub struct AnyCtreSolenoid<'a> {
    pcm: &'a CtrePcm,
    mask: u32,
}

impl<'a> Solenoid for AnyCtreSolenoid<'a> {
    fn get(&self) -> bool {
        self.pcm.get_solenoids() & self.mask != 0
    }

    fn set(&mut self, state: bool) {
        self.pcm
            .set_solenoids(self.mask, if state { u32::MAX } else { 0 });
    }
}

pub struct AnyCtreDoubleSolenoid<'a> {
    pcm: &'a CtrePcm,
    forward_mask: u32,
    backward_mask: u32,
}

impl<'a> DoubleSolenoid for AnyCtreDoubleSolenoid<'a> {
    fn get(&self) -> Result<DoubleSolenoidState, InvalidDoubleSolenoidState> {
        let masked = self.pcm.get_solenoids() & (self.forward_mask | self.backward_mask);
        match masked {
            _ if masked == self.forward_mask => Ok(DoubleSolenoidState::Forward),
            _ if masked == self.backward_mask => Ok(DoubleSolenoidState::Backward),
            0 => Ok(DoubleSolenoidState::Off),
            _ => Err(InvalidDoubleSolenoidState),
        }
    }

    fn set(&mut self, state: DoubleSolenoidState) {
        let value = match state {
            DoubleSolenoidState::Forward => self.forward_mask,
            DoubleSolenoidState::Backward => self.backward_mask,
            DoubleSolenoidState::Off => 0,
        };
        self.pcm
            .set_solenoids(self.forward_mask | self.backward_mask, value)
    }
}
