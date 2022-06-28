pub mod baresip;
pub mod config;
pub mod use_accounts;

use std::collections::HashMap;
use std::ffi::c_void;
use std::io::{Error, ErrorKind};
use std::os::raw::{c_char, c_int};

use crate::baresip::{Arg, Command, FfiArg, FfiCommandVec, Flags, ModExport, UserAgent};

static mut USER_AGENTS: Option<HashMap<String, UserAgent>> = None;
static mut COMMAND_VECTOR: Option<FfiCommandVec<'static, i32>> = None;

extern "C" fn use_accounts_handler(_dummy: *mut c_void, ffi_arg: *mut FfiArg) -> c_int {
    let res = unsafe { Arg::new(ffi_arg) }
        .and_then(|opt_arg| match unsafe { &USER_AGENTS } {
            Some(user_agents) => Ok((opt_arg, user_agents)),
            None => Err(Error::new(
                ErrorKind::Other,
                "use_accounts failed to load accounts",
            )),
        })
        .and_then(|(opt_arg, user_agents)| match opt_arg {
            Some(arg) => use_accounts::handler(arg, user_agents),
            None => Err(Error::new(
                ErrorKind::Other,
                "use_accounts expected an argument",
            )),
        });

    match res {
        Ok(()) => 0,
        Err(err) => match err.raw_os_error() {
            Some(code) => code.into(),
            None => {
                println!("use_accounts: {}", err);
                -1
            }
        },
    }
}

static COMMAND_SLICE: [Command; 1] = [Command {
    name: "use_accounts",
    key: None,
    flags: Flags::CmdProc,
    description: "register given accounts and unregister others",
    handler: use_accounts_handler,
}];

extern "C" fn mod_init() -> c_int {
    let res = FfiCommandVec::new(&COMMAND_SLICE).and_then(|commands| {
        commands
            .register()
            .and(config::read_config())
            .map(|user_agents| (commands, user_agents))
    });

    match res {
        Ok((commands, user_agents)) => unsafe {
            USER_AGENTS = Some(user_agents);
            COMMAND_VECTOR = Some(commands);
            0
        },
        Err(err) => match err.raw_os_error() {
            Some(code) => code.into(),
            None => {
                println!("Error: {}", err);
                -1
            }
        },
    }
}

extern "C" fn mod_close() -> c_int {
    match unsafe { &COMMAND_VECTOR } {
        Some(commands) => commands.unregister(),
        None => {}
    }
    0
}

static NAME: [u8; 8] = *b"account\0";
static KIND: [u8; 12] = *b"application\0";

#[no_mangle]
#[used]
pub static exports: ModExport = ModExport {
    name: NAME.as_ptr() as *const c_char,
    kind: KIND.as_ptr() as *const c_char,
    mod_init: mod_init,
    mod_close: mod_close,
};
