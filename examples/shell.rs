use std::error::Error;
use std::ffi::CString;
use std::io;
use std::io::prelude::*;

mod tcl {
    include!("../tests/embedded_interpreter.rs");
}

fn main() -> Result<(), Box<dyn Error>> {
    let interpreter = tcl::Wrapper::new();
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let command_str = line?;
        let script = CString::new(command_str)?;
        println!("{:?}", script);
    }

    Ok(())
}
