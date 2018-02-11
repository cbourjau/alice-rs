use std::path::PathBuf;
use failure::Error;
use nom::*;

use tree_reader::{ColumnFixedIntoIter, ColumnVarIntoIter, Tree};
use core::types::ClassInfo;
use core::checked_byte_count;
use core::parsers::{tobjarray_no_context, tnamed};
use RootFile;

struct SchemaIntoIter {
    aliesdrun_frunnumber: ColumnFixedIntoIter<i32>,
    aliesdrun_ftriggerclasses: ColumnFixedIntoIter<Vec<String>>,
    aliesdheader_ftriggermask: ColumnFixedIntoIter<u64>,
    primaryvertex_alivertex_fposition: ColumnFixedIntoIter<[f32; 3]>,
    primaryvertex_alivertex_fncontributors: ColumnFixedIntoIter<i32>,
    tracks_fx: ColumnVarIntoIter<f32>,
    tracks_fp: ColumnVarIntoIter<[f32; 5]>,
    tracks_falpha: ColumnVarIntoIter<f32>,
    tracks_fflags: ColumnVarIntoIter<u64>,
    tracks_fitschi2: ColumnVarIntoIter<f32>,
    tracks_fitsncls: ColumnVarIntoIter<i8>,
    tracks_fitsclustermap: ColumnVarIntoIter<u8>,
}

impl SchemaIntoIter {
    fn new(t: &Tree) -> Result<SchemaIntoIter, Error> {
        let track_counter: Vec<_> = ColumnFixedIntoIter::new(&t, "Tracks", be_u32)?.collect();
        Ok(SchemaIntoIter {
            aliesdrun_frunnumber: ColumnFixedIntoIter::new(&t, "AliESDRun.fRunNumber", be_i32)?,
            aliesdrun_ftriggerclasses: ColumnFixedIntoIter::new(&t, "AliESDRun.fTriggerClasses", parse_trigger_classes)?,
            aliesdheader_ftriggermask: ColumnFixedIntoIter::new(&t, "AliESDHeader.fTriggerMask", be_u64)?,
            primaryvertex_alivertex_fposition:
            ColumnFixedIntoIter::new(&t, "PrimaryVertex.AliVertex.fPosition[3]", |i| count_fixed!(i, f32, be_f32, 3))?,
            primaryvertex_alivertex_fncontributors:
            ColumnFixedIntoIter::new(&t, "PrimaryVertex.AliVertex.fNContributors", be_i32)?,
            tracks_fx: ColumnVarIntoIter::new(&t, "Tracks.fX", be_f32, &track_counter)?,
            tracks_fp: ColumnVarIntoIter::new(&t, "Tracks.fP[5]", |i| count_fixed!(i, f32, be_f32, 5), &track_counter)?,
            tracks_falpha: ColumnVarIntoIter::new(&t, "Tracks.fAlpha", be_f32, &track_counter)?,
            tracks_fflags: ColumnVarIntoIter::new(&t, "Tracks.fFlags", be_u64, &track_counter)?,
            tracks_fitschi2: ColumnVarIntoIter::new(&t, "Tracks.fITSchi2", parse_its_chi2, &track_counter)?,
            tracks_fitsncls: ColumnVarIntoIter::new(&t, "Tracks.fITSncls", be_i8, &track_counter)?,
            tracks_fitsclustermap: ColumnVarIntoIter::new(&t, "Tracks.fITSClusterMap", be_u8, &track_counter)?,
        })
    }
}

/// A model for the / a subset of the ESD data
#[derive(Debug)]
struct Model {
    primaryvertex_alivertex_fposition: [f32; 3],
    primaryvertex_alivertex_fncontributors: i32,
    aliesdrun_frunnumber: i32,
    aliesdrun_ftriggerclasses: Vec<String>,
    aliesdheader_ftriggermask: u64, 
    tracks_fx: Vec<f32>,
    tracks_fp: Vec<[f32; 5]>,
    tracks_falpha: Vec<f32>,
    tracks_fflags: Vec<u64>,
    tracks_fitschi2: Vec<f32>,
    tracks_fitsncls: Vec<i8>,
    tracks_fitsclustermap: Vec<u8>,
}

