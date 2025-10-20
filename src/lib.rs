#![no_std]
#![feature(alloc_error_handler)]

pub mod alloc;
pub mod fs;
#[macro_use]
pub mod stdio;
pub mod system;

pub use aos_uefi::*;
