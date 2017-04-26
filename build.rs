extern crate gcc;
extern crate bindgen;

use std::fs;
use std::io;
use std::env;
use std::path::PathBuf;

fn find_sources<S>(path_str: S, ext: &str) -> Vec<PathBuf> where S: Into<PathBuf> {
    let path = path_str.into();
    fs::read_dir(&path).expect(format!("unable to open path {:?}", &path).as_str())
        .filter_map(|f| f.ok())
        .filter_map(|f| {
            let is_file = f.metadata().expect(format!("unable to read metadata for {:?}", f.path()).as_str()).is_file();

            if is_file {
                match f.path().as_path().extension() {
                    Some(s) if s == ext => Some(f.path().clone()),
                    _ => None
                }
            } else {
                None
            }
        })
        .collect()
}

fn prebuild() -> io::Result<()> {
    let build_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let lua_dir = PathBuf::new().join("lua-source").join("src");
    let mut config = gcc::Config::new();

    let sources = find_sources(&lua_dir, "c");

    for source in sources {
        config.file(source);
    }

    config.compile("liblua.a");

    let glue_path = build_dir.join("glue.rs");

    // Ensure the presence of glue.rs
    if !glue_path.exists() {
        let gen = bindgen::Builder::default()
            .no_unstable_rust()
            .whitelisted_var("LUA.*")
            .whitelisted_var("EXT_LUA.*")
            .whitelisted_type("lua.*")
            .header("src/glue/includes.h");

        gen.generate().expect("Failed to parse source")
            .write_to_file(glue_path).expect("Failed to write bindings");
    }

    Ok(())
}

fn main() {
    match prebuild() {
        Err(e) => panic!("Error: {}", e),
        Ok(()) => (),
    }
}
