use std::fmt;

use failure::Error;
use nom::{
    self,
    bytes::complete::tag,
    combinator::map,
    number::complete::{be_i16, be_i32, be_u128, be_u16, be_u32, be_u64, be_u8},
    IResult,
};

use uuid::Uuid;

use crate::{
    code_gen::rust::{ToNamedRustParser, ToRustStruct},
    core::tstreamer::streamers,
    core::*,
    MAP_OFFSET,
};

/// Size of serialized `FileHeader` in bytes
const FILE_HEADER_SIZE: u64 = 75;

/// Size of serialized TDirectory. Depending on the ROOT version this
/// may use 32 or 64 bit pointers. This is the maximal (64 bit size).
const TDIRECTORY_MAX_SIZE: u64 = 42;

/// `RootFile` wraps the most basic information of a ROOT file.
#[derive(Debug)]
pub struct RootFile {
    source: Source,
    hdr: FileHeader,
    items: Vec<FileItem>,
}

#[derive(Debug, PartialEq)]
struct FileHeader {
    version: i32,
    begin: i32,
    end: u64,
    seek_free: u64,
    nbytes_free: i32,
    n_entries_free: i32,
    n_bytes_name: i32,
    pointer_size: u8,
    compression: i32,
    seek_info: SeekPointer,
    nbytes_info: i32,
    uuid: Uuid,
    seek_dir: SeekPointer,
}

#[derive(Debug, PartialEq)]
pub struct Directory {
    version: i16,
    c_time: u32,
    m_time: u32,
    n_bytes_keys: i32,
    n_bytes_name: i32,
    seek_dir: SeekPointer,
    seek_parent: SeekPointer,
    seek_keys: SeekPointer,
}

/// Parse opening part of a root file
fn file_header(i: &[u8]) -> IResult<&[u8], FileHeader> {
    fn version_dep_int(i: &[u8], is_64_bit: bool) -> IResult<&[u8], u64> {
        if is_64_bit {
            be_u64(i)
        } else {
            let (i, end) = be_u32(i)?;
            Ok((i, end as u64))
        }
    }
    let (i, _) = tag("root")(i)?;
    let (i, version) = be_i32(i)?;
    let is_64_bit = version > 1000000;
    let (i, begin) = be_i32(i)?;
    let (i, end) = version_dep_int(i, is_64_bit)?;
    let (i, seek_free) = version_dep_int(i, is_64_bit)?;
    let (i, nbytes_free) = be_i32(i)?;
    let (i, n_entries_free) = be_i32(i)?;
    let (i, n_bytes_name) = be_i32(i)?;
    let (i, pointer_size) = be_u8(i)?;
    let (i, compression) = be_i32(i)?;
    let (i, seek_info) = version_dep_int(i, is_64_bit)?;
    let (i, nbytes_info) = be_i32(i)?;
    let (i, _uuid_version) = be_u16(i)?;
    let (i, uuid) = be_u128(i)?;

    let uuid = Uuid::from_u128(uuid);
    let seek_dir = (begin + n_bytes_name) as u64;
    Ok((
        i,
        FileHeader {
            version,
            begin,
            end,
            seek_free,
            nbytes_free,
            n_entries_free,
            n_bytes_name,
            pointer_size,
            compression,
            seek_info,
            nbytes_info,
            uuid,
            seek_dir,
        },
    ))
}

/// Parse a file-pointer based on the version of the file
fn versioned_pointer(input: &[u8], version: i16) -> nom::IResult<&[u8], u64> {
    if version > 1000 {
        be_u64(input)
    } else {
        map(be_i32, |val| val as u64)(input)
    }
}

/// Directory within a root file; exists on ever file
fn directory(input: &[u8]) -> nom::IResult<&[u8], Directory> {
    let (input, version) = be_i16(input)?;
    let (input, c_time) = be_u32(input)?;
    let (input, m_time) = be_u32(input)?;
    let (input, n_bytes_keys) = be_i32(input)?;
    let (input, n_bytes_name) = be_i32(input)?;
    let (input, seek_dir) = versioned_pointer(input, version)?;
    let (input, seek_parent) = versioned_pointer(input, version)?;
    let (input, seek_keys) = versioned_pointer(input, version)?;
    Ok((
        input,
        Directory {
            version,
            c_time,
            m_time,
            n_bytes_keys,
            n_bytes_name,
            seek_dir,
            seek_parent,
            seek_keys,
        },
    ))
}

