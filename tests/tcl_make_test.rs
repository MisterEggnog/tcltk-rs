use std::path::PathBuf;
use std::process::Command;

#[test]
#[ignore]
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

    let test_result = Command::new("make")
        .arg("test")
        .current_dir(
            [&build_dir, &tcl_dir, "unix"]
                .iter()
                .collect::<PathBuf>()
                .to_str()
                .unwrap(),
        )
        .output()
        .expect("Failed to execute process");

    assert!(
        test_result.status.success(),
        "Running make tests for tcl failed with stderr:\n{}",
        std::str::from_utf8(&test_result.stderr).unwrap()
    );
}
