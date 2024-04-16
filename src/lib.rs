//! # rlibc
//! This is a simple implementation of libc in Rust! It provides all the functions defined in the
//! libc API (the header files). It's meant for linking against C programs.

#![no_std]
#![feature(c_size_t)]
#![feature(strict_provenance)]

pub mod ctypes;
pub mod locale;
pub mod math;
pub mod setjmp;
pub mod signal;
pub mod stdio;
pub mod stdlib;
pub mod string;
pub mod time;

#[allow(unused_imports)]
use core::panic::PanicInfo;

#[cfg(test)]
mod tests {
    use core::ffi::c_char;
    use core::ffi::c_void;
    use core::ffi::c_int;

    use crate::ctypes::isalnum;
    use crate::string::memchr;
    use crate::string::memcmp;
    use crate::string::memcpy;
    use crate::string::memmove;
    use crate::string::memset;
    use crate::string::strcat;
    use crate::string::strlen;

    #[test]
    fn ctypes() {
        let result = isalnum('A' as c_int);

        assert_eq!(result, 1);
    }

    #[test]
    fn string() {
        unsafe {
            // memcmp tests
            assert_eq!(memcmp("A".as_ptr() as *const c_void, "a".as_ptr() as *const c_void, 1), -1);
            assert_eq!(memcmp("a".as_ptr() as *const c_void, "A".as_ptr() as *const c_void, 1), 1);
            assert_eq!(memcmp("A".as_ptr() as *const c_void, "A".as_ptr() as *const c_void, 1), 0);

            // memchr test
            assert_eq!(*(memchr("rlibc".as_ptr() as *const c_void, 'l' as c_int, 5) as *const u8), b'l');

            // memcpy test
            let src = "rlibc";
            let dest: [u8; 5] = [0; 5];

            memcpy(dest.as_ptr() as *mut c_void, src.as_ptr() as *const c_void, 5);

            assert_eq!(dest, [b'r', b'l', b'i', b'b', b'c']);

            // memmove test
            let src = "rlibc";
            let dest: [u8; 5] = [0; 5];

            memmove(dest.as_ptr() as *mut c_void, src.as_ptr() as *const c_void, 5);

            assert_eq!(dest, [b'r', b'l', b'i', b'b', b'c']);
            
            // memset test
            let buffer: [u8; 5] = [0, 0, 0, 0, 0];

            memset(buffer.as_ptr() as *mut c_void, 5, 5);

            assert_eq!(buffer, [5; 5]);

            // strlen test
            
            assert_eq!(strlen(str.as_ptr() as *const c_char), 5);
            
            // strcat test
            
        }
    }
}

#[panic_handler]
#[cfg(any(rust_analyzer, not(test)))]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

