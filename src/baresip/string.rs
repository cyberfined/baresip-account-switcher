use std::ffi::CStr;
use std::os::raw::c_char;
use std::str::Utf8Error;

#[repr(C)]
pub struct String<'a> {
    ptr: &'a c_char,
    len: usize,
}

impl<'a> String<'a> {
    #[inline]
    pub fn from_cstr(cstr: &'a CStr) -> String<'a> {
        let ptr = unsafe { &*(cstr.as_ptr() as *const c_char) };
        let len = cstr.to_bytes().len();
        String { ptr, len }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const c_char {
        self.ptr as *const c_char
    }

    #[inline]
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        let cstr = unsafe { CStr::from_ptr(self.as_ptr()) };
        cstr.to_str()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
}
