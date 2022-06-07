use nom::{self,
          number::complete::{be_i16, be_i32, be_u128, be_u16, be_u32, be_u64, be_u8}, Parser};
use nom::sequence::tuple;
use nom_supreme::{ParserExt, tag::complete::tag};
use uuid::Uuid;

use std::fmt;

use crate::{
    code_gen::rust::{ToNamedRustParser, ToRustStruct},
    core::*,
    core::tstreamer::streamers,
    MAP_OFFSET,
};
use crate::core::{ReadError, WriteError};

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
fn file_header<'s, E: RootError<Span<'s>>>(i: Span<'s>) -> RResult<'s, FileHeader, E> {
    let parser = |i| {
        fn version_dep_int<'s, E: RootError<Span<'s>>>(i: Span<'s>, is_64_bit: bool) -> RResult<'s, u64, E> {
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
    };
    parser.context("file header").parse(i)
}

/// Parse a file-pointer based on the version of the file
fn versioned_pointer<'s, E>(version: i16) -> impl RParser<'s, u64, E>
    where
        E: RootError<Span<'s>>
{
    move |i| {
        if version > 1000 {
            be_u64.parse(i)
        } else {
            be_u32.map(|v| v as u64).parse(i)
        }
    }
}

/// Directory within a root file; exists on ever file
fn directory<'s, E>(input: Span<'s>) -> RResult<'s, Directory, E>
    where
        E: RootError<Span<'s>>
{
    tuple((
        be_i16.context("directory version"),
        be_u32.context("directory time created"),
        be_u32.context("directory time modified"),
        be_i32.context("directory key byte count"),
        be_i32.context("directory name byte count")
    )).flat_map(make_fn(|(version, c_time, m_time, n_bytes_keys, n_bytes_name)| {
        tuple((
            versioned_pointer(version).context("seek dir"),
            versioned_pointer(version).context("seek parent"),
            versioned_pointer(version).context("seek keys")
        )).map(move |(seek_dir, seek_parent, seek_keys)|
            Directory {
                version,
                c_time,
                m_time,
                n_bytes_keys,
                n_bytes_name,
                seek_dir,
                seek_parent,
                seek_keys,
            })
    })).context("ROOT directory").parse(input)
}


impl RootFile {
    /// Open a new ROOT file either from a `Url`, or from a `Path`
    /// (not available on `wasm32`).
    pub async fn new<S: Into<Source>>(source: S) -> Result<Self, ReadError> {
        let source = source.into();
        let hdr_buf = source.fetch(0, FILE_HEADER_SIZE).await?;
        let hdr = wrap_parser(file_header.context("file header"))(&hdr_buf)?;
        //let hdr = _hdr?;

        // Jump to the TDirectory and parse it
        let dir_buf = source.fetch(hdr.seek_dir, TDIRECTORY_MAX_SIZE).await?;
        let dir = wrap_parser(directory)(&dir_buf)?;

        let tkey_buf = source.fetch(dir.seek_keys, dir.n_bytes_keys as u64).await?;
        let tkey_of_keys = wrap_parser(tkey.all_consuming().context("root file key listing"))(&tkey_buf)?;

        let keys = wrap_parser(tkey_headers.context("root file keys"))(&tkey_of_keys.obj)?;

        let items = keys
            .iter()
            .map(|k_hdr| FileItem::new(k_hdr, source.clone()))
            .collect();

        Ok(RootFile { source, hdr, items })
    }

    pub async fn get_streamer_context(&self) -> Result<Context, ReadError> {
        let seek_info_len = (self.hdr.nbytes_info) as u64;
        let info_key_buf = self.source
            .fetch(self.hdr.seek_info, seek_info_len)
            .await?;
        let info_key = wrap_parser(tkey.all_consuming().context("streamer info key"))(&info_key_buf)?;
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

    /// Get the stream info of this file
    pub async fn streamer_infos(&self) -> Result<Vec<TStreamerInfo>, ReadError> {
        let ctx = self.get_streamer_context().await?;
        let res = wrap_parser_ctx(streamers)(&ctx)?;
        Ok(res)
    }

    /// Translate the streamer info of this file to a YAML file
    pub async fn streamer_info_as_yaml<W: fmt::Write>(&self, s: &mut W) -> Result<(), WriteError> {
        for el in &self.streamer_infos().await? {
            writeln!(s, "{:#}", el.to_yaml())?;
        }
        Ok(())
    }

    /// Generate Rust code from the streamer info of this file
    pub async fn streamer_info_as_rust<W: fmt::Write>(&self, s: &mut W) -> Result<(), WriteError> {
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
    use nom::error::VerboseError;
    use nom::multi::length_value;
    use reqwest::Url;
    use tokio;

    use std::path::Path;
    use crate::core::ReadError;

    use super::*;
    use self::UnwrapPrint;

    const SIMPLE_FILE_REMOTE: &str =
        "https://github.com/cbourjau/alice-rs/blob/master/root-io/src/test_data/simple.root?raw=true";

    #[tokio::test]
    async fn read_cms_file_remote() {
        let url = "http://opendata.web.cern.ch/eos/opendata/cms/hidata/HIRun2010/HIAllPhysics/RECO/ZS-v2/0000/001DA267-7243-E011-B38F-001617C3B6CE.root";
        let f = RootFile::new(Url::parse(url).unwrap()).await.unwrap_print();
        let mut s = String::new();
        f.streamer_info_as_yaml(&mut s).await.unwrap();
        println!("{}", s);
        for item in f.items() {
            item.as_tree().await.unwrap_print();
        }
    }

    async fn file_header_test(source: Source) -> Result<(), ReadError> {
        let buf = source.fetch(0, FILE_HEADER_SIZE).await?;

        let hdr = match wrap_parser(file_header)(&buf) {
            Ok(hdr) => hdr,
            Err(e)  => return Err(ReadError::ParseError(e))
        };

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

        Ok(())
    }

    #[tokio::test]
    async fn file_header_test_local() {
        let local = Source::new(Path::new("./src/test_data/simple.root"));
        file_header_test(local).await.unwrap_print();
    }

    #[tokio::test]
    async fn file_header_test_remote() {
        let remote = Source::new(Url::parse(SIMPLE_FILE_REMOTE).unwrap());
        file_header_test(remote).await.unwrap_print();
    }

    async fn directory_test(source: Source) -> Result<(), ReadError> {
        let hdr_buf = source.fetch(0, FILE_HEADER_SIZE).await?;
        let hdr = wrap_parser(file_header)(&hdr_buf)?;

        let dir_buf = source.fetch(hdr.seek_dir, TDIRECTORY_MAX_SIZE).await?;
        let dir = wrap_parser(directory)(&dir_buf)?;

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

        Ok(())
    }

    #[tokio::test]
    async fn directory_test_local() {
        let local = Path::new("./src/test_data/simple.root").into();
        directory_test(local).await.unwrap_print();
    }

    #[tokio::test]
    async fn directory_test_remote() {
        let remote = Source::new(Url::parse(SIMPLE_FILE_REMOTE).unwrap());
        directory_test(remote).await.unwrap_print();
    }

    async fn streamerinfo_test(source: Source) -> Result<(), ReadError> {
        let buf = source.fetch(1117, 4446).await?;
        let key = wrap_parser(tkey)(&buf)?;

        assert_eq!(key.hdr.obj_name, "StreamerInfo");

        let key_len = key.hdr.key_len;
        let k_map_offset = 2;
        let context = Context {
            source: source.clone(),
            offset: (key_len + k_map_offset) as u64,
            s: key.obj,
        };

        let mut tlist_parser = wrap_parser_ctx(|ctx| {
            length_value(checked_byte_count, move |i| {
                tlist::<VerboseError<_>>(&ctx).parse(i)
            }).all_consuming()
        });

        let tlist = tlist_parser(&context)?;
        assert_eq!(tlist.len(), 19);

        Ok(())
    }

    #[tokio::test]
    async fn streamerinfo_test_local() {

        let local = Path::new("./src/test_data/simple.root").into();
        streamerinfo_test(local).await.unwrap_print();
    }

    #[tokio::test]
    async fn streamerinfo_test_remote() {
        let remote = Url::parse(
	    "https://github.com/cbourjau/alice-rs/blob/master/root-io/src/test_data/simple.root?raw=true")
	    .unwrap()
	    .into();
        streamerinfo_test(remote).await.unwrap_print();
    }
}
