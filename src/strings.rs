use crate::endian::*;
use crate::structures::*;
use std::ffi::CStr;
use std::io::Read;

/// the structures of strings in GWF is 2 bytes of length followed by a null-terminated string.
pub(crate) fn string<R: Read>(header: &Header, reader: &mut R) -> std::io::Result<String> {
    let mut len_buf = [0; core::mem::size_of::<u16>()];
    reader.read_exact(&mut len_buf)?;
    let len = u16::from_bytes(&len_buf, header.endian);

    let mut name_buf = vec![0; len as usize];
    reader.read_exact(&mut name_buf.as_mut_slice())?;
    // TODO(goller): figure out errors
    let res = CStr::from_bytes_with_nul(&name_buf);
    match res {
        Ok(s) => Ok(s.to_string_lossy().to_string()),
        Err(e) => std::result::Result::Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        )),
    }
}

#[cfg(test)]
mod string_tests {
    use super::*;
    use std::fs::File;
    use std::io::Seek;

    #[test]
    fn test_string() -> std::io::Result<()> {
        let filename = "assets/F-TEST-600000000-60.gwf";
        let mut file = File::open(filename).expect("unable to open file");
        file.seek(std::io::SeekFrom::Start(0x59))?;
        {
            let mut reader = file.by_ref().take(9);

            let header = Header {
                version: Version {
                    major: Major::Release6,
                    minor: Minor::Minor(20),
                },
                library: Library::LibFrameL,
                endian: Endian::Big,
                machine_data_model: data_models::DataModel::LP64,
                checksum: Checksum::SumNone,
            };

            let frame_h = string(&header, &mut reader)?;
            assert_eq!(frame_h, "STRING");
        }

        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        assert_eq!(buf.len(), 3941577);
        Ok(())
    }
}
