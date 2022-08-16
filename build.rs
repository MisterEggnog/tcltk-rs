use pkg_config::Config;
use std::env;
use std::fs;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");

    let lib = Config::new().atleast_version("8.6").probe("tcl");
    match lib {
        Ok(_) => panic!("This has never worked before"),
        Err(_) => {
            copy_tcl_dir()?;
            eprintln!("Copied dir");
            build_unix()?
        }
    }
    println!("Exit succesful");

    Ok(())
}

fn copy_tcl_dir() -> anyhow::Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    Command::new("cp")
        .args(["-lRf", "tcl8.6.12/", &out_dir])
        .status()
        .expect("Failed to execute process");

    Ok(())
}

fn build_unix() -> anyhow::Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    eprintln!("{}", out_dir);
    let command = fs::canonicalize(format!("{}/tcl8.6.12/unix/configure", out_dir))?;
    eprintln!("canonicsed");
    eprintln!("{:?}", command);
    let output = Command::new(command)
        .arg(format!("--prefix={}", out_dir))
        .arg("--enable-share=no")
        .current_dir(format!("{}/unix", out_dir))
        .output()
        .expect("Failed to execute process");

    println!("{}", String::from_utf8(output.stderr).unwrap());
    Ok(())
}
