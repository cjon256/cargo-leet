use anyhow::Context;
use convert_case::{Case, Casing};
use log::{info, warn};
use std::{
    fs::{remove_file, OpenOptions},
    io::Write,
    path::PathBuf,
    process::Command,
};

fn update_lib(module_name: &str) -> anyhow::Result<()> {
    info!("Adding {module_name} to libs.rs");
    let lib_path = PathBuf::from("src/lib.rs");
    let mut lib = OpenOptions::new().append(true).open(lib_path)?;
    let _ = lib.write(format!("pub mod {module_name};").as_bytes())?;
    Ok(())
}

pub fn write_file(title_slug: &str, code_snippet: String) -> anyhow::Result<()> {
    info!("Writing code to disk for {title_slug}");
    let slug_snake = title_slug.to_case(Case::Snake);
    let module_name = slug_snake; // TODO: Find way to specify desired new file name from a config
    info!("Module name is: {module_name}");
    let path = PathBuf::from(format!("src/{module_name}.rs"));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path.clone())?;
    file.write_all(code_snippet.as_bytes())?;
    let lib_update_status = update_lib(&module_name);
    if lib_update_status.is_err() {
        warn!("Cleaning up after updating lib.rs failed");
        // clean up
        remove_file(path)?;
        lib_update_status?;
    }

    info!("Going to run rustfmt on files");
    Command::new("cargo")
        .arg("fmt")
        .arg("--all")
        .output()
        .context("Error running rustfmt")?;
    Ok(())
}
