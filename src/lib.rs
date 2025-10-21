#![no_std]
#![feature(alloc_error_handler)]

extern crate aos_uefi as uefi;

pub mod alloc;
pub mod fs;
#[macro_use]
pub mod stdio;
pub mod system;

pub use uefi::*;
