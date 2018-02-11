use std::fmt;
use std::thread;
use failure::Error;
use nom::*;

use crossbeam_channel::Receiver;
use crossbeam_channel::bounded;

use tree_reader::tree::Tree;
use tree_reader::branch::TBranch;


/// Iterator over the data of a column (`TBranch`) with a variable
/// number of elements per entry.  See the file
/// [`read_esd.rs`](https://github.com/cbourjau/root-io/blob/master/src/tests/read_esd.rs)
/// in the repository for a comprehensive example
pub struct ColumnVarIntoIter<T> {
    /// Number of elements in each entry
    elems_per_entry: ::std::vec::IntoIter<u32>,
    /// Parser for one element (note: There are many elements in one entry!)
    item_parser: Box<Fn(&[u8]) -> IResult<&[u8], T>>,    
    // /// Containers holding the data
    // ///containers: ::std::vec::IntoIter<CpuFuture<Vec<u8>, Error>>,
    /// Reciever which gets the chunks of read data from a different thread
    chunk_rx: Receiver<Vec<u8>>,    
    /// Content of the latest read container
    current_chunk: Vec<u8>,
}

impl<T> ColumnVarIntoIter<T> {
    /// Create a new iterator over the branch `name` in the given
    /// `Tree`. The `N`th entry is parsed by applying the parser `p`
    /// `el_counter[N]` times.
    /// **Note: This iterator does some gready io! It will start reading parts of the data immediatly without blocking**
    pub fn new<P>(t: &Tree, name: &str, p: P, el_counter: &[u32]) -> Result<ColumnVarIntoIter<T>, Error>
    where P: 'static + Fn(&[u8]) -> IResult<&[u8], T>
    {
        let b: &TBranch = t.branches().iter()
            .find(|b| b.name == name)
            .ok_or(format_err!("Branch {} not found in tree: \n {:#?}",
                               name,
                               t.branches().iter()
                               .map(|b| b.name.to_owned()).collect::<Vec<_>>())
            )?;
        let containers = b.containers().to_owned();
        let name  = name.to_string();        
        // Create a bounded channel for the read data chunks
        let (tx, rx) = bounded(0);
        thread::spawn(move || {
            for (i, c) in containers.into_iter().enumerate() {
                let fname = c.file();
                let thing = c.into_vec().expect("Could not read/decompress data!");
                let is_empty = thing.is_empty();
                match tx.send(thing) {
                    Ok(()) => {}
                    Err(e) => {
                        // Maybe this slice was empty and the last one? Than we don't care
                        if is_empty {
                            continue;
                        } else {
                            
                            println!("Send error happened for {} on iteration {}  in file {:?}: \n {}",
                                     name, i,
                                     fname,
                                     e.0.as_slice().to_hex(16));
                            break;
                        }
                    }
                };
            }
        });        
        Ok(ColumnVarIntoIter {
            elems_per_entry: el_counter.to_vec().into_iter(),
            item_parser: Box::new(p),
            chunk_rx: rx, //containers.into_iter(),
            current_chunk: vec![],
        })
    }
}

impl<T: fmt::Debug> Iterator for ColumnVarIntoIter<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let elems_this_entry = match self.elems_per_entry.next() {
            Some(val) => val as usize,
            _ => return None,
        };
        // Refil chunks, but only if we expect to read new data on this iteration
        // If we have 0 elements, we hold off with the next chunk!
        if elems_this_entry != 0 && self.current_chunk.is_empty() {
            // Loop until we have a chunk of non 0 size...
            loop {
                if let Ok(buf) = self.chunk_rx.recv() {
                    self.current_chunk = buf;
                } else {
                    // The above errored if the channel was closed (due to being done)
                    return None;
                }
                if !self.current_chunk.is_empty() {
                    break;
                }
            }
        }
        let (chunk, ret) = {
            let s = self.current_chunk.as_slice();
            let parse_res = count!(s, self.item_parser, elems_this_entry);
            match parse_res {
                IResult::Done(s, ret) => (s.to_vec(), ret),
                IResult::Incomplete(_) => {println!("Could not apply parser {} times as expected", elems_this_entry);
                                           panic!("Size of input slice: {}, empty: {}", s.len(), self.current_chunk.is_empty());}
                err => panic!("{:?}", err),
            }
        };
        self.current_chunk = chunk;
        Some(ret)
    }
}
