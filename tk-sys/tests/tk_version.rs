use std::ffi::{CStr, CString};
use tcl_sys::*;

mod common;
use common::Wrapper;

fn return_code(code: u32) -> &'static str {
    match code {
        TCL_OK => stringify!(TCL_OK),
        TCL_ERROR => stringify!(TCL_ERROR),
        TCL_RETURN => stringify!(TCL_RETURN),
        TCL_BREAK => stringify!(TCL_BREAK),
        TCL_CONTINUE => stringify!(TCL_CONTINUE),
        _ => unreachable!(),
    }
}

#[test]
fn tk_is_expected_version() {
    let interpreter = Wrapper::new();
    let script = CString::new("package require Tk").expect("Unable to convert string to cstring");
    let interp_result = unsafe { Tcl_Eval(interpreter.0, script.as_ptr()) };
    let return_code = return_code(interp_result.try_into().unwrap());
    assert_eq!(
        interp_result,
        TCL_OK as i32,
        "Failed to run script, return error: {return_code}, reason: {:?}",
        unsafe { CStr::from_ptr(Tcl_GetString(Tcl_GetObjResult(interpreter.0))) }
    );
}
