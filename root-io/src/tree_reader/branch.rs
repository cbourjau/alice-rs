use std::io::SeekFrom;
use std::path::PathBuf;
use nom::*;

use core::parsers::*;
// use crate::core::types::*;
use crate::core::types::{Raw, Context};

use code_gen::rust::ToRustType;

use tree_reader::container::Container;
use tree_reader::leafs::TLeaf;
use tree_reader::leafs::tleaf;

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
     containers: Vec<Container>
}

impl TBranch {
    /// Return the endpoints of all sub-branches of this branch
    pub fn branches(&self) -> Vec<&TBranch> {
        let out: Vec<_> = self.fbranches.iter()
            .flat_map(|b| b.branches())
            .collect();
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
        self.fleaves.iter()
            .map(|l| l.type_name().to_string())
            .collect()
    }

    /// Number of events in each basket. This might be important to know when parsing basket for variale length objects
    pub(crate) fn n_events_per_basket(&self) -> Vec<usize> {
        // basketentry is index of first element in each basket, e.g. [0, 2, 4]
        self.fbasketentry.iter()
            // the last event index is not in fbasketentry
            .chain([self.fentries].into_iter())
            .collect::<Vec<_>>()
            .windows(2)
            .map(|window| (window[1] - window[0]) as usize)
            .collect()
    }
}


/// `TBranchElements` are a subclass of `TBranch` if the content is an Object
/// We ignore the extra information for now and just parse the TBranch"Header" in either case
pub(crate) fn tbranch_hdr<'s>(raw: &Raw<'s>, ctxt: &'s Context) -> IResult<&'s[u8], TBranch> {
    match raw.classinfo.as_str() {
        "TBranchElement" | "TBranchObject" => {
            preceded!(raw.obj, be_u16, // version
                      length_value!(checked_byte_count, apply!(tbranch, ctxt)))
        },
        "TBranch" => tbranch(raw.obj, ctxt),
        name => panic!("Unexpected Branch type {}", name)
    }
}

fn tbranch<'s>(input: &'s [u8], context: & Context<'s>) -> IResult<&'s [u8], TBranch> {
    let _curried_raw = |i| raw(i, context);
    let wrapped_tobjarray = |i: &'s[u8]| length_value!(i, checked_byte_count, apply!(tobjarray, context));
    do_parse!(input,
              _ver: verify!(be_u16, |v| v == 12) >>
               tnamed: length_value!(checked_byte_count, tnamed) >>
               _tattfill: length_data!(checked_byte_count) >>
               fcompress: be_i32 >>
               fbasketsize: be_i32 >>
               fentryoffsetlen: be_i32 >>
               fwritebasket: be_i32 >>
               fentrynumber: be_i64 >>
               foffset: be_i32 >>
               fmaxbaskets: be_i32 >>
               fsplitlevel: be_i32 >>
               fentries: be_i64 >>
               ffirstentry: be_i64 >>
               ftotbytes: be_i64 >>
               fzipbytes: be_i64 >>
               fbranches: wrapped_tobjarray >>
               fleaves: wrapped_tobjarray >>
               fbaskets: wrapped_tobjarray >>
               fbasketbytes: preceded!(be_u8, count!(be_i32, fmaxbaskets as usize)) >>
               fbasketentry: preceded!(be_u8, count!(be_i64, fmaxbaskets as usize)) >>
               fbasketseek: preceded!(be_u8, count!(be_u64, fmaxbaskets as usize)) >>
               ffilename: string >>
              ({
                  let name = tnamed.name;
                  let fbranches = fbranches.iter()
                      .map(|s| tbranch_hdr(s, context).unwrap().1)
                      .collect();
                  let fleaves = fleaves.into_iter()
                      .map(|r| tleaf(r.obj, context, &r.classinfo).unwrap().1)
                      .collect();
                  // Remove tailing empty baskets informations
                  let fbaskets = fbaskets.into_iter()
                      .filter(|s| !s.obj.is_empty())
                      .map(|s| Container::InMemory(s.obj.to_vec()));
                  let nbaskets = fwritebasket as usize;
                  let fbasketbytes = fbasketbytes.into_iter()
                      .take(nbaskets)
                      .map(|val| val as usize);
                  let fbasketentry = fbasketentry.into_iter().take(nbaskets).collect();
                  let fbasketseek = fbasketseek.into_iter()
                      .take(nbaskets)
                      .map(SeekFrom::Start);
                  let ffilename =
                      if ffilename == "" {
                          context.path.to_owned()
                      } else {
                          PathBuf::from(ffilename)
                      };
                  let containers_disk = fbasketseek
                      .zip(fbasketbytes)
                      .map(|(seek, len)| Container::OnDisk(ffilename.clone(), seek, len));
                  let containers = fbaskets.chain(containers_disk).collect();
                  TBranch {name,
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
                           containers
                  }
              }))
}
