// bindgen build file

use std::env;
use std::path::PathBuf;

fn generate_bindings(h: &str, rs: &str) {
    let bindings = bindgen::Builder::default()
        .header(h)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new())) // track header changes with cargo
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join(rs))
        .expect("Couldn't write bindings!");
}

fn main() {
    generate_bindings("bindings/bsa.h", "bsa.rs");
    generate_bindings("bindings/fo3.h", "fo3.rs");
}
