use tcltk_sys::*;

struct Wrapper(*mut Tcl_Interp);

impl Drop for Wrapper {
    fn drop(&mut self) {
        unsafe { Tcl_DeleteInterp(self.0) }
    }
}

const program: &'static str = "
set A 6
set B 6
expr { $A * $B}
";

#[test]
fn use_embedded_tcl_engine() {
    let tcl_interp = unsafe {
        let tcl_interp = Tcl_CreateInterp();
        assert!(!tcl_interp.is_null());
        Wrapper(tcl_interp)
    };
}
