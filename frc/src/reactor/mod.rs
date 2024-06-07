//! Very unfinished reactor to wake async tasks upon external conditions
//! (e.g. an FPGA interrupt fired, we got a new packet from the driver station, etc.)

pub mod dio;
pub mod driver_station;

// NOTES
// =====
//
// HOW INTERRUPTS WORK
// the FPGA has 32 IRQs, which are allocated to various purposes
// incomplete list of IRQ mappings:
// 0..=7 - input pin interrupts, rising edges
// 8..=15 - input pin interrupts, falling edges
// 28 - timer alarm
// the first 16 IRQs are used for 8 channels of input pin interrupts
// each channel can be mapped to a DIO, AI or MXI pin (needs confirmation)
// each channel has one IRQ that triggers on rising edges and one for falling edges
// there is also an IRQ which functions as an alarm
// the alarm can be given a time, and when the FPGA time matches, the IRQ is triggered
// idk if any other IRQs are used, or what they're used for

// IrqContexts
// the fpga hands out IrqContext objects that must be used to register and listen for IRQs

// USING DIGITAL INPUT INTERRUPTS
// call HAL_InitializeInterrupts to get a HAL_InterruptHandle
// now you need to call HAL_RequestInterrupts to route a digital source (i.e. a pin) as the interrupt source
// call HAL_SetInterruptUpSourceEdge to set which signal edges to listen for (rising, falling)
// then call HAL_WaitForInterrupt to block the thread until the interrupt fires
// HAL_ReleaseWaitingInterrupt exists too - seems to force an interrupt to fire
// wpihal has a pool of 8 DIO interrupt handles, which it allocates
// when wpihal registers an interrupt, it allocates it an index from 0-7
// the IRQ corresponding to the index is used for the signal's rising edge
// the IRQ corresponding to the index + 8 is used for the signal's falling edge
// unsure, but maybe HAL_SetInterruptUpSourceEdge works by disabling one or both of these IRQs
