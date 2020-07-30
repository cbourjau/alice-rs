extern crate rustfmt as rustfmt_crate;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use clap::{crate_version, value_t, App, AppSettings, Arg, ArgMatches, SubCommand};
use failure::{format_err, Error};
use root_io::RootFile;

#[tokio::main]
async fn main() {
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
    let in_path = Path::new(matches.value_of("INPUT").unwrap());
    let f = root_io::RootFile::new(in_path)
        .await
        .expect("Failed to open file");

    if let Some(matches) = matches.subcommand_matches("inspect") {
        inspect_file(&f, matches).await;
    } else if matches.subcommand_matches("to-yaml").is_some() {
        sinfo_to_yaml(&f).await;
    } else if let Some(matches) = matches.subcommand_matches("to-rust") {
        to_rust(&f, matches).await.unwrap();
    } else {
        // Write help if no sub command is given
        println!("{}", matches.usage());
    }
}

async fn inspect_file(f: &RootFile, sub_matches: &ArgMatches<'_>) {
    if sub_matches.is_present("item-pos") {
        let idx = value_t!(sub_matches.value_of("item-pos"), usize).unwrap();
        // FIXME: This should not be specific for TTrees!
        let tree = f.items()[idx].as_tree().await.unwrap();
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

async fn sinfo_to_yaml(f: &RootFile) {
    let mut s = String::new();
    match f.streamer_info_as_yaml(&mut s).await {
        Ok(_) => println!("{}", s),
        Err(e) => println!("Failed to create yaml. Error: {:?}", e),
    }
}

async fn to_rust(f: &RootFile, sub_matches: &ArgMatches<'_>) -> Result<(), Error> {
    let mut s = String::new();
    f.streamer_info_as_rust(&mut s).await?;
    if sub_matches.is_present("rustfmt") {
        let mut path = env::temp_dir();
        path.push("root2rust.rs");
        {
            let mut f = File::create(&path)?;
            f.write_all(s.as_bytes())?;
            let config = rustfmt_crate::config::Config::default();
            rustfmt_crate::config::Config::default();
            let summary = rustfmt_crate::run(rustfmt::Input::File(path.clone()), &config);
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
