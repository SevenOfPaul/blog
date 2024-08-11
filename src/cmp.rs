use flate2::Compression;
use flate2::write::GzEncoder;
use std::io::prelude::*;
use flate2::bufread::GzDecoder;

pub fn compress(str: String) -> Result<Vec<u8>, std::io::Error> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(str.as_bytes())?;
    Ok(encoder.finish()?)
}
pub fn de_compress(compressed:Vec<u8>) -> Result<String, std::io::Error> {
    let mut decompressor = GzDecoder::new(&compressed[..]);
    let mut decompressed_data = String::new();
    decompressor.read_to_string(&mut decompressed_data)?;
    Ok(decompressed_data)
}