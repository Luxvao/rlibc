//! # rlibc
//! This is a simple implementation of libc in Rust! It provides all the functions defined in the
//! libc API (the header files). It's meant for linking against C programs.

#![no_std]
#![feature(c_size_t)]

pub mod stdlib;
pub mod stdio;
pub mod math;
pub mod signal;
pub mod string;
pub mod locale;
pub mod time;
pub mod setjmp;
pub mod ctypes;

#[allow(unused_imports)]
use core::panic::PanicInfo;

#[panic_handler]
#[cfg(any(rust_analyzer, not(test)))]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
