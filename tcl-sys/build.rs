use pkg_config::Config;
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");

    let include_paths = match Config::new().atleast_version("8.6").probe("tcl") {
        Ok(lib) => lib.include_paths,
        Err(_) => panic!("Currently does not support static building"),
    };

    bindgen(&include_paths);

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
