extern crate gcc;
extern crate bindgen;
extern crate glob;

use glob::glob;
use std::env;
use std::path::PathBuf;

fn main() {
    let root_base = env::var("ROOTSYS").expect("ROOT include not found!");
    let root_inc = format!("{}/include", root_base);
    let root_lib = format!("{}/lib", root_base);
    // We need to libs from the AliROOT side: STEERBase and ESD
    // The needed files where extreacted building the respective par
    // files, which are .tar.gz archives with the sources.
    
    // libSTEERBase: ARVersion.h is special. It is created during
    // build time by the make files. Thus, that files was copied by
    // hand from my installation
    let mut cfg = gcc::Config::new();
    cfg
        .cpp(true) // Switch to C++ library compilation.
        .include(&root_inc)
        .include("src/ffi/cpp_src/STEERBase")
        .include("src/ffi/cpp_src/ESD");
    
    // The two AliRoot classes needed to read the data
    let files = glob("src/ffi/cpp_src/STEERBase/*.cxx").unwrap().filter_map(|a| a.ok());
    for file in files {
        cfg.file(file);
    }

    // libESD: It was necessary to copy `event.h` by hand. YOLO!
    let files = glob("src/ffi/cpp_src/ESD/*.cxx").unwrap().filter_map(|a| a.ok());
    for file in files {
        cfg.file(file);
    }
    // The auto-generated file for the ESD root tree
    cfg.file("src/ffi/cpp_src/ESD.cxx");

    // ROOT libraries
    for lib in ["Tree", "Physics", "EG", "Geom", "Minuit", "VMC"].iter() {
        cfg.object(format!("{}/lib{}.so", root_lib, lib));
    }
    cfg.compile("libMyESD.a");

    // A file with the functions for wich we actual want the bindings
    // gcc::Config::new()
    //     .file("src/ffi/cpp_src/ESD.cxx")
    //     .include("src/ffi/cpp_src/ESDmerged.h")
    //     .include(&root_inc)
    //     .include("src/ffi/cpp_src/ESD")
    //     .include("src/ffi/cpp_src/STEERBase")
    //     .compile("libalice.a");

    // let bindings = bindgen::Builder::default()
    //     // Do not generate unstable Rust code that
    //     // requires a nightly rustc and enabling
    //     // unstable features.
    //     .no_unstable_rust()
    //     // The input header we would like to generate
    //     // bindings for.
    //     .header("src/ffi/lesd-c.h")
    //     // Finish the builder and generate the bindings.
    //     .generate()
    //     // Unwrap the Result and panic on failure.
    //     .expect("Unable to generate bindings");

    // // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");

    // let bindings = bindgen::Builder::default()
    //     .clang_arg("-x")
    //     .clang_arg("c++")
    //     .clang_arg("-std=c++11")
    //     .clang_arg("-I/home/christian/repos/alice/sw/ubuntu1604_x86-64/ROOT/latest/include/")
    //     .whitelisted_type("ESD")
    //     .whitelisted_function("esd_new")
    //     .whitelisted_function("esd_load_next")
    //     .whitelisted_function("esd_destroy")
    //     .opaque_type(r"T\w+")
    //     // Do not generate unstable Rust code that
    //     // requires a nightly rustc and enabling
    //     // unstable features.
    //     .unstable_rust(false)
    //     // Don't generate comments
    //     .generate_comments(false)
    //     // The input header we would like to generate
    //     // bindings for.
    //     .header("src/ffi/cpp_src/ESDmerged.h")
    //     .raw_line("#[allow(non_camel_case_types)]")
    //     .raw_line("#[allow(non_snake_case)]")
    //     .derive_debug(true)
    //     // Finish the builder and generate the bindings.
    //     .generate()
    //     // Unwrap the Result and panic on failure.
    //     .expect("Unable to generate bindings");

    // // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");
}
