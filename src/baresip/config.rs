use std::io::{Error, Result};
use std::os::raw::c_char;

use super::ffi;

pub fn get_path() -> Result<String> {
    let mut buf = [0u8; 1024];
    let buf_ptr = buf.as_mut_ptr() as *mut c_char;
    let result_code: i32 = unsafe { ffi::conf_path_get(buf_ptr, buf.len()) }.into();
    if result_code < 0 {
        return Err(Error::from_raw_os_error(result_code));
    }

    ffi::str_from_buf(&buf[..], "config_path_get returned invalid string")
}