impl RootFile {
    /// Open a new ROOT file either from a `Url`, or from a `Path`
    /// (not available on `wasm32`).
    pub async fn new<S: Into<Source>>(source: S) -> Result<Self, Error> {
        let source = source.into();
        let hdr = source.fetch(0, FILE_HEADER_SIZE).await.and_then(|buf| {
            file_header(&buf)
                .map_err(|_| format_err!("Failed to parse file header"))
                .map(|(_i, o)| o)
        })?;
        // Jump to the TDirectory and parse it
        let dir = source
            .fetch(hdr.seek_dir, TDIRECTORY_MAX_SIZE)
            .await
            .and_then(|buf| {
                directory(&buf)
                    .map_err(|_| format_err!("Failed to parse TDirectory"))
                    .map(|(_i, o)| o)
            })?;
        let tkey_of_keys = source
            .fetch(dir.seek_keys, dir.n_bytes_keys as u64)
            .await
            .and_then(|buf| {
                tkey(&buf)
                    .map_err(|_| format_err!("Failed to parse TKeys"))
                    .map(|(_i, o)| o)
            })?;
        let keys = match tkey_headers(&tkey_of_keys.obj) {
            Ok((_, hdrs)) => Ok(hdrs),
            _ => Err(format_err!("Expected TKeyHeaders")),
        }?;
        let items = keys
            .iter()
            .map(|k_hdr| FileItem::new(k_hdr, source.clone()))
            .collect();

        Ok(RootFile { source, hdr, items })
    }

    pub async fn get_streamer_context(&self) -> Result<Context, Error> {
        let seek_info_len = (self.hdr.nbytes_info + 4) as u64;
        let info_key = self
            .source
            .fetch(self.hdr.seek_info, seek_info_len)
            .await
            .map(|buf| tkey(&buf).unwrap().1)?;

        let key_len = info_key.hdr.key_len;
        Ok(Context {
            source: self.source.clone(),
            offset: key_len as u64 + MAP_OFFSET,
            s: info_key.obj,
        })
    }

    /// Slice of the items contained in this file
    pub fn items(&self) -> &[FileItem] {
        &self.items
    }

    /// Translate the streamer info of this file to a YAML file
    pub async fn streamer_infos(&self) -> Result<Vec<TStreamerInfo>, Error> {
        let ctx = self.get_streamer_context().await?;
        let buf = ctx.s.as_slice();
        let (_, streamer_vec) =
            streamers(buf, &ctx).map_err(|_| format_err!("Failed to parse TStreamers"))?;
        Ok(streamer_vec)
    }

    /// Translate the streamer info of this file to a YAML file
    pub async fn streamer_info_as_yaml<W: fmt::Write>(&self, s: &mut W) -> Result<(), Error> {
        for el in &self.streamer_infos().await? {
            writeln!(s, "{:#}", el.to_yaml())?;
        }
        Ok(())
    }

    /// Generate Rust code from the streamer info of this file
    pub async fn streamer_info_as_rust<W: fmt::Write>(&self, s: &mut W) -> Result<(), Error> {
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
        )?;
        let streamer_infos = self.streamer_infos().await?;
        // generate structs
        for el in &streamer_infos {
            // The structs contain comments which introduce line breaks; i.e. readable
            writeln!(s, "{}", el.to_struct())?;
        }

