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
// the FPGA has 32 IRQs, which are allocated to various purposes
// the first 16 IRQs are used for 8 channels of digital input interrupts
// each channel can be mapped to a DIO, AI or MXI pin (needs confirmation)
// each channel has one IRQ that triggers on rising edges and one for falling edges
// there is also an IRQ which functions as an alarm
// the alarm can be given a time, and when the FPGA time matches, the IRQ is triggered
// idk if any other IRQs are used, or what they're used for

// IrqContexts
// the fpga hands out IrqContext objects

// USING DIGITAL INPUT INTERRUPTS
// call HAL_InitializeInterrupts to get a HAL_InterruptHandle
// now you need to call HAL_RequestInterrupts to route a digital source (i.e. a pin) as the interrupt source
// then call HAL_WaitForInterrupt to block the thread until the interrupt fires
// HAL_SetInterruptUpSourceEdge exists too - seems to set which signal edges to listen for
// HAL_ReleaseWaitingInterrupt exists too - seems to force an interrupt to fire
// when wpihal registers an interrupt, it allocates it an index from 0-7
// the IRQ corresponding to the index is used for the signal's rising edge
// the IRQ corresponding to the index + 8 is used for the signal's falling edge
// unsure, but maybe HAL_SetInterruptUpSourceEdge works by disabling one or both of these IRQs

#[repr(usize)]
pub enum Irq {
    Digital0Rising = 0,
    Digital1Rising = 1,
    Digital2Rising = 2,
    Digital3Rising = 3,
    Digital4Rising = 4,
    Digital5Rising = 5,
    Digital6Rising = 6,
    Digital7Rising = 7,
    Digital0Falling = 8,
    Digital1Falling = 9,
    Digital2Falling = 10,
    Digital3Falling = 11,
    Digital4Falling = 12,
    Digital5Falling = 13,
    Digital6Falling = 14,
    Digital7Falling = 15,
    Alarm = 28,
}

pub struct IrqReactor {
    wakers: Arc<Mutex<WakerArray<16>>>,
    shutdown: Arc<AtomicBool>,
}

impl IrqReactor {
    #[must_use]
    pub fn new() -> Self {
        let shutdown = Arc::new(AtomicBool::new(false));
        let shutdown_inner = Arc::clone(&shutdown);
        let wakers: Arc<Mutex<WakerArray<16>>> = Arc::default();
        let wakers_inner = Arc::clone(&wakers);

        std::thread::spawn(move || {
            let handle = unsafe {
                wpihal_sys::panic_on_hal_error(|status| {
                    wpihal_sys::HAL_InitializeInterrupts(status)
                })
            };
            while !shutdown_inner.load(Ordering::Relaxed) {
                let mut irq = unsafe {
                    wpihal_sys::panic_on_hal_error(|status| {
                        wpihal_sys::HAL_WaitForMultipleInterrupts(handle, i64::MAX, 0.5, 0, status)
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

impl Default for IrqReactor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for IrqReactor {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
    }
}
