use nom::*;
use std::fmt;
use std::ops::Deref;

use core::parsers::*;
use core::types::*;

use tree_reader::branch::tbranch_hdr;
use tree_reader::branch::TBranch;
use tree_reader::leafs::tleaf;
use tree_reader::leafs::TLeaf;

/// `TTree` potentially has members with very large `Vec<u8>` buffers
/// The `Pointer` type is used to overwrite the default `Debug` trait
/// for those members
struct Pointer(pub Vec<u8>);

impl Deref for Pointer {
    type Target = Vec<u8>;
    fn deref(&self) -> &Vec<u8> {
        &self.0
    }
}

impl fmt::Debug for Pointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Buffer of {} bytes ", self.len())
    }
}

/// A `Tree` is the default "container" for datasets in Root files The
/// data is oranized in so-called branches. This type is exposed only
/// for the purpose of creating `ColumnFixedIntoIter` and
/// `ColumnVarIntoIter` objects from it.
#[derive(Debug)]
pub struct Tree {
    /// Version of the read layout
    ver: u16,
    /// The basis for a named object (name, title)
    tnamed: TNamed,
    /// Number of entries
    fentries: i64,
    /// Total number of bytes in all branches before compression
    ftotbytes: i64,
    /// Total number of bytes in all branches after compression
    fzipbytes: i64,
    /// Number of autosaved bytes
    fsavedbytes: i64,
    /// Number of autoflushed bytes
    fflushedbytes: i64,
    /// Tree weight (see TTree::SetWeight)
    fweight: f64,
    /// Timer interval in milliseconds
    ftimerinterval: i32,
    /// Number of runs before prompting in Scan
    fscanfield: i32,
    /// Update frequency for EntryLoop
    fupdate: i32,
    /// Maximum number of entries in case of circular buffers
    fmaxentries: i64,
    /// Maximum number of entries to process
    fmaxentryloop: i64,
    /// Number of entries to estimate histogram limits
    festimate: i64,
    /// List of Branches
    pub(crate) fbranches: Vec<TBranch>,
    /// Direct pointers to individual branch leaves
    pub(crate) fleaves: Vec<TLeaf>,
    /// List of aliases for expressions based on the tree branches.
    faliases: Option<Vec<u8>>,
    /// Sorted index values
    findexvalues: Vec<f64>,
    /// Index of sorted values
    findex: Vec<i32>,
    /// Pointer to the tree Index (if any)
    ftreeindex: Option<Pointer>,
    /// pointer to list of friend elements
    ffriends: Option<Pointer>,
    /// pointer to a list of user objects associated to this Tree
    fuserinfo: Option<Pointer>,
    /// Branch supporting the TRefTable (if any)
    fbranchref: Option<Pointer>,
}

impl<'s> Tree {
    /// Get all branches of a tree (including nested ones)
    pub(crate) fn branches(&self) -> Vec<(&TBranch)> {
        self.fbranches
            .iter()
            .flat_map(|b| vec![b].into_iter().chain(b.branches().into_iter()))
            .collect()
    }
    /// Get all the branch names and types (including nested ones) of this tree
    /// The first element is the name, the second one is the type
    pub fn branch_names_and_types(&self) -> Vec<(String, Vec<String>)> {
        self.fbranches
            .iter()
            .flat_map(|b| vec![b].into_iter().chain(b.branches().into_iter()))
            .map(|b| (b.name(), b.element_types()))
            .collect()
    }
}

/// Parse a `Tree` from the given buffer. Usually used through `FileItem::parse_with`.
#[allow(unused_variables, clippy::unnecessary_unwrap)]
pub fn ttree<'s>(input: &'s [u8], context: &Context) -> IResult<&'s [u8], Tree> {
    let _curried_raw = |i| raw(i, context);
    let none_or_u8_buf = |i: &'s [u8]| {
        switch!(i, peek!(be_u32),
                                            0 => map!(call!(be_u32), | _ | None) |
                                            _ => map!(
                                                map!(call!(_curried_raw), |r| r.obj.to_vec()),
                                                Some))
    };
    let grab_checked_byte_count = |i| length_data!(i, checked_byte_count);
    let wrapped_tobjarray = |i: &'s[u8]| length_value!(i, checked_byte_count, apply!(tobjarray, context));    
    do_parse!(input,
              ver: be_u16 >>
              tnamed: length_value!(checked_byte_count, tnamed) >>
              _tattline: grab_checked_byte_count >>
              _tattfill: grab_checked_byte_count >>
              _tattmarker: grab_checked_byte_count >>
              fentries: be_i64 >>
              ftotbytes: be_i64 >>
              fzipbytes: be_i64 >>
              fsavedbytes: be_i64 >>
              fflushedbytes: be_i64 >>
              fweight: be_f64 >>
              ftimerinterval: be_i32 >>
              fscanfield: be_i32 >>
              fupdate: be_i32 >>
              _fdefaultentryoffsetlen: cond!(ver >= 18, be_i32) >>
              fnclusterrange: cond!(ver >= 19, be_i32) >>
              fmaxentries: be_i64 >>
              fmaxentryloop: be_i64 >>
              _fmaxvirtualsize: be_i64 >>
              _fautosave: be_i64 >>
              _fautoflush: be_i64 >>
              festimate: be_i64 >>
              _fclusterrangeend: cond!(fnclusterrange.is_some(),
                                      preceded!(be_u8,
                                                count!(be_i64, fnclusterrange.unwrap() as usize))) >>
              _fclustersize: cond!(fnclusterrange.is_some(),
                                  preceded!(be_u8,
                                            count!(be_i64, fnclusterrange.unwrap() as usize))) >>
              fbranches: wrapped_tobjarray >>
              fleaves: wrapped_tobjarray >>
              faliases: none_or_u8_buf >>
              findexvalues: tarrayd >>
              findex: tarrayi >>
              ftreeindex: none_or_u8_buf >>
              ffriends: none_or_u8_buf >>
              fuserinfo: none_or_u8_buf >>
              fbranchref: none_or_u8_buf >>
              ({
                  let fbranches = fbranches.iter()
                      .map(|s| tbranch_hdr(s, context).unwrap().1)
                      .collect();
                  let fleaves = fleaves.into_iter()
                      .map(|raw| tleaf(raw.obj, context, &raw.classinfo).unwrap().1)
                      .collect();
                  let ftreeindex = ftreeindex.map(Pointer);
                  let ffriends = ffriends.map(Pointer);
                  let fuserinfo = fuserinfo.map(Pointer);
                  let fbranchref = fbranchref.map(Pointer);
                  Tree {ver,
                        tnamed,
                        fentries,
                        ftotbytes,
                        fzipbytes,
                        fsavedbytes,
                        fflushedbytes,
                        fweight,
                        ftimerinterval,
                        fscanfield,
                        fupdate,
                        fmaxentries,
                        fmaxentryloop,
                        festimate,
                        fbranches,
                        fleaves,
                        faliases,
                        findexvalues,
                        findex,
                        ftreeindex,
                        ffriends,
                        fuserinfo,
                        fbranchref } }))
}
