use std::path::PathBuf;
use std::process::Command;

#[test]
fn run_tests() {
    let build_dir = env!("OUT_DIR");
    let tcl_dir = "tcl8.6.12";

    // We must copy tests to tcl dir
    // This is probably not okay
    Command::new("cp")
        .args([
            "-Rf",
            "tcl/tests",
            [&build_dir, &tcl_dir]
                .iter()
                .collect::<PathBuf>()
                .to_str()
                .unwrap(),
        ])
        .status()
        .expect("Failed to execute process");

    panic!("{:?}", env!("OUT_DIR"));
}
