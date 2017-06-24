extern crate gcc;
extern crate bindgen;
extern crate glob;

use glob::glob;
use std::env;
use std::path::PathBuf;

fn main() {
    let root_inc = format!("{}/{}",
                           env::var("ROOTSYS").expect("ROOT include not found!"),
                           "include");
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
        .include("src/ffi/cpp_src/STEERBase");
    
    // The two AliRoot classes needed to read the data
    let files = glob("src/ffi/cpp_src/STEERBase/*.cxx").unwrap().filter_map(|a| a.ok());
    for file in files {
        cfg.file(file);
    }
    cfg.compile("libSTEERBase.a");

    // libESD: It was necessary to copy `event.h` by hand. YOLO!
    let mut cfg = gcc::Config::new();
    cfg
        .cpp(true) // Switch to C++ library compilation.
        .include(&root_inc)
        .include("src/ffi/cpp_src/ESD")
        .include("src/ffi/cpp_src/STEERBase");

    let files = glob("src/ffi/cpp_src/ESD/*.cxx").unwrap().filter_map(|a| a.ok());
    for file in files {
        cfg.file(file);
    }
    cfg.compile("libESD.a");

    let mut cfg = gcc::Config::new();
    cfg
        .cpp(true) // Switch to C++ library compilation.
        .include(&root_inc)
        .include("src/ffi/cpp_src/ESD")
        .include("src/ffi/cpp_src/STEERBase")
        // The file describing the datastructure in the files
        .file("src/ffi/cpp_src/ESD.cxx");

    // ROOT libraries
    let root_lib = "/home/christian/repos/alice/sw/ubuntu1604_x86-64/ROOT/c2208a7e68-1/lib";
    for lib in ["Tree", "Physics", "EG", "Geom", "Minuit", "VMC"].iter() {
        cfg.object(format!("{}/lib{}.so", root_lib, lib));
    }
    cfg.compile("libMyESD.a");

    // A file with the functions for wich we actual want the bindings
    gcc::Config::new()
        .file("src/ffi/lesd-c.cxx")
        .include(&root_inc)
        .include("src/ffi/cpp_src/ESD")
        .include("src/ffi/cpp_src/STEERBase")
        .compile("libalice.a");

    let bindings = bindgen::Builder::default()
        // Do not generate unstable Rust code that
        // requires a nightly rustc and enabling
        // unstable features.
        .no_unstable_rust()
        // The input header we would like to generate
        // bindings for.
        .header("src/ffi/lesd-c.h")
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
