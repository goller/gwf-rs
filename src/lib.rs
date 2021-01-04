#![feature(test)]
#![feature(vec_into_raw_parts)]

use std::io::{Error, ErrorKind, Read, Result, Seek};
use std::{fs::File, io::BufReader};

pub mod handler;
pub mod structures;

mod decompress;
mod endian;
mod header;
mod parser;
mod strings;
mod v6;
mod v8;

pub fn parse_file<T: handler::Handler>(filename: &str, handler: &mut T) -> Result<()> {
    let file = File::open(filename)?;
    let mut reader = BufReader::with_capacity(16 * 1024, file);
    parse(&mut reader, handler)
}

pub fn parse<R, T>(reader: &mut R, handler: &mut T) -> Result<()>
where
    R: Read + Seek,
    T: handler::Handler,
{
    // TODO(goller): handle error
    let hdr = match header::parse(reader) {
        Some(h) => h,
        None => {
            return Err(Error::new(
                ErrorKind::Other,
                "unknown file type".to_string(),
            ))
        }
    };

    if let Some(version) = handler.version() {
        version(handler, hdr.version);
    }

    match hdr.version.major {
        structures::Major::Release8 | structures::Major::Release6 => {
            let mut p = parser::Parser::new();
            Ok(p.parse(&hdr, reader, handler)?)
        }
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use self::test::Bencher;
    struct NopHandler {}
    impl handler::Handler for NopHandler {}

    #[bench]
    fn bench_gw_open_data_workshop_t3_0(b: &mut Bencher) {
        let filename = "assets/PyCBC_T3_0.gwf";
        let mut file = File::open(filename).expect("unable to open file");
        let mut handler = NopHandler {};
        b.iter(|| {
            parse(&mut file, &mut handler).unwrap();
            file.seek(std::io::SeekFrom::Start(0)).unwrap();
        });
    }
}
