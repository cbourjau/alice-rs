use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use failure::Error;
use nom::*;

use code_gen::rust::{ToNamedRustParser, ToRustStruct};
use core::*;
use MAP_OFFSET;

/// `RootFile` wraps the most basic information of a ROOT file.
#[derive(Debug)]
pub struct RootFile {
    path: PathBuf,
    hdr: FileHeader,
    items: Vec<FileItem>,
}

#[derive(Debug, PartialEq)]
struct FileHeader {
    version: i32,
    begin: i32,
    end: i32,
    seek_free: i32,
    nbytes_free: i32,
    n_entries_free: i32,
    n_bytes_name: i32,
    pointer_size: u8,
    compression: i32,
    seek_info: SeekFrom,
    nbytes_info: i32,
    uuid: i64,
    seek_dir: SeekFrom,
}

#[derive(Debug, PartialEq)]
pub struct Directory {
    version: i16,
    c_time: u32,
    m_time: u32,
    n_bytes_keys: i32,
    n_bytes_name: i32,
    seek_dir: SeekFrom,
    seek_parent: SeekFrom,
    seek_keys: SeekFrom,
}

named!(
    #[doc="Opening part of a root file"],
    file_header<&[u8], FileHeader>,
    do_parse!(
        tag!("root") >>
            version: be_i32 >>
            begin:   be_i32 >>
            end:     be_i32 >>
            seek_free: be_i32 >>
            nbytes_free: be_i32 >>
            n_entries_free: be_i32 >>
            n_bytes_name: be_i32 >>
            pointer_size: be_u8 >>
            compression: be_i32 >>
            seek_info: map!(be_i32, |v| SeekFrom::Start(v as u64)) >>
            nbytes_info: be_i32 >>
            uuid: be_i64 >>
            ({
                let seek_dir = SeekFrom::Start((begin + n_bytes_name) as u64);
                FileHeader {
                    version, begin, end, seek_free, nbytes_free,
                    n_entries_free, n_bytes_name, pointer_size,
                    compression, seek_info, nbytes_info, uuid,
                    seek_dir,
                }})
    )
);

named!(
    #[doc="Directory within a root file; exists on ever file"],
    directory<&[u8], Directory>,
    do_parse!(
        version: be_i16 >>
        c_time: be_u32 >>
        m_time: be_u32 >>
        n_bytes_keys: be_i32 >>
        n_bytes_name: be_i32 >>
        seek_dir: alt_complete!(
            cond_reduce!(version > 1000, be_u64) | be_i32 => {|val| val as u64}) >>
        seek_parent: alt_complete!(
            cond_reduce!(version > 1000, be_u64) | be_i32 => {|val| val as u64}) >>
        seek_keys: alt_complete!(
            cond_reduce!(version > 1000, be_u64) | be_i32 => {|val| val as u64}) >>
            ({
                let seek_dir = SeekFrom::Start(seek_dir);
                let seek_parent = SeekFrom::Start(seek_parent);
                let seek_keys = SeekFrom::Start(seek_keys);
                Directory {version, c_time, m_time, n_bytes_keys,
                           n_bytes_name, seek_dir, seek_parent, seek_keys,
                }})
    )
);

impl RootFile {
    /// Open a ROOT file and read in the necessary meta information
    pub fn new_from_file(path: &Path) -> Result<Self, Error> {
        let f = File::open(&path)?;
        let mut reader = BufReader::new(f);
        let hdr = parse_buffer(&mut reader, 256, file_header)?;

        // Jump to the TDirectory and parse it
        reader.seek(hdr.seek_dir)?;
        let dir = parse_buffer(&mut reader, 256, directory)?;
        let tkey_of_keys = {
            // Jump to TKey holding a list of TKeys describing the file content
            reader.seek(dir.seek_keys)?;
            parse_buffer(&mut reader, 1024, tkey)?
        };
        let keys = match tkey_headers(&tkey_of_keys.obj) {
            IResult::Done(_, hdrs) => Ok(hdrs),
            _ => Err(format_err!("Expected TKeyHeaders")),
        }?;
        let items = keys
            .iter()
            .map(|k_hdr| FileItem::new(k_hdr, &path))
            .collect();

        let path = path.to_owned();
        Ok(RootFile { path, hdr, items })
    }

