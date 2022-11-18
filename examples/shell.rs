use std::error::Error;
use std::io;
use std::io::prelude::*;
include!("../tests/embedded_interpreter.rs");

fn main() -> Result<(), Box<dyn Error>> {
    let interpreter = Wrapper::new();
    let ifs = io::stdin();
    let handle = ifs.lock();

    for line in handle.lines() {
        let command_str = line?;
        println!("{}", command_str);
    }

    Ok(())
}
