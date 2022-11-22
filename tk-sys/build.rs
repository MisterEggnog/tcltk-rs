use pkg_config::Config;
use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    Config::new().atleast_version("8.6").probe("tk")?;

    let bindings = bindgen::Builder::default()
        .header("src/tk.h")
        .clang_args(["-I", "/usr/include/tcl"])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings");

    set_library_path();

    Ok(())
}

fn set_library_path() {
    // The only way I know how to find this is to invoke tcl & check it's
    // output
    let mut child_tclsh = Command::new("/usr/bin/env")
        .arg("tclsh")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to find tclsh");

    child_tclsh
        .stdin
        .take()
        .expect("Failed to open stdin")
        .write_all("puts $tcl_library".as_bytes())
        .expect("Failed to write to stdin");
}
