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
