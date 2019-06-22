extern crate root_io;
#[macro_use]
extern crate clap;
extern crate rustfmt;
#[macro_use]
extern crate failure;

use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Write};
use clap::{Arg, ArgMatches, App, AppSettings, SubCommand};
use failure::Error;
use root_io::RootFile;

fn main() {
    let matches = App::new("Inspect root files")
        .version(crate_version!())
        .arg(
            Arg::with_name("INPUT")
                .help("Input .root file")
                .required(true)
                .index(1),
        )
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("inspect")
                .about("Dump infromartion about the objects in this file")
                .args_from_usage(
                    "--item-pos=[POS] 'Limit output to item at `pos`'
                         -v 'Verbose output'",
                ),
        )
        .subcommand(
            SubCommand::with_name("to-yaml").about("Output the StreamerInfo of this file as YAML"), // .arg_from_usage("<OUTPUT> 'Output is written to this file'")
        )
        .subcommand(
            SubCommand::with_name("to-rust")
                .about("Generate Rust structs and parsers form the StreamerInfo")
                .args_from_usage(
                    "--output=[OUTPUT] 'Output is written to this file'
                         --rustfmt 'Format the output with `Rustfmt` (slow!)'",
                ),
        )
        .get_matches();
    let in_path = PathBuf::from(matches.value_of("INPUT").unwrap());
    let f = root_io::RootFile::new_from_file(&in_path).expect("Failed to open file");

    if let Some(matches) = matches.subcommand_matches("inspect") {
        inspect_file(&f, matches);
    } else if matches.subcommand_matches("to-yaml").is_some() {
        sinfo_to_yaml(&f);
    } else if let Some(matches) = matches.subcommand_matches("to-rust") {
        to_rust(&f, matches).unwrap();
    } else {
        // Write help if no sub command is given
        println!("{}", matches.usage());
    }
}

fn inspect_file(f: &RootFile, sub_matches: &ArgMatches) {
    if sub_matches.is_present("item-pos") {
        let idx = value_t!(sub_matches.value_of("item-pos"), usize).unwrap();
        // FIXME: This should not be specific for TTrees!
        let tree = f.items()[idx].as_tree().unwrap();
        if sub_matches.is_present("v") {
            println!("{:#?}", tree);
        } else {
            for &(ref name, ref types) in &tree.branch_names_and_types() {
                println!("{}: {:#?}", name, types);
            }
        }
    } else {
        println!("Items in file:");
        for (i, item) in f.items().iter().enumerate() {
            if sub_matches.is_present("v") {
                println!("{}: {}", i, item.verbose_info());
            } else {
                println!("{}: {}", i, item.name());
            }
        }
    }
}

fn sinfo_to_yaml(f: &RootFile) {
    let mut s = String::new();
    match f.streamer_info_as_yaml(&mut s) {
        Ok(_) => println!("{}", s),
        Err(e) => println!("Failed to create yaml. Error: {:?}", e),
    }
}

fn to_rust(f: &RootFile, sub_matches: &ArgMatches) -> Result<(), Error> {
    let mut s = String::new();
    f.streamer_info_as_rust(&mut s)?;
    if sub_matches.is_present("rustfmt") {
        let mut path = env::temp_dir();
        path.push("root2rust.rs");
        {
            let mut f = File::create(&path)?;
            f.write_all(s.as_bytes())?;
            let config = rustfmt::config::Config::default();
            rustfmt::config::Config::default();
            let summary = rustfmt::run(rustfmt::Input::File(path.clone()), &config);
            if !summary.has_no_errors() {
                return Err(format_err!("Error formating source code: {:?}", summary));
            }
        }
        // reopen the file and read the content to a string
        let mut f = File::open(&path)?;
        s = String::new();
        f.read_to_string(&mut s)?;
    }
    println!("{}", s);
    Ok(())
}
