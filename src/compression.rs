
use bzip2;
use flate2;
use std::io::{Error, ErrorKind};

const COMPRESSION_HUFFMAN:      u8 = 0x01;
const COMPRESSION_ZLIB:         u8 = 0x02;
const COMPRESSION_PKZIP:        u8 = 0x08;
const COMPRESSION_BZIP2:        u8 = 0x10;
const COMPRESSION_SPARSE:       u8 = 0x20;
const COMPRESSION_ADPCM_MONO:   u8 = 0x40;
const COMPRESSION_ADPCM_STEREO: u8 = 0x80;
const COMPRESSION_LZMA:         u8 = 0x12;

pub fn decompress(data: &mut [u8], out: &mut [u8]) -> Result<usize, Error> {
    let compression_type = data[0];

    if compression_type & COMPRESSION_BZIP2 != 0 {
        let mut decompress = bzip2::Decompress::new(true);

        match decompress.decompress(&data[1..], out) {
            Ok(_) => {},
            Err(e) => return Err(Error::new(ErrorKind::Other, e))
        }

        return Ok(decompress.total_out() as usize);
    }

    if compression_type & COMPRESSION_ZLIB != 0 {
        let mut zlib = flate2::Decompress::new(true);

        match zlib.decompress(&data[1..], out, flate2::Flush::None) {
            Ok(_) => {},
            Err(e) => return Err(Error::new(ErrorKind::Other, e))
        }

        return Ok(zlib.total_out() as usize);
    }

    if compression_type & COMPRESSION_PKZIP != 0 {
        println!("FixMe: COMPRESSION_PKZIP");
    }

    if compression_type & COMPRESSION_HUFFMAN != 0 {
        println!("FixMe: COMPRESSION_HUFFMAN");
    }

    if compression_type & COMPRESSION_SPARSE != 0 {
        println!("FixMe: COMPRESSION_SPARSE");
    }

    if compression_type & COMPRESSION_ADPCM_STEREO != 0 {
        println!("FixMe: COMPRESSION_ADPCM_STEREO");
    }

    if compression_type & COMPRESSION_ADPCM_MONO != 0 {
        println!("FixMe: COMPRESSION_ADPCM_MONO");
    }

    if compression_type & COMPRESSION_LZMA != 0 {
       println!("FixMe: COMPRESSION_LZMA");
    }

    Err(Error::new(ErrorKind::Other, "No compression type found"))
}
