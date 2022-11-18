use std::io;
use std::io::prelude::*;
include!("../tests/embedded_interpreter.rs");

fn main() -> io::Result<()> {
    let interpreter = Wrapper::new();

    Ok(())
}
