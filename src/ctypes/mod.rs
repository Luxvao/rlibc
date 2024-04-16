//! Implementation of ctypes.h
//! It provides all necessary functions for it to be complete.

use core::ffi::c_int;

/// Checks if character is alphanumeric
#[no_mangle]
pub extern "C" fn isalnum(c: c_int) -> c_int {
    (isalpha(c) == 1 || isdigit(c) == 1) as c_int
}

/// Checks if character is alphabetic
#[no_mangle]
pub extern "C" fn isalpha(c: c_int) -> c_int {
    (isupper(c) == 1 || islower(c) == 1) as c_int
}

/// Checks if character is a control character according to ASCII
#[no_mangle]
pub extern "C" fn iscntrl(c: c_int) -> c_int {
    if (0x0..=0x1f).any(|code| code == c as u8) | (c as u8 == 0x7f) {
        return 1;
    }

    0
}

/// Checks if character is a digit
#[no_mangle]
pub extern "C" fn isdigit(c: c_int) -> c_int {
    if (b'0'..=b'9').any(|code| code == c as u8) {
        return 1;
    }

    0
}

/// Checks if character is printable excluding whitespace
#[no_mangle]
pub extern "C" fn isgraph(c: c_int) -> c_int {
    (isalnum(c) == 1 || ispunct(c) == 1) as c_int
}

/// Checks if character is lowercase
#[no_mangle]
pub extern "C" fn islower(c: c_int) -> c_int {
    if (b'a'..=b'z').any(|code| code == c as u8) {
        return 1;
    }

    0
}

/// Checks if character is printable
#[no_mangle]
pub extern "C" fn isprint(c: c_int) -> c_int {
    (isalnum(c) == 1 || ispunct(c) == 1 || isspace(c) == 1) as c_int
}

/// Checks if character is a punctuation character
#[no_mangle]
pub extern "C" fn ispunct(c: c_int) -> c_int {
    const PUNCTUATION: [u8; 32] = [
        b'!', b'"', b'#', b'$', b'%', b'&', b'\'', b'(', b')', b'*', b'+', b',', b'-', b'.', b'/',
        b':', b';', b'<', b'=', b'>', b'?', b'@', b'[', b'\\', b']', b'^', b'_', b'`', b'{', b'|',
        b'}', b'~',
    ];

    if PUNCTUATION.iter().any(|code| *code == c as u8) {
        return 1;
    }

    0
}

/// Checks if character is whitespace
#[no_mangle]
pub extern "C" fn isspace(c: c_int) -> c_int {
    const WHITESPACE: [u8; 6] = [b' ', b'\t', b'\n', b'\r', 0x0b, 0x0c];

    if WHITESPACE.iter().any(|code| *code == c as u8) {
        return 1;
    }

    0
}

/// Checks if character is uppercase
#[no_mangle]
pub extern "C" fn isupper(c: c_int) -> c_int {
    if (b'A'..=b'Z').any(|code| code == c as u8) {
        return 1;
    }

    0
}

/// Checks if character is a hexadecimal digit
#[no_mangle]
pub extern "C" fn isxdigit(c: c_int) -> c_int {
    const HEX_SPECIFIC: [u8; 12] = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'a', b'b', b'c', b'd', b'e', b'f',
    ];

    if isdigit(c) == 1 || HEX_SPECIFIC.iter().any(|code| *code == c as u8) {
        return 1;
    }

    0
}

/// Returns uppercase equivalent of character if it exists
#[no_mangle]
pub extern "C" fn tolower(c: c_int) -> c_int {
    if isupper(c) == 1 {
        return (c as u8 + 32) as c_int;
    }

    c
}

/// Returns lowercase equivalent of character if it exists
#[no_mangle]
pub extern "C" fn toupper(c: c_int) -> c_int {
    if islower(c) == 1 {
        return (c as u8 - 32) as c_int;
    }

    c
}