        // generate parsers
        for el in &streamer_infos {
            // The parsers have no comments, but are ugly; We introduce some
            // Linebreaks here to not have rustfmt choke later (doing it later
            // is inconvinient since the comments in the structs might contain
            // the patterns
            let parsers = el.to_named_parser().to_string();
            let parsers = parsers.replace(',', ",\n");
            let parsers = parsers.replace(">>", ">>\n");
            // macro names are generated as my_macro ! (...) by `quote`
            let parsers = parsers.replace(" ! (", "!(");
            writeln!(s, "{}", parsers)?;
        }
        Ok(())
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod test {
    use super::*;
    use std::path::Path;

    use nom::multi::length_value;
    use reqwest::Url;
    use tokio;

    const SIMPLE_FILE_REMOTE: &str =
	"https://github.com/cbourjau/alice-rs/blob/master/root-io/src/test_data/simple.root?raw=true";

    #[tokio::test]
    async fn read_cms_file_remote() {
        let url = "http://opendata.web.cern.ch/eos/opendata/cms/hidata/HIRun2010/HIAllPhysics/RECO/ZS-v2/0000/001DA267-7243-E011-B38F-001617C3B6CE.root";
        let f = RootFile::new(Url::parse(url).unwrap()).await.unwrap();
        let mut s = String::new();
        f.streamer_info_as_yaml(&mut s).await.unwrap();
        println!("{}", s);
        for item in f.items() {
            item.as_tree().await.unwrap();
        }
    }

    async fn file_header_test(source: Source) {
        let hdr = source
            .fetch(0, FILE_HEADER_SIZE)
            .await
            .and_then(|buf| {
                file_header(&buf)
                    .map_err(|_| format_err!("Failed to parse file header"))
                    .map(|(_i, o)| o)
            })
            .unwrap();

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
            seek_info: 1117,
            nbytes_info: 4442,
            uuid: Uuid::from_u128(154703765255331693287451041600576143087),
            seek_dir: 158,
        };
        assert_eq!(hdr, should);
    }

    #[tokio::test]
    async fn file_header_test_local() {
        let local = Source::new(Path::new("./src/test_data/simple.root"));
        file_header_test(local).await;
    }

    #[tokio::test]
    async fn file_header_test_remote() {
        let remote = Source::new(Url::parse(SIMPLE_FILE_REMOTE).unwrap());
        file_header_test(remote).await;
    }

    async fn directory_test(source: Source) {
        let hdr = source
            .fetch(0, FILE_HEADER_SIZE)
            .await
            .and_then(|buf| {
                file_header(&buf)
                    .map_err(|_| format_err!("Failed to parse file header"))
                    .map(|(_i, o)| o)
            })
            .unwrap();

        let dir = source
            .fetch(hdr.seek_dir, TDIRECTORY_MAX_SIZE)
            .await
            .and_then(|buf| {
                directory(&buf)
                    .map_err(|_| format_err!("Failed to parse file header"))
                    .map(|(_i, o)| o)
            })
            .unwrap();
        assert_eq!(
            dir,
            Directory {
                version: 5,
                c_time: 1418768412,
                m_time: 1418768412,
                n_bytes_keys: 96,
                n_bytes_name: 58,
                seek_dir: 100,
                // TODO: This should probably be an Option
                seek_parent: 0,
                seek_keys: 1021
            }
        );
    }

    #[tokio::test]
    async fn directory_test_local() {
        let local = Path::new("./src/test_data/simple.root").into();
        directory_test(local).await;
    }

    #[tokio::test]
    async fn directory_test_remote() {
        let remote = Source::new(Url::parse(SIMPLE_FILE_REMOTE).unwrap());
        directory_test(remote).await;
    }

    async fn streamerinfo_test(source: Source) {
        let key = source
            .fetch(1117, 4446)
            .await
            .and_then(|buf| {
                tkey(&buf)
                    .map_err(|_| format_err!("Failed to parse file header"))
                    .map(|(_i, o)| o)
            })
            .unwrap();
        assert_eq!(key.hdr.obj_name, "StreamerInfo");

        let key_len = key.hdr.key_len;
        let k_map_offset = 2;
        let context = Context {
            source: source.clone(),
            offset: (key_len + k_map_offset) as u64,
            s: key.obj,
        };

        match length_value(checked_byte_count, |i| tlist(i, &context))(&context.s) {
            Ok((_, l)) => {
                assert_eq!(l.len(), 19);
            }
            Err(_e) => panic!("Not parsed as TList!"),
        };
    }

    #[tokio::test]
    async fn streamerinfo_test_local() {
        let local = Path::new("./src/test_data/simple.root").into();
        streamerinfo_test(local).await;
    }

    #[tokio::test]
    async fn streamerinfo_test_remote() {
        let remote = Url::parse(
	    "https://github.com/cbourjau/alice-rs/blob/master/root-io/src/test_data/simple.root?raw=true")
	    .unwrap()
	    .into();
        streamerinfo_test(remote).await;
    }
}
