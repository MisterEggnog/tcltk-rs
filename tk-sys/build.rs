use pkg_config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Config::new().atleast_version("8.6").probe("tcl")?;
    Ok(())
}
