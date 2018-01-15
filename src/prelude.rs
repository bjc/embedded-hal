//! The prelude is a collection of all the traits in this crate
//!
//! The traits have been renamed to avoid collisions with other items when
//! performing a glob import.

#[cfg(feature = "unproven")]
pub use ::Capture as _embedded_hal_Capture;
#[cfg(feature = "unproven")]
pub use ::Pwm as _embedded_hal_Pwm;
pub use ::PwmPin as _embedded_hal_PwmPin;
#[cfg(feature = "unproven")]
pub use ::Qei as _embedded_hal_Qei;
pub use ::Timer as _embedded_hal_Timer;
pub use ::blocking::delay::DelayMs as _embedded_hal_blocking_delay_DelayMs;
pub use ::blocking::delay::DelayUs as _embedded_hal_blocking_delay_DelayUs;
pub use ::digital::OutputPin as _embedded_hal_digital_OutputPin;
pub use ::serial::Read as _embedded_hal_serial_Read;
pub use ::serial::Write as _embedded_hal_serial_Write;
pub use ::spi::FullDuplex as _embedded_hal_spi_FullDuplex;
