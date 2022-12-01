//! Rust bindings for the Tcl interpreter
//!
//! Function specific documentation can be found at the [Tcl website]
//!
//! [Tcl website]: https://www.tcl-lang.org/man/tcl8.6/TclLib/contents.htm
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
