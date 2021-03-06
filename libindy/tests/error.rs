use std::{
    ffi::{CStr, CString},
    ptr,
    str::Utf8Error,
    thread::sleep,
    time::Duration,
};

use indyrs::CommandHandle;
use libc::c_char;

#[test]
fn get_current_error_works_for_no_error() {
    let mut error_json_p: *const c_char = ptr::null();

    unsafe {
        indy_get_current_error(&mut error_json_p);
    }

    assert_eq!(None, c_str_to_string(error_json_p).unwrap());
}

#[test]
fn get_current_error_works_for_sync_error_occurred() {
    let mut error_json_p: *const c_char = ptr::null();

    unsafe { indy_set_runtime_config(ptr::null()) };

    unsafe {
        indy_get_current_error(&mut error_json_p);
    }

    assert!(c_str_to_string(error_json_p).unwrap().is_some());
}

#[test]
fn get_current_error_works_for_async_error_occurred() {
    extern "C" fn cb(_command_handle_: CommandHandle, _err: u32, _verkey: *const c_char) {
        let mut error_json_p: *const c_char = ptr::null();
        unsafe { indy_get_current_error(&mut error_json_p) };
        assert!(c_str_to_string(error_json_p).unwrap().is_some());
    }

    let did = CString::new("VsKV7grR1BUE29mG2Fm2kX").unwrap();
    let verkey = CString::new("wrong_verkey").unwrap();

    unsafe { indy_abbreviate_verkey(1, did.as_ptr(), verkey.as_ptr(), Some(cb)) };

    sleep(Duration::from_secs(1));
}

extern "C" {
    pub fn indy_set_runtime_config(config: *const c_char) -> i32;

    pub fn indy_get_current_error(error_json: *mut *const c_char);

    pub fn indy_abbreviate_verkey(
        command_handle: CommandHandle,
        did: *const c_char,
        full_verkey: *const c_char,
        cb: Option<extern "C" fn(command_handle_: CommandHandle, err: u32, verkey: *const c_char)>,
    ) -> i32;
}

pub fn c_str_to_string<'a>(cstr: *const c_char) -> Result<Option<&'a str>, Utf8Error> {
    if cstr.is_null() {
        return Ok(None);
    }

    unsafe {
        match CStr::from_ptr(cstr).to_str() {
            Ok(str) => Ok(Some(str)),
            Err(err) => Err(err),
        }
    }
}
