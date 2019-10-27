use failure::Error;
use nom::*;

use core::checked_byte_count;
use core::parsers::{tnamed, tobjarray_no_context};
use core::types::ClassInfo;
use tree_reader::Tree;
use RootFile;

struct SchemaIntoIter {
    aliesdrun_frunnumber: Box<dyn Iterator<Item = i32>>,
    aliesdrun_ftriggerclasses: Box<dyn Iterator<Item = Vec<String>>>,
    aliesdheader_ftriggermask: Box<dyn Iterator<Item = u64>>,
    primaryvertex_alivertex_fposition: Box<dyn Iterator<Item = [f32; 3]>>,
    primaryvertex_alivertex_fncontributors: Box<dyn Iterator<Item = i32>>,
    tracks_fx: Box<dyn Iterator<Item = Vec<f32>>>,
    tracks_fp: Box<dyn Iterator<Item = Vec<[f32; 5]>>>,
    tracks_falpha: Box<dyn Iterator<Item = Vec<f32>>>,
    tracks_fflags: Box<dyn Iterator<Item = Vec<u64>>>,
    tracks_fitschi2: Box<dyn Iterator<Item = Vec<f32>>>,
    tracks_fitsncls: Box<dyn Iterator<Item = Vec<i8>>>,
    tracks_fitsclustermap: Box<dyn Iterator<Item = Vec<u8>>>,
}