    /// Return all `TSreamerInfo` for the data in this file
    pub fn streamers(&self) -> Result<Vec<TStreamerInfo>, Error> {
        let f = File::open(&self.path)?;
        let mut reader = BufReader::new(f);

        // Read streamer info
        reader.seek(self.hdr.seek_info)?;
        // Dunno why we are 4 bytes off with the size of the streamer info...
        let info_key = parse_buffer(&mut reader, (self.hdr.nbytes_info + 4) as usize, tkey)?;

        let key_len = info_key.hdr.key_len;
        let context = Context {
            path: self.path.to_owned(),
            offset: key_len as u64 + MAP_OFFSET,
            s: info_key.obj.as_slice(),
        };
        // This TList in the payload has a bytecount in front...
        let wrapped_tlist = |i| apply!(i, tlist, &context);
        let tlist_objs =
            match length_value!(info_key.obj.as_slice(), checked_byte_count, wrapped_tlist) {
                IResult::Done(_, l) => Ok(l.objs),
                _ => Err(format_err!("Expected TStreamerInfo's TList")),
            }?;
        // Mainly this is a TList of `TStreamerInfo`s, but there might
        // be some "rules" in the end
        let streamers = Ok(tlist_objs
            .iter()
            .filter_map(|raw| match raw.classinfo.as_str() {
                "TStreamerInfo" => Some(raw.obj),
                _ => None,
            })
            .map(|i| tstreamerinfo(i, &context).unwrap().1)
            .collect());
        // Parse the "rules", if any, from the same tlist
        let _rules: Vec<_> = tlist_objs
            .iter()
            .filter_map(|raw| match raw.classinfo.as_str() {
                "TList" => Some(raw.obj),
                _ => None,
            })
            .map(|i| {
                let tl = tlist(i, &context).unwrap().1;
                // Each `Rule` is a TList of `TObjString`s
                tl.objs
                    .iter()
                    .map(|el| tobjstring(el.obj).unwrap().1)
                    .collect::<Vec<_>>()
            })
            .collect();
        streamers
    }

    /// Slice of the items contained in this file
    pub fn items(&self) -> &[FileItem] {
        &self.items
    }

    /// Translate the streamer info of this file to a YAML file
    pub fn streamer_info_as_yaml<W: fmt::Write>(&self, s: &mut W) -> Result<(), Error> {
        for el in &self.streamers()? {
            writeln!(s, "{:#}", el.to_yaml())?;
        }
        Ok(())
    }

    /// Generate Rust code from the streamer info of this file
    pub fn streamer_info_as_rust<W: fmt::Write>(&self, s: &mut W) -> Result<(), Error> {
        // Add necessary imports at the top of the file
        writeln!(
            s,
            "{}",
            quote! {
                use std::marker::PhantomData;
                use nom::*;
                use parsers::*;
                use parsers::utils::*;
                use core_types::*;
            }
            .to_string()
        )?;

        // generate structs
        for el in &self.streamers()? {
            // The structs contain comments which introduce line breaks; i.e. readable
            writeln!(s, "{}", el.to_struct().to_string())?;
        }

        // generate parsers
        for el in &self.streamers()? {
            // The parsers have no comments, but are ugly; We introduce some
            // Linebreaks here to not have rustfmt choke later (doing it later
            // is inconvinient since the comments in the structs might contain
            // the patterns
            let parsers = el.to_named_parser().to_string();
            let parsers = parsers.replace(",", ",\n");
            let parsers = parsers.replace(">>", ">>\n");
            // macro names are generated as my_macro ! (...) by `quote`
            let parsers = parsers.replace(" ! (", "!(");
            writeln!(s, "{}", parsers)?;
        }
        Ok(())
    }
}

