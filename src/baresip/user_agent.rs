use std::ffi::{c_void, CString};
use std::io::{Error, Result};
use std::ptr;

use super::ffi;
use super::Account;

pub struct UserAgent(pub(super) *mut c_void);

impl UserAgent {
    // Create a SIP User-Agent
    pub fn new(address_of_record: &str) -> Result<Self> {
        let cstr = CString::new(address_of_record)?;
        let mut user_agent_ptr: *mut c_void = ptr::null_mut();
        let result_code: i32 =
            unsafe { ffi::ua_alloc(&mut user_agent_ptr, cstr.into_raw()) }.into();

        if result_code < 0 {
            Err(Error::from_raw_os_error(-result_code))
        } else {
            Ok(UserAgent(user_agent_ptr))
        }
    }

    #[inline]
    // Get Account of a User-Agent
    pub fn account(&self) -> Account {
        Account(unsafe { ffi::ua_account(self.0) })
    }

    #[inline]
    // Start registration of a User-Agent
    pub fn register(&self) -> Result<()> {
        let result_code: i32 = unsafe { ffi::ua_register(self.0) }.into();
        if result_code < 0 {
            Err(Error::from_raw_os_error(-result_code))
        } else {
            Ok(())
        }
    }

    #[inline]
    // Unregister User-Agent
    pub fn unregister(&self) {
        unsafe { ffi::ua_unregister(self.0) }
    }

    #[inline]
    // Check if a User-Agent is registered
    pub fn is_registered(&self) -> bool {
        unsafe { ffi::ua_isregistered(self.0) }
    }

    #[inline]
    // Start fallback registration checks (Cisco-keep-alive) of a User-Agent
    pub fn fallback(&self) -> Result<()> {
        let result_code: i32 = unsafe { ffi::ua_fallback(self.0) }.into();
        if result_code < 0 {
            Err(Error::from_raw_os_error(-result_code))
        } else {
            Ok(())
        }
    }
}
