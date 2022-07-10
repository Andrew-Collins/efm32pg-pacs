//! Peripheral access API for EFM32PG22 microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.24.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.24.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [efm32-rs](https://github.com/efm32-rs/efm32pg-pacs)
//!
//! This crate supports all EFM32PG22 devices; for the complete list please see:
//! [efm32pg22](https://github.com/efm32-rs/efm32pg-pacs/pacs/efm32pg22)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "efm32pg22c200")]
pub mod efm32pg22c200;