/// Use given parse on reader. If the initial `buf_size` is not large
/// enough it will increase it appropriately
fn parse_buffer<R, F, O>(reader: &mut BufReader<R>, buf_size: usize, f: F) -> Result<O, Error>
where
    R: Read + Seek,
    F: Fn(&[u8]) -> IResult<&[u8], O>,
{
    let buf = &mut vec![0; buf_size];
    let read_bytes = {
        match reader.read_exact(buf) {
            // We probably hit the end of the file; backup and read as much as possible
            Err(_) => reader.read_to_end(buf)?,
            _ => buf_size,
        }
    };
    match f(buf) {
        IResult::Done(_, parsed) => Ok(parsed),
        IResult::Incomplete(needed) => {
            // Reset seek position and try again with updated buf size
            reader.seek(SeekFrom::Current(-(read_bytes as i64)))?;
            match needed {
                Needed::Size(s) => parse_buffer(reader, s, f),
                _ => parse_buffer(reader, buf_size + 1000, f),
            }
        }
        IResult::Error(e) => {
            println!("{}", buf.to_hex(8));
            Err(format_err!("{:?}", e))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn file_header_test() {
        let path = "./src/test_data/simple.root";
        let f = File::open(path).unwrap();
        let mut reader = BufReader::new(f);

        let hdr = parse_buffer(&mut reader, 100, file_header).unwrap();
        let should = FileHeader {
            version: 60600,
            begin: 100,
            end: 5614,
            seek_free: 5559,
            nbytes_free: 55,
            n_entries_free: 1,
            n_bytes_name: 58,
            pointer_size: 4,
            compression: 1,
            seek_info: SeekFrom::Start(1117),
            nbytes_info: 4442,
            uuid: 409442932018821,
            seek_dir: SeekFrom::Start(158),
        };
        assert_eq!(hdr, should);
    }

    #[test]
    fn directory_test() {
        let path = "./src/test_data/simple.root";
        let f = File::open(path).unwrap();
        let mut reader = BufReader::new(f);
        let hdr = parse_buffer(&mut reader, 100, file_header).unwrap();
        reader.seek(hdr.seek_dir).unwrap();
        let dir = parse_buffer(&mut reader, 100, directory).unwrap();
        assert_eq!(
            dir,
            Directory {
                version: 5,
                c_time: 1418768412,
                m_time: 1418768412,
                n_bytes_keys: 96,
                n_bytes_name: 58,
                seek_dir: SeekFrom::Start(100),
                seek_parent: SeekFrom::Start(0),
                seek_keys: SeekFrom::Start(1021)
            }
        );
    }

    #[test]
    fn streamerinfo_test() {
        let path = PathBuf::from("./src/test_data/simple.root");
        let f = File::open(&path).unwrap();
        let mut reader = BufReader::new(f);
        // See file test
        reader.seek(SeekFrom::Start(1117)).unwrap();
        let key = parse_buffer(&mut reader, 4446, tkey).unwrap();
        assert_eq!(key.hdr.obj_name, "StreamerInfo");

        let key_len = key.hdr.key_len;
        let k_map_offset = 2;
        let context = Context {
            path: path.to_path_buf(),
            offset: (key_len + k_map_offset) as u64,
            s: key.obj.as_slice(),
        };

        match length_value!(
            key.obj.as_slice(),
            checked_byte_count,
            apply!(tlist, &context)
        ) {
            IResult::Done(_, l) => {
                assert_eq!(l.ver, 5);
                assert_eq!(l.name, "");
                assert_eq!(l.len, 19);
            }
            _ => panic!("Not parsed as TList!"),
        };
    }
}
