extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if env::var("CARGO_FEATURE_NATIVE").is_ok() {
        // Tell cargo to tell rustc to link the system openpnp-capture shared
        // library and its dependencies
        println!("cargo:rustc-link-lib=openpnp-capture");

        if target_os == "linux" {
            println!("cargo:rustc-link-lib=turbojpeg");
        }
    } else if env::var("CARGO_FEATURE_VENDOR").is_ok() {
        // Compile the included library distribution
        let out = cmake::build("vendor");

        // Tell cargo to link the static library
        println!(
            "cargo:rustc-link-search=native={}",
            out.join("lib").display()
        );
        println!(
            "cargo:rustc-link-search=native={}",
            out.join("lib64").display()
        );
        println!("cargo:rustc-link-lib=static=openpnp-capture");

        if target_os == "linux" {
            // We built a C++ library, tell Rust to link the C++ stdlib
            println!("cargo:rustc-flags=-l dylib=stdc++");

            println!("cargo:rustc-link-lib=static=turbojpeg");
        }

        if target_os == "macos" {
            // We built a C++ library, tell Rust to link the C++ stdlib
            println!("cargo:rustc-flags=-lc++");

            println!("cargo:rustc-link-lib=framework=AVFoundation");
            println!("cargo:rustc-link-lib=framework=Foundation");
            println!("cargo:rustc-link-lib=framework=CoreMedia");
            println!("cargo:rustc-link-lib=framework=CoreVideo");
            println!("cargo:rustc-link-lib=framework=Accelerate");
            println!("cargo:rustc-link-lib=framework=IOKit");
        }
    }

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .clang_arg("--include-directory=vendor/include")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
