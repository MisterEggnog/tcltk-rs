use pkg_config::Config;
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");

    let lib = Config::new().atleast_version("8.6").probe("tcl");
    match lib {
        Ok(_) => panic!("This has never worked before"),
        Err(_) => {
            copy_tcl_dir()?;
            configure_unix()?;
            build_unix()?
        }
    }

    Ok(())
}

fn copy_tcl_dir() -> anyhow::Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    Command::new("cp")
        .args(["-Rf", "tcl8.6.12/", &out_dir])
        .status()
        .expect("Failed to execute process");

    Ok(())
}

fn build_unix() -> anyhow::Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let work_dir = "tcl8.6.12/unix";
    eprintln!("{}", out_dir);

    let output = Command::new("make")
        .arg(format!("--jobs={}", num_cpus::get()))
        .current_dir(format!("{}/{}", out_dir, work_dir))
        .output()
        .expect("Failed to execute process");
    println!("{}", String::from_utf8(output.stderr).unwrap());

    let output = Command::new("make")
        .arg("install")
        .current_dir(format!("{}/{}", out_dir, work_dir))
        .output()
        .expect("Failed to execute process");
    println!("{}", String::from_utf8(output.stderr).unwrap());

    Ok(())
}

fn configure_unix() -> anyhow::Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let configured_file = format!("{}/TCL_SYS_CONFIGURED", out_dir);

    if Path::new(&configured_file).exists() {
        return Ok(());
    }

    drop(File::create(configured_file)?);

    let work_dir = "tcl8.6.12/unix";
    let command = fs::canonicalize(format!("{}/{}/configure", out_dir, work_dir))?;
    eprintln!("canonicsed");
    eprintln!("{:?}", command);
    let output = Command::new(command)
        .arg(format!("--prefix={}", out_dir))
        .arg("--enable-share=no")
        .current_dir(format!("{}/{}", out_dir, work_dir))
        .output()
        .expect("Failed to execute process");
    println!("{}", String::from_utf8(output.stderr).unwrap());

    Ok(())
}
