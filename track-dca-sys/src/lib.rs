#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use root_io::RootFile;
    use alice_open_data;
    use reqwest::Url;
    use tokio;
    use failure::Error;
    use futures::prelude::*;
    use nom::number::complete::*;
    use nom::sequence::tuple;
    use root_io::{
	stream_zip,
	tree_reader::Tree,
    };
    
    const REMOTE_FILE: &str =
	"http://opendata-dev.web.cern.ch/eos/opendata/alice/2010/LHC10h/000139038/ESD/0001/AliESDs.root";

    /// A model for the / a subset of the ESD data
    #[derive(Debug)]
    struct Model {
	tracks_fx: Vec<f32>,
	tracks_fp: Vec<(f32, f32, f32, f32, f32)>,
	tracks_falpha: Vec<f32>,
    }

    impl Model {
	pub async fn stream_from_tree(t: &Tree) -> Result<impl Stream<Item = Self>, Error> {
            let track_counter: Vec<_> = t
		.branch_by_name("Tracks")?
		.as_fixed_size_iterator(|i| be_u32(i))
		.collect::<Vec<_>>()
		.await;
            let s = stream_zip!(
		t.branch_by_name("Tracks.fX")?
                    .as_var_size_iterator(|i| be_f32(i), &track_counter),
		t.branch_by_name("Tracks.fP[5]")?.as_var_size_iterator(
                    |i| tuple((be_f32, be_f32, be_f32, be_f32, be_f32))(i),
                    &track_counter
		),
		t.branch_by_name("Tracks.fAlpha")?
                    .as_var_size_iterator(|i| be_f32(i), &track_counter),
            )
		.map(
		    |(
			tracks_fx,
			tracks_fp,
			tracks_falpha,
		    )| {
			Self {
			    tracks_fx,
			    tracks_fp,
			    tracks_falpha,
			}
		    },
		);
            Ok(s)
	}
    }


    #[tokio::test]
    async fn read_esd_local_and_remote() {
        let path = alice_open_data::test_file().unwrap();
        let files = [
            // There is an issue on MacOs with opening the ESD test files
            #[cfg(not(target_os = "macos"))]
            RootFile::new(path).await.expect("Failed to open file"),
            RootFile::new(Url::parse(REMOTE_FILE).unwrap())
                .await
                .expect("Failed to open file"),
        ];
        for f in &files {
            let t = f.items()[0].as_tree().await.unwrap();
            test_branch_iterators(&t).await;
        }
    }


    async fn test_branch_iterators(tree: &Tree) {
	let mut schema_iter = Box::pin(Model::stream_from_tree(tree).await.unwrap());

	let mut cnt = 0;
	let mut dca_sum = 0.0;
	let mut return0_sum = 0.0;
	let mut return1_sum = 0.0;

	while let Some(event) = schema_iter.next().await {
            cnt += 1;
	    let params: Vec<_> = event.tracks_fx.iter()
		.zip(&event.tracks_fp)
		.zip(&event.tracks_falpha)
		.map(|((x, p), alpha)|
		     unsafe {
			 AliExternalTrackParam::new(
                             *x as f64,
                             *alpha as f64,
                             [p.0 as f64, p.1 as f64, p.2 as f64, p.3 as f64, p.4 as f64].as_ptr(),
			 )
                     })
		.collect();
	    if params.len() < 2 {
		continue;
	    }
	    let mut xthis = 0.0;
	    let mut xp = 0.0;
	    assert_eq!(
		unsafe{
		    params[0].GetDCA(&params[0], 42.0, &mut xthis, &mut xp)
		},
		0.0
	    );
	    
	    for param0 in &params {
		for param1 in &params {
		    
		    unsafe{
			dca_sum += param0.GetDCA(param1, 0.0, &mut xthis, &mut xp);
			return0_sum += xthis;
			return1_sum += xp;
		    }
		}
		break;
	    }
	}
	assert_eq!(cnt, 4);
	assert_eq!(dca_sum, 352250.034292012);
	assert_eq!(return0_sum, -53307.868580856884);
	assert_eq!(return1_sum, 36503.19644310874);
    }
}
