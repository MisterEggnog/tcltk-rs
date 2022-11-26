#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod find_lib;
use std::env::{var, VarError};

pub fn init_library() {
    if let Err(VarError::NotPresent) = var("TCL_LIBRARY") {
        find_lib::set_library_path("tclsh", "TCL_LIBRARY");
    }
}
