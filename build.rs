use bindgen;

use std::env;
use std::error::Error;
use std::path::PathBuf;

fn get_binding_for_header(path: &str) -> bindgen::Bindings {
    let bindings = bindgen::Builder::default()
        .header(path)
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .derive_default(true)
        .generate_cstr(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rustc-link-lib=dylib=pam");

    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    let ffi_pam = get_binding_for_header("ffi/pam.h");

    ffi_pam
        .write_to_file(out_path.join("pam.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}
