use std::process::Command;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

    let bindings = bindgen::Builder::default()
        .header("raybridge.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
        
    
    // Run the "make" command
    let status = Command::new("make")
        .status()
        .expect("Failed to run make");

    if !status.success() {
        panic!("Make process failed with exit code: {:?}", status);
    }
    
    // Link the Rust project with the C static library
    println!("cargo:rustc-link-search=native={}", env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=raybridge");
    
    println!("cargo:rustc-link-search=native={}/raylib/src", env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=raylib");
}