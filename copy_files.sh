#!/bin/sh

rm -r tcl-sys/tcl8.6.12 tk-sys/tk8.6

mkdir tcl-sys/tcl8.6.12
cp -r tcl/generic tcl/compat tcl/libtommath tcl/macosx tcl/unix tcl/win tcl/library tcl/doc tcl-sys/tcl8.6.12

mkdir tk-sys/tk8.6 tk-sys/tk8.6/generic
cp -r tk/generic/*.h tk-sys/tk8.6/generic
