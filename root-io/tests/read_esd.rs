#![cfg(test)]

use failure::Error;
use futures::prelude::*;
use nom::number::complete::*;
use nom::sequence::tuple;

use root_io::{
    core::parsers::{parse_custom_mantissa, parse_tobjarray_of_tnameds},
    stream_zip,
    tree_reader::Tree,
    RootFile,
};

/// A model for the / a subset of the ESD data
#[derive(Debug)]
struct Model {
    primaryvertex_alivertex_fposition: (f32, f32, f32),
    primaryvertex_alivertex_fncontributors: i32,
    aliesdrun_frunnumber: i32,
    aliesdrun_ftriggerclasses: Vec<String>,
    aliesdheader_ftriggermask: u64,
    tracks_fx: Vec<f32>,
    tracks_fp: Vec<(f32, f32, f32, f32, f32)>,
    tracks_falpha: Vec<f32>,
    tracks_fflags: Vec<u64>,
    tracks_fitschi2: Vec<f32>,
    tracks_fitsncls: Vec<i8>,
    tracks_fitsclustermap: Vec<u8>,
    tracks_ftpcncls: Vec<u16>,
    tracks_ftpcchi2: Vec<f32>,
}

impl Model {
    async fn stream_from_tree(t: &Tree) -> Result<impl Stream<Item = Self>, Error> {
        let track_counter: Vec<_> = t
            .branch_by_name("Tracks")?
            .as_fixed_size_iterator(|i| be_u32(i))
            .collect::<Vec<_>>()
            .await;
        let s = stream_zip!(
            t.branch_by_name("AliESDRun.fRunNumber")?
                .as_fixed_size_iterator(|i| be_i32(i)),
            t.branch_by_name("AliESDRun.fTriggerClasses")?
                .as_fixed_size_iterator(parse_tobjarray_of_tnameds),
            t.branch_by_name("AliESDHeader.fTriggerMask")?
                .as_fixed_size_iterator(|i| be_u64(i)),
            t.branch_by_name("PrimaryVertex.AliVertex.fPosition[3]")?
                .as_fixed_size_iterator(|i| tuple((be_f32, be_f32, be_f32))(i)),
            t.branch_by_name("PrimaryVertex.AliVertex.fNContributors")?
                .as_fixed_size_iterator(|i| be_i32(i)),
            t.branch_by_name("Tracks.fX")?
                .as_var_size_iterator(|i| be_f32(i), &track_counter),
            t.branch_by_name("Tracks.fP[5]")?.as_var_size_iterator(
                |i| tuple((be_f32, be_f32, be_f32, be_f32, be_f32))(i),
                &track_counter
            ),
            t.branch_by_name("Tracks.fAlpha")?
                .as_var_size_iterator(|i| be_f32(i), &track_counter),
            t.branch_by_name("Tracks.fFlags")?
                .as_var_size_iterator(|i| be_u64(i), &track_counter),
            t.branch_by_name("Tracks.fITSchi2")?
                .as_var_size_iterator(|i| parse_custom_mantissa(i, 8), &track_counter),
            t.branch_by_name("Tracks.fITSncls")?
                .as_var_size_iterator(|i| be_i8(i), &track_counter),
            t.branch_by_name("Tracks.fITSClusterMap")?
                .as_var_size_iterator(|i| be_u8(i), &track_counter),
            t.branch_by_name("Tracks.fTPCncls")?
                .as_var_size_iterator(|i| be_u16(i), &track_counter),
            t.branch_by_name("Tracks.fTPCchi2")?
                .as_var_size_iterator(|i| parse_custom_mantissa(i, 8), &track_counter),
        )
        .map(
            |(
                aliesdrun_frunnumber,
                aliesdrun_ftriggerclasses,
                aliesdheader_ftriggermask,
                primaryvertex_alivertex_fposition,
                primaryvertex_alivertex_fncontributors,
                tracks_fx,
                tracks_fp,
                tracks_falpha,
                tracks_fflags,
                tracks_fitschi2,
                tracks_fitsncls,
                tracks_fitsclustermap,
                tracks_ftpcncls,
                tracks_ftpcchi2,
            )| {
                Self {
                    aliesdrun_frunnumber,
                    aliesdrun_ftriggerclasses,
                    aliesdheader_ftriggermask,
                    primaryvertex_alivertex_fposition,
                    primaryvertex_alivertex_fncontributors,
                    tracks_fx,
                    tracks_fp,
                    tracks_falpha,
                    tracks_fflags,
                    tracks_fitschi2,
                    tracks_fitsncls,
                    tracks_fitsclustermap,
                    tracks_ftpcchi2,
                    tracks_ftpcncls,
                }
            },
        );
        Ok(s)
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    use reqwest::Url;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test(async)]
    async fn read_esd_wasm() {
        let files = [
            // There is an issue on MacOs with opening the ESD test files
            RootFile::new(
                // "http://opendata.cern.ch/eos/opendata/alice/2010/LHC10h/000139038/ESD/0001/AliESDs.root"
                Url::parse("http://cirrocumuli.com/eos/opendata/alice/2010/LHC10h/000139038/ESD/0001/AliESDs.root").unwrap()
            ).await.expect("Failed to open file"),
        ];
        for f in &files {
            let t = f.items()[0].as_tree().await.unwrap();
            test_branch_iterators(&t).await;
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod x64 {
    use super::*;
    use alice_open_data;
    use tokio;
    use reqwest::Url;

    const REMOTE_FILE: &str =
	"http://opendata-dev.web.cern.ch/eos/opendata/alice/2010/LHC10h/000139038/ESD/0001/AliESDs.root";

    #[tokio::test]
    async fn read_esd_local_and_remote() {
        let path = alice_open_data::test_file().unwrap();
        let files = [
            // There is an issue on MacOs with opening the ESD test files
            #[cfg(not(target_os="macos"))]
            RootFile::new(path).await.expect("Failed to open file"),
            RootFile::new(Url::parse(REMOTE_FILE).unwrap()
            ).await.expect("Failed to open file"),
        ];
        for f in &files {
            let t = f.items()[0].as_tree().await.unwrap();
            test_branch_iterators(&t).await;
        }
    }
}

async fn test_branch_iterators(tree: &Tree) {
    let mut schema_iter = Box::pin(Model::stream_from_tree(tree).await.unwrap());

    let mut cnt = 0;
    let mut aliesdrun_frunnumber = 0;
    let mut aliesdheader_ftriggermask = 0;
    let mut primaryvertex_alivertex_fncontributors = 0;
    let mut tracks_fx: Vec<f32> = vec![];
    let mut tracks_falpha: Vec<f32> = vec![];
    let mut tracks_fflags: Vec<u64> = vec![];
    let mut tracks_fitschi2: Vec<f32> = vec![];
    let mut tracks_fitsncls: Vec<i8> = vec![];
    let mut tracks_fitsclustermap: Vec<u8> = vec![];
    let mut primaryvertex_alivertex_fposition: Vec<(f32, f32, f32)> = vec![];
    let mut tracks_fp: Vec<Vec<(f32, f32, f32, f32, f32)>> = vec![];
    let mut aliesdrun_ftriggerclasses: Vec<String> = vec![];
    let mut tracks_ftpcchi2: Vec<f32> = vec![];
    let mut tracks_ftpcncls: Vec<u16> = vec![];

    while let Some(event) = schema_iter.next().await {
        cnt += 1;
        aliesdrun_frunnumber += event.aliesdrun_frunnumber;
        aliesdheader_ftriggermask += event.aliesdheader_ftriggermask;
        primaryvertex_alivertex_fncontributors += event.primaryvertex_alivertex_fncontributors;
        tracks_fx.extend(event.tracks_fx.iter());
        tracks_falpha.extend(event.tracks_falpha.iter());
        tracks_fflags.extend(event.tracks_fflags.iter());
        tracks_fitschi2.extend(event.tracks_fitschi2.iter());
        tracks_fitsncls.extend(event.tracks_fitsncls.iter());
        tracks_fitsclustermap.extend(event.tracks_fitsclustermap.iter());
        tracks_ftpcchi2.extend(event.tracks_ftpcchi2.iter());
        tracks_ftpcncls.extend(event.tracks_ftpcncls.iter());
        primaryvertex_alivertex_fposition.push(event.primaryvertex_alivertex_fposition);
        tracks_fp.push(event.tracks_fp);
        aliesdrun_ftriggerclasses.extend(event.aliesdrun_ftriggerclasses.into_iter());
    }

    assert_eq!(cnt, 4);
    assert_eq!(aliesdrun_frunnumber, 556152);
    assert_eq!(aliesdheader_ftriggermask, 98);
    assert_eq!(primaryvertex_alivertex_fncontributors, 2746);
    assert_eq!(tracks_fx.iter().sum::<f32>(), -26.986227);
    assert_eq!(tracks_falpha.iter().sum::<f32>(), -199.63356);
    assert_eq!(tracks_fflags.iter().sum::<u64>(), 25876766546549);
    assert_eq!(tracks_fitschi2.iter().sum::<f32>(), 376158.6);
    assert_eq!(
        tracks_fitsncls.iter().map(|el| *el as i64).sum::<i64>(),
        24783
    );
    assert_eq!(
        tracks_fitsclustermap
            .iter()
            .map(|el| *el as u64)
            .sum::<u64>(),
        293099
    );
    assert_eq!(tracks_ftpcchi2.iter().sum::<f32>(), 2352277.0);
    assert_eq!(
        tracks_ftpcncls.iter().map(|el| *el as i64).sum::<i64>(),
        984359
    );

    assert_eq!(
        primaryvertex_alivertex_fposition
            .iter()
            .fold([0.0, 0.0, 0.0], |acc, el| {
                [acc[0] + el.0, acc[1] + el.1, acc[2] + el.2]
            }),
        [-0.006383737, 0.3380862, 2.938151]
    );
    assert_eq!(
        tracks_fp.iter().flat_map(|i| i).fold(0.0, |acc, el| {
            acc + [el.0, el.1, el.2, el.3, el.4].iter().sum::<f32>()
        }),
        39584.777
    );

    // Just add up all the chars in the strings
    assert_eq!(
        aliesdrun_ftriggerclasses
            .iter()
            .map(|s| { s.chars().map(|c| c as u64).sum::<u64>() })
            .sum::<u64>(),
        109268
    );
}
