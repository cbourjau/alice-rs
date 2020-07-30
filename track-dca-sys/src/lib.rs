#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[warn(clippy::all)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl AliExternalTrackParam {
    pub fn get_dca(&self, other: &AliExternalTrackParam, mag_field_b: f64) -> (f64, f64, f64) {
	let mut xthis = 0.0;
	let mut xp = 0.0;
	let dca = unsafe {self.GetDCA(other, mag_field_b, &mut xthis, &mut xp) };
	(dca, xthis, xp)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

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

    use assert_approx_eq::assert_approx_eq;

    const REMOTE_FILE: &str =
	// "http://opendata-dev.web.cern.ch/eos/opendata/alice/2010/LHC10h/000139038/ESD/0001/AliESDs.root";
        "http://opendata.cern.ch/eos/opendata/alice/2010/LHC10h/000139038/ESD/0001/AliESDs.root";

    // x(t) = r1 * cos(t)
    // y(t) = r1 * sin(t)
    // z(t) = c*t

    #[test]
    fn test_dca() {
        // Two helix with a 45 degree slope (i.e. tan = 1)
        let b = 1.0;
    	let helix_1 = unsafe {
	    AliExternalTrackParam::new(
                0.0,
                0.0,
                [ 0.0, 1.0, 0.0, 1.0, 1.0, ].as_ptr(),
	    )
        };
    	let helix_45_deg_z_shift = unsafe {
	    AliExternalTrackParam::new(
                1.0,
                0.0,
                [ 0.0, 1.0, 0.0, 1.0, 1.0, ].as_ptr(),
	    )
        };
    	let helix_negative_charge = unsafe {
	    AliExternalTrackParam::new(
                0.0,
                0.0,
                [ 0.0, 1.0, 0.0, 1.0, -1.0, ].as_ptr(),
	    )
        };
	let mut xthis = 0.0;
	let mut xp = 0.0 as f64;
        // parallel with z-shift
        let expected_distance_45_deg = (PI / 4.0).sin() / (PI / 2.0).sin();
        let dca = unsafe {helix_1.GetDCA(&helix_45_deg_z_shift, b, &mut xthis, &mut xp)};
        assert_approx_eq!(dca, expected_distance_45_deg, 1.0e-5);

        // opposite chirality
        let expected_distance = 0.0;
        let dca = unsafe {helix_1.GetDCA(&helix_negative_charge, b, &mut xthis, &mut xp)};
        assert_approx_eq!(dca, expected_distance, 1.0e-5);
    }

    #[test]
    fn test_dca_deadlock() {
        // Two helix with a 45 degree slope (i.e. tan = 1)
        let b = 5.00668049;
    	let helix_1 = unsafe {
	    AliExternalTrackParam::new(
                0.11572355031967163, // x
                2.4826834201812744, // alpha
                [
                    -0.07366123795509338,
                    6.936183452606201,
                    0.000000000706358305180288,
                    0.12432964891195297,
                    4.714923858642578,
                ].as_ptr(),
	    )
        };
    	let helix_2 = unsafe {
	    AliExternalTrackParam::new(
                0.1059807538986206,
                2.553640365600586,
                [
                    -0.13346174359321594,
                    6.964710712432861,
                    -0.0000000004830002509059739,
                    0.25471046566963196,
                    -1.5283981561660767,
                ].as_ptr(),
	    )
        };
        // This produces a dead-lock
        unsafe {helix_1.GetDCA(&helix_2, b, &mut 0.0, &mut 0.0)};
    }

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
