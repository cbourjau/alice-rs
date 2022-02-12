use futures::prelude::*;
use nom::{error::VerboseError, IResult, multi::{count, length_data, length_value}, number::complete::*, Parser};
use nom::combinator::eof;
use nom_supreme::ParserExt;

use std::fmt::Debug;

use crate::{
    code_gen::rust::ToRustType, core::parsers::*, core::types::*,
    tree_reader::container::Container, tree_reader::leafs::TLeaf,
};

/// A `TBranch` describes one "Column" of a `TTree`
/// Even though this class is described in the `TStreamerInfo` of a ROOT
/// file, it is hard coded in this library to provide a reliable API
/// for working with `TTree`s
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TBranch {
    /// The name of this object
    pub name: String,
    /// Compression level and algorithm
    fcompress: i32,
    /// Initial Size of  Basket Buffer
    fbasketsize: i32,
    /// Initial Length of fEntryOffset table in the basket buffers
    fentryoffsetlen: i32,
    /// Last basket number written
    fwritebasket: i32,
    /// Current entry number (last one filled in this branch)
    fentrynumber: i64,
    /// Offset of this branch
    foffset: i32,
    /// Branch split level
    fsplitlevel: i32,
    /// Number of entries
    fentries: i64,
    /// Number of the first entry in this branch
    ffirstentry: i64,
    /// Total number of bytes in all leaves before compression
    ftotbytes: i64,
    /// Total number of bytes in all leaves after compression
    fzipbytes: i64,
    /// -> List of Branches of this branch
    fbranches: Vec<TBranch>,
    /// -> List of leaves of this branch (TODO: Parse to TLeafC/I/F..)
    fleaves: Vec<TLeaf>,
    /// Table of first entry in each basket
    fbasketentry: Vec<i64>,
    containers: Vec<Container>,
}

impl TBranch {
    /// Return the endpoints of all sub-branches of this branch
    pub fn branches(&self) -> Vec<&TBranch> {
        let out: Vec<_> = self.fbranches.iter().flat_map(|b| b.branches()).collect();
        if out.is_empty() {
            vec![self]
        } else {
            out
        }
    }

    /// Access to the `Containers` containing the data of this branch
    pub(crate) fn containers(&self) -> &[Container] {
        &self.containers
    }

    /// The name of this branch
    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    /// The type(s) of the elements in this branch For some reason,
    /// there may be situations where a branch has several leaves and thus types.
    pub fn element_types(&self) -> Vec<String> {
        self.fleaves
            .iter()
            .map(|l| l.type_name().to_string())
            .collect()
    }

    /// Create an iterator over the data of a column (`TBranch`) with a
    /// constant number of element per entry (or at least not a
    /// variable number of entries which depends on an external list of
    /// indices. For the latter case see `as_var_size_iterator`).
    ///
    /// # Example
    /// ```
    /// extern crate failure;
    /// extern crate nom;
    /// extern crate root_io;
    /// use futures::StreamExt;
    ///
    /// use std::path::Path;
    /// use nom::number::complete::be_i32;
    ///
    /// use root_io::tree_reader::Tree;
    /// use root_io::RootFile;
    ///
    /// #[tokio::main]
    ///# async fn main
    ///
    ///# () {
    ///     let path = Path::new("./src/test_data/simple.root");
    ///     let f = RootFile::new(path).await.expect("Failed to open file");
    ///     let tree = f.items()[0].as_tree().await.unwrap();
    ///     let numbers = tree
    ///         .branch_by_name("one").unwrap()
    ///         // Must pass parser as closure
    ///         .as_fixed_size_iterator(|i| be_i32(i));
    ///     numbers.for_each(|n| async move {
    ///         println!("All the numbers of this branch{:?}", n);
    ///     }).await;
    ///# }
    /// ```
    pub fn as_fixed_size_iterator<T, P>(&self, p: P) -> impl Stream<Item=T>
        where
            P: Fn(&[u8]) -> IResult<&[u8], T, VerboseError<&[u8]>>,
    {
        stream::iter(self.containers().to_owned())
            .then(|basket| async move { basket.raw_data().await.unwrap() })
            .map(move |(n_events_in_basket, buffer)| {
                // Parse the entire basket buffer; if something is left over its just junk
                let x = count(&p, n_events_in_basket as usize)(&buffer);
                let events = match x {
                    Ok((_rest, output)) => output,
                    Err(e) => panic!("Parser failed unexpectedly {:?}", e),
                };
                stream::iter(events)
            })
            .flatten()
    }

