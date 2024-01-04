//! Very unfinished reactor to wake async tasks upon external conditions
//! (e.g. an FPGA interrupt fired, we got a new packet from the driver station, etc.)

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    task::Waker,
};

use self::waker_array::WakerArray;

pub mod driver_station;
mod waker_array;

// HOW INTERRUPTS WORK
// call HAL_InitializeInterrupts to get a HAL_InterruptHandle
// now you need to call HAL_RequestInterrupts to route a digital source (i.e. a pin) as the interrupt source
// then call HAL_WaitForInterrupt to block the thread until the interrupt fires
// HAL_SetInterruptUpSourceEdge exists too - maybe it sets which signal edges to listen for?
// HAL_ReleaseWaitingInterrupt exists too - maybe it cancels an existing HAL_WaitForInterrupt?

#[repr(usize)]
pub enum Irq {
    Irq0 = 0,
    Irq1 = 1,
    Irq2 = 2,
    Irq3 = 3,
    Irq4 = 4,
    Irq5 = 5,
    Irq6 = 6,
    Irq7 = 7,
}

pub struct IrqReactor {
    wakers: Arc<Mutex<WakerArray<8>>>,
    shutdown: Arc<AtomicBool>,
}

impl IrqReactor {
    pub fn new() -> Self {
        let shutdown = Arc::new(AtomicBool::new(false));
        let shutdown_inner = Arc::clone(&shutdown);
        let wakers: Arc<Mutex<WakerArray<8>>> = Default::default();
        let wakers_inner = Arc::clone(&wakers);

        std::thread::spawn(move || {
            let handle = unsafe {
                wpihal_ffi::panic_on_hal_error(|status| {
                    wpihal_ffi::HAL_InitializeInterrupts(status)
                })
            };
            while !shutdown_inner.load(Ordering::Relaxed) {
                let mut irq = unsafe {
                    wpihal_ffi::panic_on_hal_error(|status| {
                        wpihal_ffi::HAL_WaitForMultipleInterrupts(handle, i64::MAX, 0.5, 0, status)
                    })
                };
                let mut wakers_guard = wakers_inner.lock().unwrap();
                while irq != 0 {
                    let irq_number = irq.trailing_zeros();
                    wakers_guard.wake(irq_number as usize);
                    irq &= !(1 << irq_number);
                }
            }
        });

        Self { wakers, shutdown }
    }

    pub fn register(&self, irq_number: Irq, waker: Waker) -> Result<(), Waker> {
        self.wakers
            .lock()
            .unwrap()
            .register(waker, irq_number as usize)
    }

    pub fn replace(&self, irq_number: Irq, waker: Waker) -> Option<Waker> {
        self.wakers
            .lock()
            .unwrap()
            .replace(waker, irq_number as usize)
    }
}
impl Drop for IrqReactor {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
    }
}
