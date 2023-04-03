use jobserver::Client;
use pkg_config::Config;
use std::env;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;

fn token_name() -> PathBuf {
    [
        env::var("OUT_DIR").unwrap(),
        "TCL_SYS_CONFIGURED".to_string(),
    ]
    .iter()
    .collect()
}

fn token_exists() -> bool {
    Path::new(&token_name()).exists()
}

fn token_touch() -> anyhow::Result<()> {
    File::create(token_name())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:LOCAL_INCLUDE_DIR=5");

    if std::env::var("DOCS_RS").is_ok() {
        bindgen(&["tcl8.6.12/generic"].map(|f| PathBuf::from(f)));
        return Ok(());
    }

    let include_paths = match Config::new().atleast_version("8.6").probe("tcl") {
        Ok(lib) => lib.include_paths,
        Err(_) => {
            copy_tcl_dir()?;
            configure_unix()?;
            token_touch()?;

            build_unix()?;
            vec![]
        }
    };

    bindgen(&include_paths);

    Ok(())
}

fn copy_tcl_dir() -> anyhow::Result<()> {
    if token_exists() {
        return Ok(());
    }

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
    let jobserver = unsafe { Client::from_env().unwrap() };

    let mut cmd = Command::new("make");
    jobserver.configure(&mut cmd);
    let output = cmd
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
    if token_exists() {
        return Ok(());
    }

    let out_dir = env::var("OUT_DIR")?;
    let work_dir = "tcl8.6.12/unix";
    let command = fs::canonicalize(format!("{}/{}/configure", out_dir, work_dir)).expect(&format!(
        "canonicalize failed for {out_dir}/{work_dir}/configure"
    ));
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

fn bindgen(include_dir: &[PathBuf]) {
    let header: PathBuf = ["src", "defs.h"].iter().collect();

    let bindings = bindgen::Builder::default()
        .header(header.to_str().unwrap())
        .blocklist_item("stdin")
        .blocklist_item("stdout")
        .blocklist_item("stderr")
        .opaque_type("Tcl_Interp")
        .clang_args(
            include_dir
                .iter()
                .flat_map(|dir| vec!["-I", dir.to_str().unwrap()]),
        )
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings");
}
