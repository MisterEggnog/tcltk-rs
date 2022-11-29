use pkg_config::Config;
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    let include_paths = Config::new()
        .atleast_version("8.6")
        .probe("tk")?
        .include_paths;

    let bindings = bindgen::Builder::default()
        .header("src/tk.h")
        .clang_args(
            include_paths
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

    Ok(())
}
