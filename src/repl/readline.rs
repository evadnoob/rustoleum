
use std::ffi::{CStr, CString};
use std::{error, str};
use std::io::prelude::*;

mod ext_readline {
    extern crate libc;
    use self::libc::c_char;
    #[link(name = "readline")]
    extern {
        pub fn add_history(line: *const c_char);
        pub fn readline(p: *const c_char) -> *const c_char;
    }
}

pub fn add_history(line: &str) {
    unsafe {
        ext_readline::add_history(CString::new(line).unwrap().as_ptr());
    }
}


pub fn readline(prompt: &str) -> Result<String, String> {
    let cprmt = CString::new(prompt).unwrap();
    unsafe {
        let ptr = ext_readline::readline(cprmt.as_ptr());
        if ptr.is_null() {  // user pressed Ctrl-D
            return Err("null pointer".to_string());
        } else {
            let ret = str::from_utf8(CStr::from_ptr(ptr).to_bytes());
            let ret = ret.ok().map(|s| s.to_string());
            ext_readline::libc::free(ptr as *mut _);
            
            return Ok(ret.unwrap());
        }
    }
}
