extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let build_dir = env::current_dir().unwrap().join("threadx/build");
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", build_dir.display());
    println!("cargo:rustc-link-search=C:\\dev\\scratch\\azure_rtos\\threadx\\ports\\cortex_a9\\gnu\\inc");


    //println!("cargo:warning=***** current dir: {:?}", env::current_dir().unwrap());
    println!("cargo:rustc-link-lib=static=libthreadx");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("writing to {}", out_path.display());

    bindings

        //.write_to_file(out_path.join("bindings.rs"))
        .write_to_file("./bindings.rs")
        .expect("Couldn't write bindings!");
}
