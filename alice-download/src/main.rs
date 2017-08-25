extern crate indicatif;
#[macro_use]
extern crate clap;
extern crate alice_open_data;

use indicatif::ProgressBar;
use clap::{Arg, App};
use alice_open_data::*;


fn main() {
    let matches = App::new("alice-download")
        .version("0.1")
        .about("Download specified amount of ALICE open Pb--Pb data to ~/lhc_open_data \
                Visit `http://opendata.cern.ch/search?cc=ALICE-Reconstructed-Data` for \
                more information."
               )
        .arg(Arg::with_name("amount")
             .help("Download amount specified in GB. Does not re-download existing files. \
                    1GB is enough for simple debugging. Use 50+GB to make meaningful \
                    plots.")
             .required(true)
             .index(1))
        .get_matches();
    let max_vol = value_t!(matches.value_of("amount"), i64).unwrap_or_else(|e| e.exit());
    // convert from GB to B
    let max_vol = max_vol * (1_000_000_000);
    let runs = [
        139_038,
        139_173,
        139_437,
        139_438,
        139_465,
    ];
    let files: Vec<FileDetails> = runs.iter()
        .flat_map(|r| file_details_of_run(*r).unwrap())
        .collect();
    let total_size = files.iter().fold(0, |acc, v| acc + v.file_size) as f64 / 1e9;
    println!("Total available data: {} files with total of {} GB", files.len(), total_size);

    let max_files: Vec<FileDetails> =
        files
        .into_iter()
        .scan(0, |acc, v| {
            if max_vol > *acc {
                *acc += v.file_size;
                Some(v)
            } else {
                None
            }})
        .collect();

    let pbar = ProgressBar::new(max_files.len() as u64);
    let max_files = pbar.wrap_iter(max_files.iter());

    for f in max_files {
        f.download().unwrap();
    }
}

