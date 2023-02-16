extern crate cbindgen;

use cbindgen::Language;
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(format!(
            "target/{}/{}.h",
            env::var("PROFILE").unwrap(),
            env::var("CARGO_PKG_NAME").unwrap()
        ));
}
