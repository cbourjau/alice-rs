use flate2::bufread::ZlibDecoder;
use lzma_rs::xz_decompress;
use thiserror::Error;

use std::io::Read;
use std::*;
use DecompressionError::*;

#[derive(Error, Debug)]
pub enum DecompressionError {
    #[error("Header too short")]
    InsufficientData,
    #[error("Compression algorithm '{0}' not supported")]
    AlgorithmNotImplemented(String),
    #[error("Failed to decompress LZMA section")]
    LzmaFailure(#[from] lzma_rs::error::Error),
    #[error("Failed to decompress LZ4 section")]
    Lz4Failure,
    #[error("Failed to decompress ZLib section")]
    ZLibFailure(#[from] std::io::Error),
}

pub(crate) fn decompress(input: &[u8]) -> Result<Vec<u8>, DecompressionError> {
    if input.len() < 9 {
        return Err(InsufficientData);
    }

    // There is something in bytes 2..=8, but we haven't identified it yet
    let magic = &input[..2];
    let compressed = &input[9..];

    let mut ret = vec![];

    match magic {
        b"ZL" => {
            let mut decoder = ZlibDecoder::new(compressed);
            decoder.read_to_end(&mut ret)?;
            Ok(ret)
        }
        b"XZ" => {
            let mut reader = std::io::BufReader::new(compressed);
            xz_decompress(&mut reader, &mut ret)?;
            Ok(ret)
        }
        b"L4" => {
            // TODO checksum verification?
            // skip leading u64
            lz4_compress::decompress(&compressed[8..]).map_err(|_| Lz4Failure)
        }
        other => Err(AlgorithmNotImplemented(
            String::from_utf8(other.to_vec()).unwrap_or(format!("Bad magic {other:?}")),
        )),
    }
}
