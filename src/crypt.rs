extern crate libc;
use self::libc::{
    c_char,
    c_long,
    c_int,
    strlen
};

use std::ffi::CString;
use std::vec::Vec;
use std::mem::uninitialized;

#[repr(C)]
#[allow(dead_code)]
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


pub fn crypt (key: CString, salt: CString) -> Option <Vec <u8>>
{
    let ckey       = key.as_ptr ();
    let csalt      = salt.as_ptr ();
    let static_str = unsafe {
        let mut data = uninitialized::<crypt_data> ();
        data.initialized = 0;
        crypt_r (ckey, csalt, &mut data)
    };
    if static_str.is_null () {
        None
    }
    else {
        let result = unsafe {
            let length = strlen (static_str) as usize;
            Vec::<u8>::from_raw_buf (static_str as *const u8, length)
        };
        Some (result)
    }
}
