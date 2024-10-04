// bindgen build file

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("bindings/fo3.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new())) // track header changes with cargo
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("fo3.rs"))
        .expect("Couldn't write bindings!");
}
