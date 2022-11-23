use std::ffi::OsStr;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn set_library_path(shell: &str, env_name: &str) {
    // The only way I know how to find this is to invoke tcl & check it's
    // output
    let mut child_tclsh = Command::new("/usr/bin/env")
        .arg(OsStr::new(&shell))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to find tclsh");

    child_tclsh
        .stdin
        .take()
        .expect("Failed to open stdin")
        .write_all(format!("puts ${}", env_name.to_lowercase()).as_bytes())
        .expect("Failed to write to stdin");

    let output = child_tclsh
        .wait_with_output()
        .expect("Failed to read stdout");
    let output = String::from_utf8(output.stdout).unwrap();

    println!("cargo:rustc-env={}={}", env_name.to_uppercase(), output);
}
