use std::mem::MaybeUninit;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;

use tokio::sync::Notify;

#[derive(PartialEq, Eq, Clone, Copy, Default)]
pub enum RobotMode {
    #[default]
    Disabled,
    Autonomous,
    Teleop,
    EStop,
}

#[derive(Debug, PartialEq)]
pub struct ControllerAxes {
    count: i16,
    axes: [f32; 12],
}

impl ControllerAxes {
    fn get(&self, n: u8) -> Option<f32> {
        if i16::from(n) < self.count {
            Some(self.axes[usize::from(n)])
        } else {
            None
        }
    }
}

impl From<wpihal_ffi::HAL_JoystickAxes> for ControllerAxes {
    fn from(value: wpihal_ffi::HAL_JoystickAxes) -> Self {
        Self {
            count: value.count,
            axes: value.axes,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ControllerPOVs {
    count: i16,
    povs: [i16; 12],
}

impl ControllerPOVs {
    fn get(&self, n: u8) -> Option<i16> {
        if i16::from(n) < self.count {
            Some(self.povs[usize::from(n)])
        } else {
            None
        }
    }
}

impl From<wpihal_ffi::HAL_JoystickPOVs> for ControllerPOVs {
    fn from(value: wpihal_ffi::HAL_JoystickPOVs) -> Self {
        Self {
            count: value.count,
            povs: value.povs,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ControllerButtons {
    count: u8,
    buttons: u32,
}

impl ControllerButtons {
    fn get(&self, n: u8) -> Option<bool> {
        if n < self.count {
            Some(self.buttons & (1 << n) != 0)
        } else {
            None
        }
    }
}

impl From<wpihal_ffi::HAL_JoystickButtons> for ControllerButtons {
    fn from(value: wpihal_ffi::HAL_JoystickButtons) -> Self {
        Self {
            count: value.count,
            buttons: value.buttons,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ControllerState {
    axes: ControllerAxes,
    povs: ControllerPOVs,
    buttons: ControllerButtons,
}

impl ControllerState {
    /// Gets the value of a controller axis, or None if no such axis exists.
    pub fn axis(&self, n: u8) -> Option<f32> {
        self.axes.get(n)
    }

    /// Gets the value of a controller POV (D-pad), or None if no such POV exists.
    pub fn pov(&self, n: u8) -> Option<i16> {
        self.povs.get(n)
    }

    /// Gets the value of a controller button, or None if no such button exists.
    pub fn button(&self, n: u8) -> Option<bool> {
        self.buttons.get(n)
    }
}

pub struct DriverStation {
    new_packet: Arc<Notify>,
    shutdown: Arc<AtomicBool>,
}

impl DriverStation {
    pub fn new() -> Self {
        let shutdown = Arc::new(AtomicBool::new(false));
        let shutdown2 = Arc::clone(&shutdown);
        let new_packet = Arc::new(Notify::new());
        let new_packet2 = Arc::clone(&new_packet);
        std::thread::spawn(move || {
            let shutdown = shutdown2;
            let new_packet = new_packet2;
            let event_handle = DsEvent::new(false, false);
            while !shutdown.load(Ordering::Relaxed) {
                if event_handle.wait_timeout(Duration::from_secs(1)).is_ok() {
                    new_packet.notify_waiters();
                }
            }
        });
        Self {
            new_packet,
            shutdown,
        }
    }

    /// Waits until a new packet is received from the Driver Station.
    /// This signifies that updated Driver Station data, such as robot and controller state, is available.
    pub async fn wait_for_packet(&self) {
        self.new_packet.notified().await
    }

    /// Gets the most recently reported state of the controller on a given port, if any is plugged in.
    /// Returns None if no controller exists on the given port.
    pub fn get_controller_state(&self, port: u8) -> Option<ControllerState> {
        let axes = unsafe {
            let mut axes = MaybeUninit::uninit();
            let result = wpihal_ffi::HAL_GetJoystickAxes(i32::from(port), axes.as_mut_ptr());
            if result != 0 {
                return None;
            }
            axes.assume_init()
        };
        let povs = unsafe {
            let mut povs = MaybeUninit::uninit();
            let result = wpihal_ffi::HAL_GetJoystickPOVs(i32::from(port), povs.as_mut_ptr());
            if result != 0 {
                return None;
            }
            povs.assume_init()
        };
        let buttons = unsafe {
            let mut buttons = MaybeUninit::uninit();
            let result = wpihal_ffi::HAL_GetJoystickButtons(i32::from(port), buttons.as_mut_ptr());
            if result != 0 {
                return None;
            }
            buttons.assume_init()
        };
        Some(ControllerState {
            axes: ControllerAxes::from(axes),
            povs: ControllerPOVs::from(povs),
            buttons: ControllerButtons::from(buttons),
        })
    }

    /// Gets the most recently reported robot mode.
    pub fn get_robot_mode() -> RobotMode {
        // SAFETY: safe because HAL_GetControlWord is guaranteed to initialize control_word
        let control_word = unsafe {
            let mut control_word = MaybeUninit::uninit();
            wpihal_ffi::HAL_GetControlWord(control_word.as_mut_ptr());
            control_word.assume_init()
        };
        if control_word.enabled() != 0 {
            if control_word.autonomous() != 0 {
                RobotMode::Autonomous
            } else {
                RobotMode::Teleop
            }
        } else if control_word.eStop() != 0 {
            RobotMode::EStop
        } else {
            RobotMode::Disabled
        }
    }
}

impl Default for DriverStation {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DriverStation {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
    }
}

struct DsEvent {
    handle: u32,
}

impl DsEvent {
    fn new(manual_reset: bool, initial_state: bool) -> Self {
        let handle = unsafe {
            wpiutil_ffi::WPI_CreateEvent(
                if manual_reset { 1 } else { 0 },
                if initial_state { 1 } else { 0 },
            )
        };
        unsafe {
            wpihal_ffi::HAL_ProvideNewDataEventHandle(handle);
        }
        Self { handle }
    }

    fn wait_timeout(&self, timeout: Duration) -> Result<(), ()> {
        unsafe {
            let mut timed_out = MaybeUninit::uninit();
            wpiutil_ffi::WPI_WaitForObjectTimeout(
                self.handle,
                timeout.as_secs_f64(),
                timed_out.as_mut_ptr(),
            );
            if timed_out.assume_init() == 0 {
                Ok(())
            } else {
                Err(())
            }
        }
    }
}

impl Drop for DsEvent {
    fn drop(&mut self) {
        unsafe { wpiutil_ffi::WPI_DestroyEvent(self.handle) }
    }
}
