use std::ffi::{CStr, CString};
use tcl_sys::*;

mod common;
use common::Wrapper;

#[test]
fn tk_is_expected_version() {
    let interpreter = Wrapper::new();
    let script = CString::new("package require Tk").expect("Unable to convert string to cstring");
    assert_eq!(
        unsafe { Tcl_Eval(interpreter.0, script.as_ptr()) },
        TCL_OK as i32,
        "Failed to run script, reason: {:?}",
        unsafe { CStr::from_ptr(Tcl_ErrnoMsg(Tcl_GetErrno())) }
    );
}
