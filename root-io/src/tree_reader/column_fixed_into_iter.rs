use failure::Error;
use nom::*;

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
    /// Containers holding the data
    containers: Box<Iterator<Item=T>>,
}

impl<T> ColumnFixedIntoIter<T> {
    pub fn new<P>(tr: &Tree, name: &str, p: P) -> Result<ColumnFixedIntoIter<T>, Error>
    where P: 'static + Fn(&[u8]) -> IResult<&[u8], T>,
          T: 'static
    {
        let br: &TBranch = tr.branches().iter()
            .find(|b| b.name == name)
            .ok_or_else(|| format_err!("Branch {} not found in tree: \n {:#?}",
                                       name,
                                       tr.branches().iter()
                                       .map(|b| b.name.to_owned()).collect::<Vec<_>>())
            )?;
        let containers = Box::new(
            br.containers().to_owned().into_iter()
                // Read and decompress data into a vec
                .flat_map(|c| c.raw_data())
                .flat_map(move |(n_entries, raw_slice)| {
                    let s: &[u8] = raw_slice.as_slice();
                    match count!(s, p, n_entries as usize) {
                        Ok((_, o)) => o,
                        _ => panic!("Parser failed unexpectedly!"),
                    }
                }));
        Ok(ColumnFixedIntoIter {
            containers: containers,
        })
    }
}

impl<T> Iterator for ColumnFixedIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.containers.next()
    }
}
