extern crate libc;

use std::{str, vec};
use std::ffi::CString;
use self::libc::{c_char, size_t, c_int};

extern {
  pub fn gethostname(name: *mut c_char, size: size_t) -> c_int;
}


fn main() {
    let name = String::new();
    //let len = 255; 
    let buf = name.as_ptr() as *mut c_char;
    //let buf = String::with_capacity(len);
    let len = buf.len() as size_t;

    let err = unsafe {gethostname (buf, len as u64)};
    println!("err {}", err);
    if err != 0 { println!("oops, gethostname failed"); return; }

    // find the first 0 byte (i.e. just after the data that gethostname wrote)
    //let actual_len = buf.iter().position(|byte| *byte == 0).unwrap_or(len);

    // trim the hostname to the actual data written
    //println!(str::from_utf8_slice(buf.slice_to(actual_len)));
    println!("{:?}", buf);
}
