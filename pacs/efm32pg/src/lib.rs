//! Peripheral access API for EFM32PG microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.28.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.28.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [efm32-rs](https://github.com/efm32-rs/efm32pg-pacs)
//!
//! This crate supports all EFM32PG devices; for the complete list please see:
//! [efm32pg](https://github.com/efm32-rs/efm32pg-pacs/pacs/efm32pg)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "efm32pg12b500")]
pub mod efm32pg12b500;

#[cfg(feature = "efm32pg1b100")]
pub mod efm32pg1b100;

#[cfg(feature = "efm32pg1b200")]
pub mod efm32pg1b200;
