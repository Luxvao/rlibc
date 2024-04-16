//! Implementation of string.h in Rust

use core::{cmp::Ordering, ffi::{c_char, c_int, c_size_t, c_void}};

/// A function that returns a ptr to the first character that matches `c`
///
/// # Safety
/// This function is unsafe and prone to bugs. Reason: raw pointers
#[no_mangle]
pub unsafe extern "C" fn memchr(str: *const c_void, c: c_int, n: c_size_t) -> *const c_void {
    for i in 0..n {
        let current_char = str.byte_add(i);

        if *(current_char as *const c_char) == c as i8 {
            return current_char;
        }
    }

    core::ptr::null()
}

/// A function that compares n-bytes to see if it's greater, less or Equal
///
/// # Safety
/// This function is unsafe and prone to bugs. Reason: raw pointers
#[no_mangle]
pub unsafe extern "C" fn memcmp(str1: *const c_void, str2: *const c_void, n: c_size_t) -> c_int {
    for i in 0..n {
        let char1 = *(str1.byte_add(i) as *const c_char) as u8;
        let char2 = *(str2.byte_add(i) as *const c_char) as u8;

        match char1.cmp(&char2) {
            Ordering::Less => return -1,
            Ordering::Equal => (),
            Ordering::Greater => return 1,
        }
    }

    0
}

/// This function copies content from source buffer to destination buffer
///
/// # Safety
/// This function is unsafe and prone to bugs. Reason: raw pointers
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: c_size_t) -> *const c_void {
    for i in 0..n {
        *(dest.byte_add(i) as *mut c_char) = *(src.byte_add(i) as *const c_char);
    }

    dest
}

/// This function is like `memcpy`, but it's meant to be used with overlapping memory areas
///
/// # Safety
/// This function is unsafe and prone to bugs. Reason: raw pointers
#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut c_void, src: *const c_void, n: c_size_t) -> *const c_void {
    match src.addr().cmp(&dest.addr()) {
        Ordering::Equal => (),
        Ordering::Less => return memcpy(dest, src, n),
        Ordering::Greater => {
            for i in (0..n).rev() {
                *(dest.byte_add(i) as *mut c_char) = *(src.byte_add(i) as *const c_char);
            }
        }
    }

    dest
}

/// This function sets all bytes in a buffer to `c`
///
/// # Safety
/// This function is unsafe and prone to bugs. Reason: raw pointers
#[no_mangle]
pub unsafe extern "C" fn memset(str: *mut c_void, c: c_int, n: c_size_t) -> *const c_void {
    for i in 0..n {
        *(str.byte_add(i) as *mut c_char) = c as c_char;
    }

    str
}

/// This function calculates the lenght of a string
///
/// # Safety
/// This function is unsafe and prone to bugs. Reason: raw pointers
#[no_mangle]
pub unsafe extern "C" fn strlen(str: *const c_char) -> c_size_t {
    let mut size = 0;

    while *(str.byte_add(size)) != '\0' as i8 {
        size += 1;
    }

    size
}

#[no_mangle]
pub unsafe extern "C" fn strcat(dest: *mut c_char, src: *const c_char) -> *const c_char {
    let start = strlen(dest);
    let src_len = strlen(src) + 1;

    for i in 0..src_len {
        *dest.add(i + start) = *src.add(i);   
    }

    dest
}

