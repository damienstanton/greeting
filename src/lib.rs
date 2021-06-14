#![allow(dead_code, unused_imports)]
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

pub fn greeting(s: &str) -> String {
    format!("Hello, {}!", s)
}

#[no_mangle]
pub extern "C" fn greeting_c(s: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };

    CString::new(greeting(
        c_str.to_str().expect("string must be convertible"),
    ))
    .expect("input cannot be null")
    .into_raw()
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn rust_greeting() {
        assert_eq!(greeting("Damien"), String::from("Hello, Damien!"));
    }
}