    /// Iterator over the data of a column (`TBranch`) with a variable
    /// number of elements per entry.  See the file
    /// [`read_esd.rs`](https://github.com/cbourjau/root-io/blob/master/src/tests/read_esd.rs)
    /// in the repository for a comprehensive example
    pub fn as_var_size_iterator<T, P>(
        &self,
        p: P,
        el_counter: Vec<u32>,
    ) -> impl Stream<Item = Vec<T>>
    where
        P: Fn(&[u8]) -> IResult<&[u8], T, VerboseError<&[u8]>>,
    {
        let mut elems_per_event = el_counter.into_iter();
        stream::iter(self.containers().to_owned())
            .then(|basket| async move { basket.raw_data().await.unwrap() })
            .map(move |(n_events_in_basket, buffer)| {
                let mut buffer = buffer.as_slice();
                let mut events = Vec::with_capacity(n_events_in_basket as usize);
                for _ in 0..n_events_in_basket {
                    if let Some(n_elems_in_event) = elems_per_event.next() {
                        match count(&p, n_elems_in_event as usize)(buffer) {
                            Ok((rest, output)) => {
                                buffer = rest;
                                events.push(output)
                            }
                            Err(e) => panic!("Parser failed unexpectedly {:?}", e),
                        }
                    }
                }
                stream::iter(events)
            })
            .flatten()
    }
}

/// `TBranchElements` are a subclass of `TBranch` if the content is an Object
/// We ignore the extra information for now and just parse the TBranch"Header" in either case
pub(crate) fn tbranch_hdr<'s, E>(ctxt: &'s Context) -> impl Parser<Raw<'s>, TBranch, E>
    where
        E: RootError<&'s [u8]>,
{
    move |raw: Raw<'s>| {
        match raw.classinfo {
            "TBranchElement" | "TBranchObject" => {
                be_u16.precedes(length_value(checked_byte_count, tbranch(ctxt)))
                    .terminated(eof)
                    .parse(raw.obj)
            }
            "TBranch" =>
                tbranch(ctxt)
                    .terminated(eof)
                    .parse(raw.obj),
            name => panic!("Unexpected Branch type {}", name),
        }.map(|(i, res)| (Raw { classinfo: raw.classinfo, obj: i }, res))
    }
}

fn tbranch<'s, E>(context: &'s Context) -> impl Parser<&'s [u8], TBranch, E>
    where
        E: RootError<&'s [u8]>,
{
    move |inpt| {
        let (i, _ver) = be_u16.verify(|v| [11, 12].contains(v)).parse(inpt)?;
        let (i, tnamed) = length_value(checked_byte_count, tnamed).parse(i)?;
        let (i, _tattfill) = length_data(checked_byte_count).parse(i)?;
        let (i, fcompress) = be_i32(i)?;
        let (i, fbasketsize) = be_i32(i)?;
        let (i, fentryoffsetlen) = be_i32(i)?;
        let (i, fwritebasket) = be_i32(i)?;
        let (i, fentrynumber) = be_i64(i)?;
        let (i, foffset) = be_i32(i)?;
        let (i, fmaxbaskets) = be_i32(i)?;
        let (i, fsplitlevel) = be_i32(i)?;
        let (i, fentries) = be_i64(i)?;
        let (i, ffirstentry) = be_i64(i)?;
        let (i, ftotbytes) = be_i64(i)?;
        let (i, fzipbytes) = be_i64(i)?;
        let (i, fbranches) =
            length_value(checked_byte_count, tobjarray(tbranch_hdr, context))(i)?;
        let (i, fleaves) =
            length_value(checked_byte_count, tobjarray(TLeaf::parse_from_raw, context))(i)?;
        let (i, fbaskets) =
            length_value(checked_byte_count,
                         tobjarray(|_| |r: Raw<'s>| Ok((Raw { classinfo: r.classinfo, obj: &[] }, r.obj)), context))(i)?;
        let (i, fbasketbytes) = be_u8.precedes(count(be_i32, fmaxbaskets as usize)).parse(i)?;
        let (i, fbasketentry) = be_u8.precedes(count(be_i64, fmaxbaskets as usize)).parse(i)?;
        let (i, fbasketseek) = be_u8.precedes(count(be_u64, fmaxbaskets as usize)).parse(i)?;
        let (i, ffilename) = string(i)?;

        let name = tnamed.name;
        let fbaskets = fbaskets
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|s| Container::InMemory(s.to_vec()));
        let nbaskets = fwritebasket as usize;
        let fbasketbytes = fbasketbytes
            .into_iter()
            .take(nbaskets)
            .map(|val| val as usize);
        let fbasketentry = fbasketentry.into_iter().take(nbaskets).collect();
        let fbasketseek = fbasketseek.into_iter().take(nbaskets);
        let source = if ffilename.is_empty() {
            context.source.to_owned()
        } else {
            unimplemented!("Root files referencing other Root files is not implemented")
        };
        let containers_disk = fbasketseek
            .zip(fbasketbytes)
            .map(|(seek, len)| Container::OnDisk(source.clone(), seek, len as u64));
        let containers = fbaskets.chain(containers_disk).collect();
        Ok((
            i,
            TBranch {
                name,
                fcompress,
                fbasketsize,
                fentryoffsetlen,
                fwritebasket,
                fentrynumber,
                foffset,
                fsplitlevel,
                fentries,
                ffirstentry,
                ftotbytes,
                fzipbytes,
                fbranches,
                fleaves,
                fbasketentry,
                containers,
            },
        ))
    }
}