/// Iterator over models from the schema
impl Iterator for SchemaIntoIter {
    type Item = Model;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Model {
            primaryvertex_alivertex_fposition: self.primaryvertex_alivertex_fposition.next()?,
            primaryvertex_alivertex_fncontributors: self.primaryvertex_alivertex_fncontributors.next()?,
            aliesdrun_frunnumber: self.aliesdrun_frunnumber.next()?,
            aliesdrun_ftriggerclasses: self.aliesdrun_ftriggerclasses.next()?,
            aliesdheader_ftriggermask: self.aliesdheader_ftriggermask.next()?,
            tracks_fx: self.tracks_fx.next()?,
            tracks_fp: self.tracks_fp.next()?,
            tracks_falpha: self.tracks_falpha.next()?,
            tracks_fflags: self.tracks_fflags.next()?,
            tracks_fitschi2: self.tracks_fitschi2.next()?,
            tracks_fitsncls: self.tracks_fitsncls.next()?,
            tracks_fitsclustermap: self.tracks_fitsclustermap.next()?,
        })
    }
}

/// ESD trigger classes are strings describing a particular
/// Trigger. Each event (but in reality every run) might have a
/// different "menu" of available triggers. The trigger menu is saved
/// as an `TObjArray` of `TNamed` objects for each event. This breaks
/// it down to a simple vector
fn parse_trigger_classes(input: &[u8]) -> IResult<&[u8], Vec<String>> {
    let vals = length_value!(input, checked_byte_count, tobjarray_no_context);
    vals.map(|arr| {
        arr.iter()
            .map(|&(ref ci, ref el)| {
                match *ci {
                    ClassInfo::References(0) => "".to_string(),
                    _ => {
                        match tnamed(el.as_slice()).map(|tn| tn.name) {
                            IResult::Done(_, n) => n,
                            _ => panic!()
                        }
                    }
                }
            })
            .collect::<Vec<String>>()
    })
}

/// Some Double_32 values are saved with a custom mantissa... The
/// number of bytes can be found in the comment string of the filed
/// (check the YAML code for ALIESD)
/// This function reconstructs a float from the exponent and mantissa
/// TODO: Use ByteOrder crate to be cross-platform!
fn parse_its_chi2(input: &[u8]) -> IResult<&[u8], f32> {
    pair!(input, be_u8, be_u16).map(|(exp, man)| {
        let nbits = 8;
        let mut s = exp as u32;
        // Move the exponent into the last 23 bits
        s <<= 23;
        s |= (man as u32 & ((1<<(nbits+1))-1)) <<(23-nbits);
        f32::from_bits(s)
    })
}
    

#[test]
fn read_esd() {
    let path = PathBuf::from("./src/test_data/AliESDs.root");
    let f = RootFile::new_from_file(&path).expect("Failed to open file");
    let t = f.items()[0].as_tree().unwrap();
    let schema_iter = match SchemaIntoIter::new(&t) {
        Ok(s) => s,
        Err(err) => panic!("An error occured! Message: {}", err)
    };

    println!("{:?}", schema_iter
             .flat_map(|m| m.tracks_fitschi2.into_iter())
             .fold(0.0, |max, chi2| if chi2 > max {chi2} else {max}));

    let schema_iter = match SchemaIntoIter::new(&t) {
        Ok(s) => s,
        Err(err) => panic!("An error occured! Message: {}", err)
    };
    println!("{:?}", schema_iter
             .flat_map(|m| m.tracks_fitschi2.into_iter())
             .fold(0.0, |max, chi2| if chi2 > max {chi2} else {max}));
}
