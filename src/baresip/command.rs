use std::ffi::{c_void, CStr, CString};
use std::io::{Error, ErrorKind, Result};
use std::marker::PhantomData;
use std::os::raw::{c_char, c_int};
use std::vec::Vec;

use super::ffi;

#[repr(C)]
pub struct FfiArg {
    key: c_char,
    param: *const c_char,
    data: *const c_void,
}

#[repr(C)]
struct FfiCommand<'a, T: 'a> {
    name: *const c_char,
    key: c_char,
    flags: c_int,
    description: *const c_char,
    handler: extern "C" fn(dummy: *mut c_void, arg: *mut FfiArg) -> c_int,
    phantom: PhantomData<&'a T>,
}

#[derive(Clone, Copy)]
pub enum Flags {
    CmdProc = 0,
    CmdFunc = 1,
}

pub struct Arg<'a> {
    pub key: Option<char>,
    pub param: &'a str,
}

impl<'a> Arg<'a> {
    pub unsafe fn new(arg_ptr: *mut FfiArg) -> Result<Option<Self>> {
        let arg = match arg_ptr.as_ref() {
            Some(arg) => arg,
            None => return Ok(None),
        };
        if arg.param.is_null() {
            return Ok(None);
        }

        let key = ffi::c_char_to_char(arg.key);
        let param = CStr::from_ptr(arg.param).to_str().map_err(|_| {
            Error::new(
                ErrorKind::InvalidData,
                "invalid string data in command handler argument",
            )
        })?;

        Ok(Some(Arg { key, param }))
    }
}

pub struct Command<'a> {
    pub name: &'a str,
    pub key: Option<char>,
    pub flags: Flags,
    pub description: &'a str,
    pub handler: extern "C" fn(dummy: *mut c_void, arg: *mut FfiArg) -> c_int,
}

pub struct FfiCommandVec<'a, T: 'a> {
    commands: Vec<FfiCommand<'a, T>>,
}

impl<'a, T: 'a> FfiCommandVec<'a, T> {
    pub fn new(commands: &[Command]) -> Result<Self> {
        let mut ffi_commands = Vec::with_capacity(commands.len());

        for cmd in commands.iter() {
            ffi_commands.push(FfiCommand {
                name: CString::new(cmd.name)?.into_raw(),
                key: cmd.key.map(|k| k as c_char).unwrap_or(0),
                flags: cmd.flags as c_int,
                description: CString::new(cmd.description)?.into_raw(),
                handler: cmd.handler,
                phantom: PhantomData,
            })
        }

        Ok(FfiCommandVec {
            commands: ffi_commands,
        })
    }

    pub fn register(&self) -> Result<()> {
        let all_commands = unsafe { ffi::baresip_commands() };
        let new_commands = self.commands.as_ptr() as *const c_void;
        let result_code: i32 =
            unsafe { ffi::cmd_register(all_commands, new_commands, self.commands.len()) }
                .into();

        if result_code < 0 {
            Err(Error::from_raw_os_error(-result_code))
        } else {
            Ok(())
        }
    }

    pub fn unregister(&self) {
        let all_commands = unsafe { ffi::baresip_commands() };
        let unregister_commands = self.commands.as_ptr() as *const c_void;
        unsafe { ffi::cmd_unregister(all_commands, unregister_commands) }
    }
}
