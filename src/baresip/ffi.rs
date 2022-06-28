use std::ffi::{c_void, CStr};
use std::io::{Error, ErrorKind, Result};
use std::os::raw::{c_char, c_int};

extern "C" {
    pub fn ua_alloc(
        user_agent_ptr: *mut *mut c_void,
        address_of_record: *const c_char,
    ) -> c_int;
    pub fn ua_account(user_agent: *const c_void) -> *mut c_void;
    pub fn ua_register(user_agent: *mut c_void) -> c_int;
    pub fn ua_unregister(user_agent: *mut c_void);
    pub fn ua_fallback(user_agent: *mut c_void) -> c_int;
    pub fn ua_isregistered(user_agent: *mut c_void) -> bool;

    pub fn account_regint(account: *const c_void) -> u32;
    pub fn account_prio(account: *const c_void) -> u32;
    pub fn account_auth_user(account: *const c_void) -> *const c_char;
    pub fn account_auth_pass(account: *const c_void) -> *const c_char;
    pub fn account_set_auth_pass(
        account: *const c_void,
        password: *const c_char,
    ) -> c_int;

    pub fn baresip_commands() -> *mut c_void;
    pub fn cmd_register(
        commands: *mut c_void,
        new_commands: *const c_void,
        num_new_commands: usize,
    ) -> c_int;
    pub fn cmd_unregister(commands: *mut c_void, unregister_commands: *const c_void);

    pub fn conf_path_get(path: *mut c_char, size: usize) -> c_int;
}

#[inline]
pub fn c_char_to_char(ch: c_char) -> Option<char> {
    let chu8 = if ch <= 0 { return None } else { ch as u8 };
    if chu8.is_ascii() {
        Some(chu8 as char)
    } else {
        None
    }
}

#[inline]
pub fn str_from_ptr(ptr: *const c_char, invalid_str_error: &str) -> Result<String> {
    let cstr = unsafe { CStr::from_ptr(ptr) };
    Ok(cstr
        .to_str()
        .map_err(|_| Error::new(ErrorKind::InvalidData, invalid_str_error))?
        .to_string())
}

#[inline]
pub fn str_from_buf(buf: &[u8], invalid_str_error: &str) -> Result<String> {
    let zero_idx = match buf.iter().copied().position(|x| x == 0) {
        Some(idx) => idx + 1,
        None => buf.len(),
    };
    let cstr = CStr::from_bytes_with_nul(&buf[0..zero_idx])
        .map_err(|_| Error::new(ErrorKind::InvalidData, invalid_str_error))?;
    Ok(cstr
        .to_str()
        .map_err(|_| Error::new(ErrorKind::InvalidData, invalid_str_error))?
        .to_string())
}
