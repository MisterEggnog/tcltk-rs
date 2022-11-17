use tcltk_sys::*;

struct Wrapper(*mut Tcl_Interp);

impl Drop for Wrapper {
    fn drop(&mut self) {
        unsafe { Tcl_DeleteInterp(self.0) }
    }
}
