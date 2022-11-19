use std::ffi::CString;
use tcltk_sys::*;

pub struct Wrapper(pub *mut Tcl_Interp);

impl Wrapper {
    pub fn new() -> Wrapper {
        unsafe {
            let tcl_interp = Tcl_CreateInterp();
            assert!(!tcl_interp.is_null());
            Wrapper(tcl_interp)
        }
    }
}

impl Drop for Wrapper {
    fn drop(&mut self) {
        unsafe { Tcl_DeleteInterp(self.0) }
    }
}

const PROGRAM: &'static str = "\
set A 6
set B 6
expr { $A * $B}";

#[test]
fn use_embedded_tcl_engine() {
    let tcl_interp = Wrapper::new();

    let script = CString::new(PROGRAM).expect("Unable to create cstring");
    assert_eq!(
        unsafe { Tcl_Eval(tcl_interp.0, script.as_ptr()) },
        TCL_OK as i32,
        "Failed to run tcl script"
    );

    let obj_result = unsafe { Tcl_GetObjResult(tcl_interp.0) };
    assert!(!obj_result.is_null());

    let mut result = 0;
    assert_eq!(
        unsafe { Tcl_GetIntFromObj(tcl_interp.0, obj_result, &mut result) },
        TCL_OK as i32,
        "Contents of tcl result was not an integer"
    );

    assert_eq!(result, 36);
}
