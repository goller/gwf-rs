use crate::structures::*;
use std::io::{Read, Seek};

const MAGIC: [u8; 5] = *b"IGWD\0";

// TODO(goller): convert this to use Result
pub(crate) fn parse<R: Read + Seek>(reader: &mut R) -> Option<Header> {
    let mut val = [0; Header::size_of()];
    reader.read_exact(&mut val).ok()?;
    if val[..5] != MAGIC {
        // TODO(goller): figure out how we want to do errors
    }

    Some(Header {
        version: Version::from([val[5], val[6]]),
        library: Library::from(val[38]),
        endian: Endian::from([val[12], val[13]]),
        machine_data_model: data_models::DataModel::new(
            val[8] as usize,
            val[9] as usize,
            val[9] as usize,
        ),
        checksum: Checksum::from(val[39]),
    })
}

#[cfg(test)]
mod header_tests {
    use super::*;
    use std::fs::File;

    fn check_header(filename: &str, expected: Header) {
        let mut file = File::open(filename).expect("unable to open file");
        let header = parse(&mut file).unwrap();
        assert_eq!(header, expected);
    }

    #[test]
    fn test_f_headers() {
        for file in [
            "assets/F-TEST-600000000-60.gwf",
            "assets/F-TEST-600000060-60.gwf",
            "assets/F-TEST-600000120-60.gwf",
        ]
        .iter_mut()
        {
            check_header(
                file,
                Header {
                    version: Version {
                        major: Major::Release6,
                        minor: Minor::Minor(20),
                    },
                    library: Library::LibFrameL,
                    endian: Endian::Big,
                    machine_data_model: data_models::DataModel::LP64,
                    checksum: Checksum::SumNone,
                },
            );
        }
    }

    #[test]
    fn test_calibration_headers() {
        for file in [
            "assets/H-CAL_FAC_V03-729273600-5094000.gwf",
            "assets/H-CAL_REF_V03-734073939-64.gwf",
            "assets/L-CAL_FAC_V03-729273600-5094000.gwf",
            "assets/L-CAL_REF_V03-731488397-64.gwf",
        ]
        .iter_mut()
        {
            check_header(
                file,
                Header {
                    version: Version {
                        major: Major::Release6,
                        minor: Minor::Minor(6),
                    },
                    library: Library::LibFrameL,
                    endian: Endian::Little,
                    machine_data_model: data_models::DataModel::LP64,
                    checksum: Checksum::SumNone,
                },
            );
        }
    }

    #[test]
    fn test_pycbc_headers() {
        for file in [
            "assets/PyCBC_T2_0.gwf",
            "assets/PyCBC_T2_1.gwf",
            "assets/PyCBC_T2_2.gwf",
            "assets/PyCBC_T3_0.gwf",
        ]
        .iter_mut()
        {
            check_header(
                file,
                Header {
                    version: Version {
                        major: Major::Release8,
                        minor: Minor::Minor(1),
                    },
                    library: Library::LibFrameCPP,
                    endian: Endian::Little,
                    machine_data_model: data_models::DataModel::LP64,
                    checksum: Checksum::SumCRC,
                },
            );
        }
    }
}