impl SchemaIntoIter {
    fn new(t: &Tree) -> Result<SchemaIntoIter, Error> {
        let track_counter: Vec<_> = t
            .branch_by_name("Tracks")?
            .as_fixed_size_iterator(be_u32)?
            .collect();
        Ok(SchemaIntoIter {
            aliesdrun_frunnumber: Box::new(
                t.branch_by_name("AliESDRun.fRunNumber")?
                    .as_fixed_size_iterator(be_i32)?,
            ),
            aliesdrun_ftriggerclasses: Box::new(
                t.branch_by_name("AliESDRun.fTriggerClasses")?
                    .as_fixed_size_iterator(parse_trigger_classes)?,
            ),
            aliesdheader_ftriggermask: Box::new(
                t.branch_by_name("AliESDHeader.fTriggerMask")?
                    .as_fixed_size_iterator(be_u64)?,
            ),
            primaryvertex_alivertex_fposition: Box::new(
                t.branch_by_name("PrimaryVertex.AliVertex.fPosition[3]")?
                    .as_fixed_size_iterator(|i| count_fixed!(i, f32, be_f32, 3))?,
            ),
            primaryvertex_alivertex_fncontributors: Box::new(
                t.branch_by_name("PrimaryVertex.AliVertex.fNContributors")?
                    .as_fixed_size_iterator(be_i32)?,
            ),
            tracks_fx: Box::new(
                t.branch_by_name("Tracks.fX")?
                    .as_var_size_iterator(be_f32, &track_counter)?,
            ),
            tracks_fp: Box::new(
                t.branch_by_name("Tracks.fP[5]")?
                    .as_var_size_iterator(|i| count_fixed!(i, f32, be_f32, 5), &track_counter)?,
            ),
            tracks_falpha: Box::new(
                t.branch_by_name("Tracks.fAlpha")?
                    .as_var_size_iterator(be_f32, &track_counter)?,
            ),
            tracks_fflags: Box::new(
                t.branch_by_name("Tracks.fFlags")?
                    .as_var_size_iterator(be_u64, &track_counter)?,
            ),
            tracks_fitschi2: Box::new(
                t.branch_by_name("Tracks.fITSchi2")?
                    .as_var_size_iterator(parse_its_chi2, &track_counter)?,
            ),
            tracks_fitsncls: Box::new(
                t.branch_by_name("Tracks.fITSncls")?
                    .as_var_size_iterator(be_i8, &track_counter)?,
            ),
            tracks_fitsclustermap: Box::new(
                t.branch_by_name("Tracks.fITSClusterMap")?
                    .as_var_size_iterator(be_u8, &track_counter)?,
            ),
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
            primaryvertex_alivertex_fncontributors: self
                .primaryvertex_alivertex_fncontributors
                .next()?,
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
            .map(|&(ref ci, ref el)| match *ci {
                ClassInfo::References(0) => "".to_string(),
                _ => match tnamed(el.as_slice()).map(|tn| tn.name) {
                    IResult::Done(_, n) => n,
                    _ => panic!(),
                },
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
        s |= (man as u32 & ((1 << (nbits + 1)) - 1)) << (23 - nbits);
        f32::from_bits(s)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_esd() {
        use alice_open_data;
        let path = alice_open_data::test_file().unwrap();
        let files = [
            // There is an issue on MacOs with opening the ESD test files
            #[cfg(not(target_os="macos"))]
            RootFile::new_from_file(&path).expect("Failed to open file"),
            RootFile::new_from_url(
                "https://eospublichttp.cern.ch/eos/opendata/alice/2010/LHC10h/000139038/ESD/0001/AliESDs.root"
            ).expect("Failed to open file"),
        ];
        for f in &files {
            let t = f.items()[0].as_tree().unwrap();
            test_branch_iterators(&t);
        }
    }

    fn test_branch_iterators(tree: &Tree) {
        let schema_iter = SchemaIntoIter::new(tree).unwrap();
        assert_eq!(schema_iter.aliesdrun_frunnumber.count(), 4);

        let schema_iter = SchemaIntoIter::new(tree).unwrap();
        assert_eq!(schema_iter.aliesdrun_frunnumber.sum::<i32>(), 556152);
        assert_eq!(schema_iter.aliesdheader_ftriggermask.sum::<u64>(), 98);
        assert_eq!(
            schema_iter
                .primaryvertex_alivertex_fncontributors
                .sum::<i32>(),
            2746
        );
        assert_eq!(
            schema_iter.tracks_fx.flat_map(|i| i).sum::<f32>(),
            -26.986227
        );
        assert_eq!(
            schema_iter.tracks_falpha.flat_map(|i| i).sum::<f32>(),
            -199.63356
        );
        assert_eq!(
            schema_iter.tracks_fflags.flat_map(|i| i).sum::<u64>(),
            25876766546549
        );
        assert_eq!(
            schema_iter.tracks_fitschi2.flat_map(|i| i).sum::<f32>(),
            376158.6
        );
        assert_eq!(
            schema_iter
                .tracks_fitsncls
                .flat_map(|i| i)
            // Avoid an error due to overflow
                .map(|ncls| ncls as i64)
                .sum::<i64>(),
            24783
        );
        assert_eq!(
            schema_iter
                .tracks_fitsclustermap
                .flat_map(|i| i)
            // Avoid an error due to overflow
                .map(|ncls| ncls as u64)
                .sum::<u64>(),
            293099
        );
        assert_eq!(
            schema_iter
                .primaryvertex_alivertex_fposition
                .fold([0.0, 0.0, 0.0], |acc, el| {
                    [acc[0] + el[0], acc[1] + el[1], acc[2] + el[2]]
                }),
            [-0.006383737, 0.3380862, 2.938151]
        );
        assert_eq!(
            schema_iter
                .tracks_fp
                .flat_map(|i| i)
                .fold(0.0, |acc, el| { acc + el.iter().sum::<f32>() }),
            39584.777
        );
        // Just add up all the chars in the strings
        assert_eq!(
            schema_iter
                .aliesdrun_ftriggerclasses
                .flat_map(|s| s)
                .map(|s| { s.chars().map(|c| c as u64).sum::<u64>() })
                .sum::<u64>(),
            109268
        );

        let schema_iter = match SchemaIntoIter::new(&tree) {
            Ok(s) => s,
            Err(err) => panic!("An error occured! Message: {}", err),
        };
        println!(
            "{:?}",
            schema_iter
                .flat_map(|m| m.tracks_fitschi2.into_iter())
                .fold(0.0, |max, chi2| if chi2 > max { chi2 } else { max })
        );
    }
}
