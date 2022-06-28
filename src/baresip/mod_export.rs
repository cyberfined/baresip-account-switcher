use std::os::raw::{c_char, c_int};

#[repr(C)]
pub struct ModExport {
    pub name: *const c_char,
    pub kind: *const c_char,
    pub mod_init: extern "C" fn() -> c_int,
    pub mod_close: extern "C" fn() -> c_int,
}

unsafe impl Sync for ModExport {}
