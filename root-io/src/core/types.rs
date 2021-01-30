use std::fmt;

use crate::core::Source;

use nom::HexDisplay;

/// Absolute point in file to seek data
pub(crate) type SeekPointer = u64;

bitflags! {
    pub(crate) struct Flags: u32 {
        const BYTE_COUNT_MASK = 0x4000_0000;
        const BYTE_COUNT_VMASK = 0x4000;      // 16384
        const CLASS_MASK = 0x8000_0000;
        const NEW_CLASSTAG = 0xFFFF_FFFF;
    }
}
bitflags! {
    pub(crate) struct TObjectFlags: u32 {
        const IS_ON_HEAP = 0x0100_0000;
        const IS_REFERENCED = 1 << 4;
    }
}

/// Used in `TStreamerInfo`
/// Describes if the following entry is a new class or if it was
/// already described.
#[derive(Debug)]
pub enum ClassInfo<'s> {
    /// Class name of the new class
    New(&'s str),
    /// Byte offset of new class tag in record, + 2; whatever... followed by object
    Exists(u32),
    /// Byte offset of new class tag in record, + 2; whatever... ref to object
    References(u32),
}

/// The most basic ROOT object from which almost everything inherits
#[derive(Debug, Clone)]
pub struct TObject {
    pub(crate) ver: u16,
    pub(crate) id: u32,
    pub(crate) bits: TObjectFlags,
}

/// A ROOT object with a name and a title
#[derive(Debug, Clone)]
pub struct TNamed {
    // pub(crate) ver: u16,
    // pub(crate) tobj: TObject,
    pub name: String,
    pub title: String,
}

/// A "list" (implemented as `Vec`) of arbitrary objects
#[derive(Debug)]
pub struct TList<'a> {
    pub(crate) ver: u16,
    pub(crate) tobj: TObject,
    pub(crate) name: String,
    pub(crate) len: usize,
    pub(crate) objs: Vec<Raw<'a>>,
}

/// A type holding nothing but the original data and a class info object
pub struct Raw<'s> {
    pub(crate) classinfo: &'s str,
    pub(crate) obj: &'s [u8],
}

/// The context from which we are currently parsing
#[derive(Debug)]
pub struct Context<'s> {
    /// Path to file of this context
    pub source: Source,
    /// Offset between the beginning of `s` and to where absolute
    /// positions in the buffer point (e.g. for class defs)
    /// Usually something like TKey-length + 4
    pub offset: u64,
    /// The full buffer we are working on
    pub s: &'s [u8],
}

impl<'s> fmt::Debug for Raw<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} \n {}", self.classinfo, self.obj.to_hex(16))
    }
}

// Types which are so far unused:
// pub type TArrayD = Vec<f64>;
// pub type TArrayI = Vec<i32>;
// pub type TArrayL64 = Vec<i64>;
