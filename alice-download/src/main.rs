extern crate indicatif;
#[macro_use]
extern crate clap;
extern crate alice_open_data;
extern crate failure;

use indicatif::{ProgressBar, ProgressStyle};
use clap::{Arg, App};
use alice_open_data::*;

fn main() {
    ::std::process::exit(match do_thing() {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}

fn do_thing() -> Result<(), failure::Error>
{
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
    let max_vol = value_t!(matches.value_of("amount"), u64)?;
    // convert from GB to B
    let max_vol = max_vol * (1_000_000_000);
    let runs = [
        139_038,
        139_173,
        139_437,
        139_438,
        139_465,
    ];

    // size of existing files
    let base_dir = data_dir()?;
    let mut total: u64 = 0;
    for entry in all_files_10h()?.iter() {
        let data = entry.metadata()?;
        if data.is_file() {
            total += data.len();
        }
    }
    if total >= max_vol {
        return Ok(());
    }
    let urls = runs.iter()
        .map(|r| get_file_list(*r))
        .collect::<Result<Vec<_>, _>>()?;
    let pbar = ProgressBar::new(max_vol as u64);
    pbar.set_style(ProgressStyle::default_bar()
                   .template("[{elapsed_precise}] {bar:40.cyan/blue} ETA: {eta}"));
    pbar.inc(total);
    for url in urls.iter().flat_map(|r| r.iter()) {
        if total < max_vol  {
            let size = download(base_dir.clone(), url.clone())?;
            pbar.inc(size);
            total += size;
        } else {
            break;
        }
    }
    Ok(())
}

