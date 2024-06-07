use thiserror::Error;
use tokio::sync::oneshot;

use crate::error::HalError;

#[derive(Error, Debug)]
pub enum InterruptError {
    #[error("all interrupts are in use")]
    NoAvailableInterrupt,
    #[error("invalid handle")]
    BadHandle,
    #[error(transparent)]
    Fpga(#[from] HalError),
}

impl InterruptError {
    pub(super) fn from_status(status: i32) -> Result<(), Self> {
        match status {
            wpihal_sys::HAL_SUCCESS => Ok(()),
            wpihal_sys::NO_AVAILABLE_RESOURCES => Err(InterruptError::NoAvailableInterrupt),
            wpihal_sys::HAL_HANDLE_ERROR => Err(InterruptError::BadHandle),
            a => Err(InterruptError::Fpga(HalError::new(a))),
        }
    }
}

pub(crate) enum EdgeType {
    Rising,
    Falling,
    Either,
}

pub(crate) async fn wait_for_edge(
    dio_handle: wpihal_sys::HAL_DigitalHandle,
    edge: EdgeType,
) -> Result<(), InterruptError> {
    let mut status = wpihal_sys::HAL_SUCCESS;
    let irq_handle =
        unsafe { wpihal_sys::HAL_InitializeInterrupts(std::ptr::from_mut(&mut status)) };
    InterruptError::from_status(status)?;
    unsafe {
        wpihal_sys::HAL_RequestInterrupts(
            irq_handle,
            dio_handle,
            wpihal_sys::HAL_AnalogTriggerType_HAL_Trigger_kInWindow,
            std::ptr::from_mut(&mut status),
        );
    };
    InterruptError::from_status(status)?;
    let (rising, falling) = match edge {
        EdgeType::Rising => (true, false),
        EdgeType::Falling => (false, true),
        EdgeType::Either => (true, true),
    };
    unsafe {
        wpihal_sys::HAL_SetInterruptUpSourceEdge(
            irq_handle,
            i32::from(rising),
            i32::from(falling),
            std::ptr::from_mut(&mut status),
        );
    }
    InterruptError::from_status(status)?;
    wait_for_blocking(|| loop {
        let asserted = unsafe {
            wpihal_sys::HAL_WaitForInterrupt(
                irq_handle,
                10.0,
                i32::from(false),
                std::ptr::from_mut(&mut status),
            )
        };
        if asserted != 0 {
            break Ok(());
        }
        InterruptError::from_status(status)?;
    })
    .await
}

async fn wait_for_blocking<R: Send>(f: impl FnOnce() -> R + Send) -> R {
    let (tx, rx) = oneshot::channel();
    std::thread::scope(|scope| {
        scope.spawn(move || {
            let out = f();
            tx.send(out).ok().unwrap();
        });
    });
    rx.await.unwrap()
}
