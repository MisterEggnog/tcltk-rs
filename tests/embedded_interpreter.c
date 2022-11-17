#include <assert.h>
#include <tcl/tcl.h>
#include <stdio.h>

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

	if (Tcl_Eval(tcl_interp, program) != TCL_OK) {
		fprintf(stderr, "Failed to run tcl script\n");
		return 1;
	}

	Tcl_Obj* obj_result = Tcl_GetObjResult(tcl_interp);
	assert(obj_result != NULL);

	int result;
	if (Tcl_GetIntFromObj(tcl_interp, obj_result, &result) != TCL_OK) {
		fprintf(stderr, "Contents of tcl result was not an integer\n");
		return 1;
	}

	assert(result == 36);

	Tcl_DeleteInterp(tcl_interp);
}
