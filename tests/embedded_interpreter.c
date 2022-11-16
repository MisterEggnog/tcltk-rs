#include <tcl/tcl.h>

// Derived from
// https://stackoverflow.com/questions/13402536/embedding-tcl-in-a-c-program
//
// Compile with `gcc tests/embedded_interpreter.c -ltcl`

int
main(void) {
	Tcl_Interp* tcl_interp = Tcl_CreateInterp();

	Tcl_DeleteInterp(tcl_interp);
}
