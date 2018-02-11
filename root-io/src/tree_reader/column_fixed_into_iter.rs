use std::thread;
use failure::Error;
use nom::*;

use crossbeam_channel::Receiver;
use crossbeam_channel::bounded;

use tree_reader::tree::Tree;
use tree_reader::branch::TBranch;

/// Iterator over the data of a column (`TBranch`) with a single element per entry
/// # Example
/// ```
/// extern crate failure;
/// extern crate nom;
/// extern crate root_io;
///
/// use std::path::PathBuf;
/// use failure::Error;
/// use nom::{be_i32, be_f32};
///
/// use root_io::tree_reader::{ColumnFixedIntoIter, Tree};
/// use root_io::core::parsers::{string};
/// use root_io::RootFile;
///
/// /// A model for the (or a subset) of the data.
/// /// This is the object which contains the data of one "event"
/// #[derive(Debug)]
/// struct Model {
///     one: i32,
///     two: f32,
///     three: String,
/// }
///
/// /// Struct holding all the iterators in one place needed for an
/// /// analysis in one place. This makes it much harder to get them out
/// /// of sync
/// struct SchemaIter {
///     one: ColumnFixedIntoIter<i32>,
///     two: ColumnFixedIntoIter<f32>,
///     three: ColumnFixedIntoIter<String>,
/// }
///
/// /// Initiate a new iterator by passing it the `Tree` which contains the data
/// impl SchemaIter {
///     fn new(t: Tree) -> Result<SchemaIter, Error> {
///         Ok(SchemaIter {
///             // Initialize each column; they are identified by name and
///             // a `nom`-like parser is needed to parse the
///             // data. ::core::parsers contains many more parsers for
///             // common ROOT types
///             one: ColumnFixedIntoIter::new(&t, "one", be_i32)?,
///             two: ColumnFixedIntoIter::new(&t, "two", be_f32)?,
///             three: ColumnFixedIntoIter::new(&t, "three", string)?,
///         })
///     }
/// }
///
/// /// Iterator popping out `Model`s. Each model is one "event"
/// impl Iterator for SchemaIter {
///     type Item = Model;
///     fn next(&mut self) -> Option<Self::Item> {
///         Some(Model {
///             one: self.one.next()?,
///             two: self.two.next()?,
///             three: self.three.next()?
///         })
///     }
/// }
///
/// fn main() {
///     let path = PathBuf::from("./src/test_data/simple.root");
///     let f = RootFile::new_from_file(&path).expect("Failed to open file");
///     let t = f.items()[0].as_tree().unwrap();
///     let schema = SchemaIter::new(t).unwrap();
///     for m in schema.into_iter() {
///         println!("{:?}", m);
///     }
/// }
/// ```
pub struct ColumnFixedIntoIter<T> {
    /// Parser for one element (note: There are many elements in one entry!)
    item_parser: Box<Fn(&[u8]) -> IResult<&[u8], T>>,    
    /// Reciever which gets the chunks of read data from a different thread
    chunk_rx: Receiver<Result<Vec<u8>, Error>>,
    /// Content of the latest read container
    current_chunk: Vec<u8>,
}

impl<T> ColumnFixedIntoIter<T> {
    pub fn new<P>(t: &Tree, name: &str, p: P) -> Result<ColumnFixedIntoIter<T>, Error>
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
                match tx.send(c.into_vec()) {
                    Ok(()) => {}
                    Err(e) => {println!("Send error happened for {} on iteration {}  in file {:?}: \n {}",
                                        name, i,
                                        fname,
                                        e.0.expect("IO error?!").as_slice().to_hex(16));
                               break;
                    }
                };
            }
        });
        Ok(ColumnFixedIntoIter {
            item_parser: Box::new(p),
            chunk_rx: rx,
            current_chunk: vec![],
        })
    }
}

impl<T> Iterator for ColumnFixedIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // Refil chunks (we parse from this chunk until its empty)
        if self.current_chunk.is_empty() {
            if let Ok(buf) = self.chunk_rx.recv() {
                self.current_chunk =
                    buf.expect("Could not read data container from disk");
            } else {
                // The above errored if the channel was closed (due to being done)
                return None;
            }
        }
        let (chunk, ret) = {
            let s = self.current_chunk.as_slice();
            let parse_res = (self.item_parser)(s);
            if let IResult::Done(s, ret) = parse_res {
                (s.to_vec(), ret)
            } else {
                panic!("Error applying parser!");
            }
        };
        self.current_chunk = chunk;
        Some(ret)
    }
}
