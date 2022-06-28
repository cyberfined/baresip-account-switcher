use std::ffi::{c_void, CString};
use std::io::{Error, Result};

use super::ffi;

pub struct Account(pub(super) *mut c_void);

impl Account {
    #[inline]
    // Get the registration interval of an account
    pub fn registration_interval(&self) -> u32 {
        unsafe { ffi::account_regint(self.0) }
    }

    #[inline]
    // Get the priority of an account. Priority 0 is default.
    pub fn priority(&self) -> u32 {
        unsafe { ffi::account_prio(self.0) }
    }

    #[inline]
    // Get the authentication username of an account
    pub fn auth_user(&self) -> Result<String> {
        let ptr = unsafe { ffi::account_auth_user(self.0) };
        ffi::str_from_ptr(ptr, "account_auth_user returned invalid string")
    }

    #[inline]
    // Get the SIP authentication password of an account
    pub fn auth_pass(&self) -> Result<String> {
        let ptr = unsafe { ffi::account_auth_pass(self.0) };
        ffi::str_from_ptr(ptr, "account_auth_pass returned invalid string")
    }

    #[inline]
    // Set the authentication password for a SIP account
    pub fn set_auth_pass(&self, password: &str) -> Result<()> {
        let cstr = CString::new(password)?;
        let result_code: i32 =
            unsafe { ffi::account_set_auth_pass(self.0, cstr.into_raw()) }.into();
        if result_code < 0 {
            Err(Error::from_raw_os_error(-result_code))
        } else {
            Ok(())
        }
    }
}
