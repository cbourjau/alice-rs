use std::fmt;
use std::fmt::Debug;
use std::ops::Deref;

use failure::Error;
use nom::{
    combinator::{cond, peek, verify},
    multi::{count, length_data, length_value},
    number::complete::*,
    sequence::preceded,
    IResult,
};

use crate::{
    core::parsers::*, core::types::*, tree_reader::branch::tbranch_hdr,
    tree_reader::branch::TBranch, tree_reader::leafs::TLeaf,
};

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
#[allow(dead_code)]
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
    fflushedbytes: Option<i64>,
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

impl Tree {
    /// Get all branches of a tree (including nested ones)
    pub(crate) fn branches(&self) -> Vec<&TBranch> {
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

    pub fn branch_by_name(&self, name: &str) -> Result<&TBranch, Error> {
        self.branches()
            .into_iter()
            .find(|b| b.name == name)
            .ok_or_else(|| {
                format_err!(
                    "Branch {} not found in tree: \n {:#?}",
                    name,
                    self.branches()
                        .iter()
                        .map(|b| b.name.to_owned())
                        .collect::<Vec<_>>()
                )
            })
    }
}

/// Parse a `Tree` from the given buffer. Usually used through `FileItem::parse_with`.
pub fn ttree<'s>(i: &'s [u8], context: &'s Context) -> IResult<&'s [u8], Tree> {
    let _curried_raw = |i| raw(i, context);
    let none_or_u8_buf = |i: &'s [u8]| match peek(be_u32)(i)? {
        (i, 0) => be_u32(i).map(|(i, _)| (i, None)),
        (i, _) => _curried_raw(i).map(|(i, r)| (i, Some(r.obj.to_vec()))),
    };
    let grab_checked_byte_count = move |i| {
        length_data(|i| {
            let (i, cnt) = checked_byte_count(i)?;
            Ok((i, cnt))
        })(i)
    };
    let (i, ver) = verify(be_u16, |v| [16, 17, 18, 19].contains(v))(i)?;
    let (i, tnamed) = length_value(checked_byte_count, tnamed)(i)?;
    let (i, _tattline) = grab_checked_byte_count(i)?;
    let (i, _tattfill) = grab_checked_byte_count(i)?;
    let (i, _tattmarker) = grab_checked_byte_count(i)?;
    let (i, fentries) = be_i64(i)?;
    let (i, ftotbytes) = be_i64(i)?;
    let (i, fzipbytes) = be_i64(i)?;
    let (i, fsavedbytes) = be_i64(i)?;
    let (i, fflushedbytes) = cond(ver >= 18, be_i64)(i)?;
    let (i, fweight) = be_f64(i)?;
    let (i, ftimerinterval) = be_i32(i)?;
    let (i, fscanfield) = be_i32(i)?;
    let (i, fupdate) = be_i32(i)?;
    let (i, _fdefaultentryoffsetlen) = cond(ver >= 17, be_i32)(i)?;
    let (i, fnclusterrange) = cond(ver >= 19, be_i32)(i)?;
    let (i, fmaxentries) = be_i64(i)?;
    let (i, fmaxentryloop) = be_i64(i)?;
    let (i, _fmaxvirtualsize) = be_i64(i)?;
    let (i, _fautosave) = be_i64(i)?;
    let (i, _fautoflush) = cond(ver >= 18, be_i64)(i)?;
    let (i, festimate) = be_i64(i)?;
    let (i, _fclusterrangeend) = {
        if let Some(n_clst_range) = fnclusterrange {
            preceded(be_u8, count(be_i64, n_clst_range as usize))(i)
                .map(|(i, ends)| (i, Some(ends)))?
        } else {
            (i, None)
        }
    };
    let (i, _fclustersize) = {
        if let Some(n_clst_range) = fnclusterrange {
            preceded(be_u8, count(be_i64, n_clst_range as usize))(i)
                .map(|(i, ends)| (i, Some(ends)))?
        } else {
            (i, None)
        }
    };
    let (i, fbranches) =
        length_value(checked_byte_count, |i| tobjarray(tbranch_hdr, i, context))(i)?;
    let (i, fleaves) = length_value(checked_byte_count, |i| {
        tobjarray(TLeaf::parse_from_raw, i, context)
    })(i)?;

    let (i, faliases) = none_or_u8_buf(i)?;
    let (i, findexvalues) = tarray(be_f64, i)?;
    let (i, findex) = tarray(be_i32, i)?;
    let (i, ftreeindex) = none_or_u8_buf(i)?;
    let (i, ffriends) = none_or_u8_buf(i)?;
    let (i, fuserinfo) = none_or_u8_buf(i)?;
    let (i, fbranchref) = none_or_u8_buf(i)?;
    let ftreeindex = ftreeindex.map(Pointer);
    let ffriends = ffriends.map(Pointer);
    let fuserinfo = fuserinfo.map(Pointer);
    let fbranchref = fbranchref.map(Pointer);
    Ok((
        i,
        Tree {
            ver,
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
            fbranchref,
        },
    ))
}
