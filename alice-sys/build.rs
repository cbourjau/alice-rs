extern crate gcc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let root_inc = {
        match env::var("ROOTSYS") {
            Ok(root_base) => format!("{}/include", root_base),
            // Fallback (should at least work for Arch)
            Err(_) => "/usr/include/root".to_string()
        }
    };
    let root_lib = {
        match env::var("ROOTSYS") {
            Ok(root_base) => format!("{}/lib", root_base),
            // Fallback (should at least work for Arch)
            Err(_) => "/usr/lib/root".to_string()
        }
    };

    let mut cfg = gcc::Build::new();
    cfg
        .cpp(true) // Switch to C++ library compilation.
        .flag("-std=c++14")
        .include(&root_inc)
        // The auto-generated file for the ESD root tree
        .file("src/ffi/cpp_src/ESD.cxx");

    // ROOT libraries
    let root_libs = vec![
        "Tree",
    ];
    for lib in root_libs.iter() {
        cfg.object(format!("{}/lib{}.so", root_lib, lib));
    }
    cfg.compile("libMyESD.a");

    // Use this extra bit of configuration to avoid the constructor
    // (and idealy the other not-working member functions)
    let mut config = bindgen::CodegenConfig::nothing();
    config.functions = true;
    config.types = true;
    config.vars = true;
    
    let bindings = bindgen::Builder::default()
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++14")
        .clang_arg(format!("-I{}", root_inc))
        .whitelisted_type("ESD")
        // Whitelist esd help functions in that file
        .whitelisted_function("esd_.*")
        .whitelisted_function("tobjarray_.*")
        .whitelisted_function("setup_root")
        .opaque_type(r"T\w+")
        // Do not generate unstable Rust code that
        // requires a nightly rustc and enabling
        // unstable features.
        .unstable_rust(false)
        // Don't generate comments
        .generate_comments(false)
        // The input header we would like to generate
        // bindings for.
        .header("src/ffi/cpp_src/ESDmerged.h")
        .raw_line("#[allow(non_camel_case_types)]")
        .raw_line("#[allow(non_snake_case)]")
        .with_codegen_config(config)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
