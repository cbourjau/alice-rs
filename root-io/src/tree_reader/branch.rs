use futures::prelude::*;
use nom::multi::count;
use nom::number::complete::*;
use nom::*;

use crate::{
    code_gen::rust::ToRustType, core::parsers::*, core::types::*,
    tree_reader::container::Container, tree_reader::leafs::tleaf, tree_reader::leafs::TLeaf,
};

/// A `TBranch` describes one "Column" of a `TTree`
/// Even though this class is described in the `TStreamerInfo` of a ROOT
/// file, it is hard coded in this library to provide a reliable API
/// for working with `TTree`s
#[derive(Debug, Clone)]
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
    /// use std::path::PathBuf;
    /// use nom::number::complete::be_i32;
    ///
    /// use root_io::tree_reader::Tree;
    /// use root_io::RootFile;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let path = PathBuf::from("./src/test_data/simple.root");
    ///     let f = RootFile::new_from_file(&path).await.expect("Failed to open file");
    ///     let tree = f.items()[0].as_tree().await.unwrap();
    ///     let numbers = tree
    ///         .branch_by_name("one").unwrap()
    ///         // Must pass parser as closure
    ///         .as_fixed_size_iterator(|i| be_i32(i));
    ///     numbers.for_each(|n| async move {
    ///         println!("All the numbers of this branch{:?}", n);
    ///     }).await;
    /// }
    /// ```
    pub fn as_fixed_size_iterator<T, P>(&self, p: P) -> impl Stream<Item = T>
    where
        P: Fn(&[u8]) -> IResult<&[u8], T>,
    {
        stream::iter(self.containers().to_owned())
            .then(|basket| async move { basket.raw_data().await.unwrap() })
            .map(move |(n_events_in_basket, buffer)| {
                // Parse the entire basket buffer; if something is left over its just junk
                let events = match count(&p, n_events_in_basket as usize)(&buffer) {
                    Ok((_rest, output)) => output,
                    Err(e) => panic!(format!("Parser failed unexpectedly {:?}", e)),
                };
                stream::iter(events)
            })
            .flatten()
    }

    /// Iterator over the data of a column (`TBranch`) with a variable
    /// number of elements per entry.  See the file
    /// [`read_esd.rs`](https://github.com/cbourjau/root-io/blob/master/src/tests/read_esd.rs)
    /// in the repository for a comprehensive example
    pub fn as_var_size_iterator<T, P>(&self, p: P, el_counter: &[u32]) -> impl Stream<Item = Vec<T>>
    where
        P: Fn(&[u8]) -> IResult<&[u8], T>,
    {
        let mut elems_per_event = el_counter.to_owned().into_iter();
        stream::iter(self.containers().to_owned())
            .then(|basket| async move { basket.raw_data().await.unwrap() })
            .map(move |(n_events_in_basket, buffer)| {
                let mut buffer = buffer.as_slice();
                let mut events = Vec::with_capacity(n_events_in_basket as usize);
                for _ in 0..n_events_in_basket {
                    if let Some(n_elems_in_event) = elems_per_event.next() {
                        match count(&p, n_elems_in_event as usize)(&buffer) {
                            Ok((rest, output)) => {
                                buffer = rest;
                                events.push(output)
                            }
                            Err(e) => panic!(format!("Parser failed unexpectedly {:?}", e)),
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
pub(crate) fn tbranch_hdr<'s>(raw: &Raw<'s>, ctxt: &'s Context) -> IResult<&'s [u8], TBranch> {
    match raw.classinfo.as_str() {
        "TBranchElement" | "TBranchObject" => {
            preceded!(
                raw.obj,
                be_u16, // version
                length_value!(checked_byte_count, call!(tbranch, ctxt))
            )
        }
        "TBranch" => tbranch(raw.obj, ctxt),
        name => panic!("Unexpected Branch type {}", name),
    }
}

fn tbranch<'s>(input: &'s [u8], context: &Context<'s>) -> IResult<&'s [u8], TBranch> {
    let _curried_raw = |i| raw(i, context);
    let wrapped_tobjarray =
        |i: &'s [u8]| length_value!(i, checked_byte_count, call!(tobjarray, context));
    do_parse!(
        input,
        _ver: verify!(be_u16, |v| *v == 12)
            >> tnamed: length_value!(checked_byte_count, tnamed)
            >> _tattfill: length_data!(checked_byte_count)
            >> fcompress: be_i32
            >> fbasketsize: be_i32
            >> fentryoffsetlen: be_i32
            >> fwritebasket: be_i32
            >> fentrynumber: be_i64
            >> foffset: be_i32
            >> fmaxbaskets: be_i32
            >> fsplitlevel: be_i32
            >> fentries: be_i64
            >> ffirstentry: be_i64
            >> ftotbytes: be_i64
            >> fzipbytes: be_i64
            >> fbranches: wrapped_tobjarray
            >> fleaves: wrapped_tobjarray
            >> fbaskets: wrapped_tobjarray
            >> fbasketbytes: preceded!(be_u8, count!(be_i32, fmaxbaskets as usize))
            >> fbasketentry: preceded!(be_u8, count!(be_i64, fmaxbaskets as usize))
            >> fbasketseek: preceded!(be_u8, count!(be_u64, fmaxbaskets as usize))
            >> ffilename: string
            >> ({
                let name = tnamed.name;
                let fbranches = fbranches
                    .iter()
                    .map(|s| tbranch_hdr(s, context).unwrap().1)
                    .collect();
                let fleaves = fleaves
                    .into_iter()
                    .map(|r| tleaf(r.obj, context, &r.classinfo).unwrap().1)
                    .collect();
                // Remove tailing empty baskets informations
                let fbaskets = fbaskets
                    .into_iter()
                    .filter(|s| !s.obj.is_empty())
                    .map(|s| Container::InMemory(s.obj.to_vec()));
                let nbaskets = fwritebasket as usize;
                let fbasketbytes = fbasketbytes
                    .into_iter()
                    .take(nbaskets)
                    .map(|val| val as usize);
                let fbasketentry = fbasketentry.into_iter().take(nbaskets).collect();
                let fbasketseek = fbasketseek.into_iter().take(nbaskets);
                let source = if ffilename == "" {
                    context.source.to_owned()
                } else {
                    unimplemented!("Root files referencing other Root files is not implemented")
                };
                let containers_disk = fbasketseek
                    .zip(fbasketbytes)
                    .map(|(seek, len)| Container::OnDisk(source.clone(), seek, len as u64));
                let containers = fbaskets.chain(containers_disk).collect();
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
                }
            })
    )
}

// /// Iterator over chunks of variable size. It solves the issue that we
// /// get chunks of data from the underlying TBuckets, but then need to
// /// chunk that data into smaller, variable sized chunks again. The
// /// size of the smaller chunks is given by the `chunk_size`
// /// iterator. The bigger chunks are produced by an inner stream of
// /// `Vec<el>`s as read from file.
// struct VarChunkIter<IChunkSizes: Iterator<Item = u32>, IBigChunks: Stream> {
//     /// Number of elements in each chunk
//     chunk_sizes: IChunkSizes,
//     /// Inner iterators over elements. This is wanted since we don't
//     /// want to read all baskets form file immediately
//     inner: IBigChunks,
//     /// The big chunk we are currently chopping up
//     current_big_chunk: IBigChunks::Item
// }

// impl<T, IChunkSizes: Iterator<Item = u32>, IBigChunks: Stream<Item=Vec<T>>> VarChunkIter<IChunkSizes, IBigChunks> {
//     fn new(chunk_sizes: IChunkSizes, elements: IBigChunks) -> Self {
//         Self {
//             chunk_sizes,
//             inner: elements,
//             current_big_chunk: Vec::new(),
//         }
//     }
// }

// impl<T, IChunkSizes, IBigChunks> Stream
//     for VarChunkIter<IChunkSizes, IBigChunks>
// where
//     IChunkSizes: Iterator<Item = u32>,
//     IBigChunks: Stream<Item=Vec<T>>,
// {
//     type Item = <IBigChunks as Stream>::Item;

//     fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut futures::task::Context) -> futures::task::Poll<Option<Self::Item>> {
//         // if self.current_big_chunk.is_empty() {
//         //     match self.inner.poll_next(&mut cx) {
//         //     }
//         // }
//         unimplemented!()
//     }
//     // fn next(&mut self) -> Option<Self::Item> {
//     //     if let Some(n_elem) = self.chunk_sizes.next() {
//     //         let mut chunk = Vec::with_capacity(n_elem as usize);
//     //         for _ in 0..n_elem {
//     //             chunk.push(self.inner.next().expect("Inner Iterator ended"));
//     //         }
//     //         Some(chunk)
//     //     } else {
//     //         // No more elements expected; TODO check if `inner` is also
//     //         // depleted
//     //         None
//     //     }
//     // }
// }
