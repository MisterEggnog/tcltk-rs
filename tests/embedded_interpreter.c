#include <tcl/tcl.h>

// Derived from
// https://stackoverflow.com/questions/13402536/embedding-tcl-in-a-c-program
//
// Compile with `gcc tests/embedded_interpreter.c -ltcl`

const char program[] = "set A 6\n"
	"set B 6\n"
	"expr { $A * $B}";

int
main(void) {
	Tcl_Interp* tcl_interp = Tcl_CreateInterp();

	Tcl_Eval(tcl_interp, program);

	Tcl_DeleteInterp(tcl_interp);
}
