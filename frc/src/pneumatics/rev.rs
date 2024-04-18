use super::{
    AnyDoubleSolenoid, AnySolenoid, Compressor, DoubleSolenoid, DoubleSolenoidState,
    GenericDoubleSolenoid, GenericSolenoid, InvalidDoubleSolenoidState, Solenoid,
};

pub struct RevPh {
    handle: wpihal_sys::HAL_REVPHHandle,
}

impl RevPh {
    pub fn new(can_id: i32) -> Self {
        let handle = unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_InitializeREVPH(
                    can_id,
                    c"".as_ptr(),
                    status,
                )
            })
        };
        Self { handle }
    }

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

    fn get_solenoids(&self) -> u32 {
        let solenoids = unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_GetREVPHSolenoids(self.handle, status)
            })
        };
        solenoids as u32
    }

    fn set_solenoids(&self, mask: u32, values: u32) {
        unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
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

pub struct RevCompressor<'a> {
    ph: &'a RevPh,
}

impl<'a> Compressor for RevCompressor<'a> {
    fn get_enabled(&self) -> bool {
        unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_GetREVPHCompressor(self.ph.handle, status)
            }) != 0
        }
    }

    fn get_pressure_switch(&self) -> bool {
        unsafe {
            wpihal_sys::panic_on_hal_error(|status| {
                wpihal_sys::HAL_GetREVPHPressureSwitch(self.ph.handle, status)
            }) != 0
        }
    }
}

pub trait RevChannel {
    const MASK: u32;
    fn get_controller(&self) -> &RevPh;
}

macro_rules! rev_channel {
    ($name:ident, $num:expr) => {
        pub struct $name<'a> {
            ph: &'a RevPh,
        }

        impl<'a> RevChannel for $name<'a> {
            const MASK: u32 = 1 << $num;

            fn get_controller(&self) -> &RevPh {
                self.ph
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

pub struct RevSolenoid<C: RevChannel> {
    channel: C,
}

impl<Channel: RevChannel> RevSolenoid<Channel> {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }

    pub fn into_channel(self) -> Channel {
        self.channel
    }

    pub fn erase_channel(&mut self) -> AnyRevSolenoid<'_> {
        AnyRevSolenoid {
            ph: self.channel.get_controller(),
            mask: Channel::MASK,
        }
    }

    pub fn erase_all(&mut self) -> AnySolenoid<'_> {
        AnySolenoid {
            solenoid: GenericSolenoid::Rev(self.erase_channel()),
        }
    }
}

impl<Channel: RevChannel> Solenoid for RevSolenoid<Channel> {
    fn get(&self) -> bool {
        self.channel.get_controller().get_solenoids() & Channel::MASK != 0
    }

    fn set(&mut self, state: bool) {
        self.channel
            .get_controller()
            .set_solenoids(Channel::MASK, if state { u32::MAX } else { 0 });
    }
}

pub struct RevDoubleSolenoid<ForwardChannel: RevChannel, BackwardChannel: RevChannel> {
    forward_channel: ForwardChannel,
    backward_channel: BackwardChannel,
}

impl<ForwardChannel: RevChannel, BackwardChannel: RevChannel>
    RevDoubleSolenoid<ForwardChannel, BackwardChannel>
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

    pub fn erase_channel(&mut self) -> AnyRevDoubleSolenoid<'_> {
        AnyRevDoubleSolenoid {
            ph: self.forward_channel.get_controller(),
            forward_mask: ForwardChannel::MASK,
            backward_mask: BackwardChannel::MASK,
        }
    }

    pub fn erase_all(&mut self) -> AnyDoubleSolenoid<'_> {
        AnyDoubleSolenoid {
            solenoid: GenericDoubleSolenoid::Rev(self.erase_channel()),
        }
    }
}

impl<ForwardChannel: RevChannel, BackwardChannel: RevChannel> DoubleSolenoid
    for RevDoubleSolenoid<ForwardChannel, BackwardChannel>
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

pub struct AnyRevSolenoid<'a> {
    ph: &'a RevPh,
    mask: u32,
}

impl<'a> Solenoid for AnyRevSolenoid<'a> {
    fn get(&self) -> bool {
        self.ph.get_solenoids() & self.mask != 0
    }

    fn set(&mut self, state: bool) {
        self.ph
            .set_solenoids(self.mask, if state { u32::MAX } else { 0 });
    }
}

pub struct AnyRevDoubleSolenoid<'a> {
    ph: &'a RevPh,
    forward_mask: u32,
    backward_mask: u32,
}

impl<'a> DoubleSolenoid for AnyRevDoubleSolenoid<'a> {
    fn get(&self) -> Result<DoubleSolenoidState, InvalidDoubleSolenoidState> {
        let masked = self.ph.get_solenoids() & (self.forward_mask | self.backward_mask);
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
        self.ph
            .set_solenoids(self.forward_mask | self.backward_mask, value)
    }
}
