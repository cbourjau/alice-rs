use std::fmt;
// use std::thread;
use failure::Error;
use nom::*;

// use crossbeam_channel::Receiver;
// use crossbeam_channel::bounded;

use tree_reader::tree::Tree;
use tree_reader::branch::TBranch;


/// Iterator over the data of a column (`TBranch`) with a variable
/// number of elements per entry.  See the file
/// [`read_esd.rs`](https://github.com/cbourjau/root-io/blob/master/src/tests/read_esd.rs)
/// in the repository for a comprehensive example
pub struct ColumnVarIntoIter<T> {
    /// Number of elements in each entry
    elems_per_entry: ::std::vec::IntoIter<u32>,
    containers: Box<Iterator<Item=T>>,
}

impl<T> ColumnVarIntoIter<T> {
    /// Create a new iterator over the branch `name` in the given
    /// `Tree`
    pub fn new<P>(tr: &Tree, name: &str, p: P, el_counter: &[u32]) -> Result<ColumnVarIntoIter<T>, Error>
    where P: 'static + Fn(&[u8]) -> IResult<&[u8], T>,
          T: 'static + ::std::fmt::Debug
    {
        // The `N`th entry is parsed by applying the parser `p`
        // `el_counter[N]` times. TODO: This is not what currently
        // happens! Currently, I'm parsing all _elements_ of a basket,
        // ignoring the size of each event. I then chunk the numer of
        // elements into the size of the entry in the `next` function        
        let br: &TBranch = tr.branches().iter()
            .find(|b| b.name == name)
            .ok_or_else(|| format_err!("Branch {} not found in tree: \n {:#?}",
                                       name,
                                       tr.branches().iter()
                                       .map(|b| b.name.to_owned()).collect::<Vec<_>>())
            )?;
        let mut n_elems_per_event = el_counter.iter();
        let n_elems_per_basket: Vec<u32> =
            br.n_events_per_basket()
            .into_iter()
            .map(|nevts_this_bskt| {
                (0..nevts_this_bskt)
                    .map(|_i_evt_this_bskt| *n_elems_per_event.next().unwrap())
                    .sum()
            })
            .collect();
        let containers = Box::new(
            br.containers().to_owned().into_iter()
                // Read and decompress data into a vec
                .flat_map(|c| c.raw_data())
                .zip(n_elems_per_basket.into_iter())
                .flat_map(move |((n_entries_in_buf, raw_slice), n_elems)| {
                    let s: &[u8] = raw_slice.as_slice();
                    match count!(s, p, n_elems as usize) {
                        IResult::Done(_, o) => o,
                        _ => panic!("Parser failed unexpectedly! {}, {}", n_entries_in_buf, s.len()),
                    }})
        );

        Ok(ColumnVarIntoIter {
            elems_per_entry: el_counter.to_vec().into_iter(),
            containers: containers,
        })
    }
}

impl<T: fmt::Debug + Clone> Iterator for ColumnVarIntoIter<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n_elems) = self.elems_per_entry.next() {
            // `take` consumses self, thus this uglyness
            let ret = (0..n_elems)
                .map(|_| self.containers.next().unwrap())
                .collect();
            Some(ret)
        } else {
            None
        }
    }
}
