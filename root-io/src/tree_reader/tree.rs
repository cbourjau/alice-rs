use nom::branch::alt;
use nom::combinator::cond;
use nom::multi::length_data;
use nom::multi::{count, length_value};
use nom::number::complete::{be_f64, be_i32, be_i64, be_u16, be_u32, be_u8};
use nom::sequence::preceded;
use nom::Parser;
use nom_supreme::ParserExt;
use thiserror::Error;

use std::fmt;
use std::fmt::Debug;
use std::ops::Deref;

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

#[derive(Error, Debug)]
#[error("No branch named {0} (available: {1:?})")]
pub struct MissingBranch(String, Vec<String>);

impl<'s> Tree {
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

    pub fn branch_by_name(&self, name: &str) -> Result<&TBranch, MissingBranch> {
        self.branches()
            .into_iter()
            .find(|b| b.name == name)
            .ok_or_else(|| {
                MissingBranch(
                    name.to_string(),
                    self.branches()
                        .iter()
                        .map(|b| b.name.to_string())
                        .collect::<Vec<_>>(),
                )
            })
    }
}

/// Parse a `Tree` from the given buffer. Usually used through `FileItem::parse_with`.
#[allow(clippy::unnecessary_unwrap)]
pub fn ttree<'s, E>(context: &'s Context) -> impl RParser<'s, Tree, E>
where
    E: RootError<Span<'s>>,
{
    let parser = move |i| {
        let none_or_u8_buf = |input: Span<'s>| {
            alt((
                be_u32
                    .verify(|&v| v == 0)
                    .map(|_| None)
                    .context("empty ttree buffer"),
                raw(context)
                    .map(|r| Some(r.obj.to_vec()))
                    .context("filled ttree buffer"),
            ))
            .parse(input)
        };
        let (i, ver) = be_u16
            .verify(|v| [16, 17, 18, 19].contains(v))
            .context("assertion: ttree version is in 16-19")
            .parse(i)?;
        let (i, tnamed) = length_value(checked_byte_count, tnamed)
            .context("tnamed")
            .complete()
            .context("length-prefixed data")
            .parse(i)?;
        let (i, _tattline) = length_data(checked_byte_count)
            .context("tattrline")
            .complete()
            .context("length-prefixed data")
            .parse(i)?;
        let (i, _tattfill) = length_data(checked_byte_count)
            .context("tattrfill")
            .complete()
            .context("length-prefixed data")
            .parse(i)?;
        let (i, _tattmarker) = length_data(checked_byte_count)
            .context("tattrmarker")
            .complete()
            .context("length-prefixed data")
            .parse(i)?;
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
        // TODO change None to empty vec?
        let (i, _fclusterrangeend) = {
            if let Some(n_clst_range) = fnclusterrange {
                preceded(be_u8, count(be_i64, n_clst_range as usize))
                    .context("fclusterrange end")
                    .map(Some)
                    .parse(i)?
            } else {
                (i, None)
            }
        };
        let (i, _fclustersize) = {
            if let Some(n_clst_range) = fnclusterrange {
                preceded(be_u8, count(be_i64, n_clst_range as usize))
                    .context("fcluster size")
                    .map(Some)
                    .parse(i)?
            } else {
                (i, None)
            }
        };

        let (i, fbranches) = length_value(checked_byte_count, tobjarray(tbranch_hdr(context)))
            .context("ttree branches")
            .complete()
            .context("length-prefixed data")
            .parse(i)?;

        let (i, fleaves) = length_value(checked_byte_count, tobjarray(TLeaf::parse(context)))
            .context("ttree leaves")
            .complete()
            .context("length-prefixed data")
            .parse(i)?;

        let (i, faliases) = none_or_u8_buf.context("faliases").parse(i)?;
        let (i, findexvalues) = tarray(be_f64).context("findexvalues").parse(i)?;
        let (i, findex) = tarray(be_i32).context("findex").parse(i)?;
        let (i, ftreeindex) = none_or_u8_buf.context("ftreeindex").parse(i)?;
        let (i, ffriends) = none_or_u8_buf.context("ffriends").parse(i)?;
        let (i, fuserinfo) = none_or_u8_buf.context("fuserinfo").parse(i)?;
        let (i, fbranchref) = none_or_u8_buf.context("fbranchref").parse(i)?;
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
    };

    parser.context("ttree")
}
