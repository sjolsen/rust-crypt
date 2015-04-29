extern crate libc;
use libc::{
    c_char,
    c_long,
    c_int
};

#[repr(C)]
struct crypt_data {
    keysched         : [c_char; 16 * 8],
    sb0              : [c_char; 32768],
    sb1              : [c_char; 32768],
    sb2              : [c_char; 32768],
    sb3              : [c_char; 32768],
    crypt_3_buf      : [c_char; 14],
    current_salt     : [c_char; 2],
    current_saltbits : c_long,
    direction        : c_int,
    initialized      : c_int
}

#[link(name = "crypt")]
extern "C" {
    fn crypt_r (key  : *const c_char,
                salt : *const c_char,
                data : *mut crypt_data) -> *mut c_char;
}

use libc::funcs::c95::string::strlen;
use std::ffi::CString;
use std::vec;

pub fn crypt (key: CString, salt: CString) -> Option <Vec <u8>>
{
    let ckey       = key.as_ptr ();
    let csalt      = salt.as_ptr ();
    let data       = crypt_data {initialized : 0};
    let static_str = unsafe {
        crypt_r (ckey, csalt, &data)
    };
    if static_str.is_null () {
        None
    }
    else {
        let result = unsafe {
            let length = strlen (static_str);
            Vec::<u8>::from_raw_parts (static_str, length, length)
        };
        Some (result)
    }
}
