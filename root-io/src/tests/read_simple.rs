use std::path::PathBuf;
use failure::Error;
use nom::{be_i32, be_f32};

use tree_reader::{ColumnFixedIntoIter, Tree};
use core::parsers::{string};
use RootFile;

/// A model for the (or a subset) of the data.
/// This is the object which contains the data of one "event"
#[derive(Debug)]
struct Model {
    one: i32,
    two: f32,
    three: String,
}


/// Struct holding all the iterators in one place needed for an
/// analysis in one place. This makes it much harder to get them out
/// of sync
struct SchemaIter {
    one: ColumnFixedIntoIter<i32>,
    two: ColumnFixedIntoIter<f32>,
    three: ColumnFixedIntoIter<String>,
}

/// Initiate a new iterator by passing it the `Tree` which contains the data
impl SchemaIter {
    fn new(t: Tree) -> Result<SchemaIter, Error> {
        Ok(SchemaIter {
            // Initialize each column; they are identified by name and
            // a `nom`-like parser is needed to parse the
            // data. ::core::parsers contains many more parsers for
            // common ROOT types
            one: ColumnFixedIntoIter::new(&t, "one", be_i32)?,
            two: ColumnFixedIntoIter::new(&t, "two", be_f32)?,
            three: ColumnFixedIntoIter::new(&t, "three", string)?,
        })
    }
}

/// Iterator popping out `Model`s. Each model is one "event"
impl Iterator for SchemaIter {
    type Item = Model;
    fn next(&mut self) -> Option<Self::Item> {
        Some(Model {
            one: self.one.next()?,
            two: self.two.next()?,
            three: self.three.next()?
        })
    }
}

#[test]
fn read_simple() {
    let path = PathBuf::from("./src/test_data/simple.root");
    let f = RootFile::new_from_file(&path).expect("Failed to open file");
    let t = f.items()[0].as_tree().unwrap();
    let schema = SchemaIter::new(t).unwrap();
    for m in schema.into_iter() {
        println!("{:?}", m);
    }
}
