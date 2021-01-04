use std::{io::SeekFrom, str::FromStr};
use strum_macros::EnumString;

#[derive(Debug, PartialEq, Default)]
pub(crate) struct Common {
    length: u64, // Byte length of this structure, including byte count of this variable
    class: u8,   // Structure class for this particular structure.
}

impl Common {
    pub fn new(length: u64, class: u8) -> Self {
        Common { length, class }
    }

    #[inline]
    pub const fn size_of() -> usize {
        14
    }

    #[inline]
    pub fn struct_length(&self) -> u64 {
        self.length - Self::size_of() as u64
    }

    #[inline]
    pub fn seek_past(&self) -> SeekFrom {
        SeekFrom::Current(self.struct_length() as i64)
    }

    #[inline]
    pub fn class(&self) -> u8 {
        self.class
    }
}

#[derive(Debug, PartialEq)]
pub struct FrameHeader {
    pub name: String,
    pub run: i32,
    pub frame: u32,
    // TODO(goller): split these up like Appendix A
    pub data_quality: u32,
    pub gps_start_time_s: u32,
    pub gps_residual_time_ns: u32,
    pub gps_leap_s: u16,
    pub frame_length_s: f64,
}

#[derive(Debug, PartialEq)]
pub struct ADC {
    pub name: String,
    pub comment: String,
    pub channel_group: u32,
    pub channel_number: u32,
    pub num_bits: u32,
    pub bias: f32,
    pub slope: f32,
    pub units: Option<String>,
    pub sample_rate: f64,
    pub time_offset_s: f64,
    pub f_shift: f64,
    pub phase: f32,
    pub data_valid: bool,
}

#[derive(Debug, PartialEq)]
pub struct Detector {
    pub name: String,    // TODO(this should be an enum)
    pub prefix: [i8; 2], // impl Detector fn prefix
    pub longitude_radians: f64,
    pub latitude_radians: f64,
    pub elevation_meters: f32,
    pub arm_x_azimuth_radians: f32,
    pub arm_y_azimuth_radians: f32,
    pub arm_x_altitude_radians: f32,
    pub arm_y_altitude_radians: f32,
    pub arm_x_midpoint_meters: f32,
    pub arm_y_midpoint_meters: f32,
    pub local_time_utc_offset_s: i32,
}

#[derive(Debug, PartialEq)]
pub struct Event {
    pub name: String,
    pub comment: String,
    pub inputs: String,
    pub gps_time_s: u32,
    pub gps_residual_time_ns: u32,
    pub duration_before_s: f32,
    pub duration_after_s: f32,
    pub event_status: u32,
    pub amplitude: f32,
    pub probability: Option<f32>,
    pub statistics: String,
    pub parameters: Vec<EventParameter>,
}

#[derive(Debug, PartialEq)]
pub struct EventParameter {
    pub value: f64,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct History {
    pub name: String,
    pub gps_time_s: u32,
    pub comment: String,
}

#[derive(Debug, PartialEq)]
pub struct Message {
    pub alarm: String,
    pub message: String,
    pub severity: u32,
    pub gps_time_s: u32,
    pub gps_residual_time_ns: u32,
}

#[derive(Debug, PartialEq)]
pub struct PostProcessed {
    pub name: String,
    pub comment: String,
    pub data_type: u16, // TODO(goller): turn this into an enum
    pub sub_type: u16,
    pub time_offset_s: f64,
    pub time_range_s: f64,
    pub f_shift: f64,
    pub phase: f32,
    pub frequency_range: f64,
    pub bandwidth: f64,
    pub auxiliary_parameters: Vec<AuxiliaryParameter>,
}

#[derive(Debug, PartialEq)]
pub struct AuxiliaryParameter {
    pub value: f64,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct RawData {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct Serial {
    pub name: String,
    pub gps_time_s: u32,
    pub gps_residual_time_ns: u32,
    pub sample_rate: f64,
    pub data: String,
}

#[derive(Debug, PartialEq)]
pub struct Simulation {
    pub name: String,
    pub comment: String,
    pub sample_rate: f64,
    pub time_offset_s: f64,
    pub f_shift: f64,
    pub phase: f32,
}

#[derive(Debug, PartialEq)]
pub struct SimulatedEvent {
    pub name: String,
    pub comment: String,
    pub inputs: String,
    pub gps_event_max_time_s: u32,
    pub gps_residual_time_ns: u32,
    pub duration_before_s: f32,
    pub duration_after_s: f32,
    pub amplitude: f32,
    pub parameters: Vec<EventParameter>,
}

#[derive(Debug, PartialEq)]
pub struct StaticData {
    pub name: String,
    pub comment: String,
    pub representation: String,
    pub gps_time_start_s: u32,
    pub gps_time_end_s: u32,
    pub version: u32,
}

#[derive(Debug, PartialEq)]
pub struct Summary {
    pub name: String,
    pub comment: String,
    pub test: String,
    pub gps_time_s: u32,
    pub gps_residual_time_ns: u32,
}

#[derive(Debug, PartialEq)]
pub struct Table {
    pub name: String,
    pub comment: String,
    pub num_rows: u32,
    pub column_names: Vec<String>, // would be nice to have a 2-d table;
}

#[derive(Debug, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imaginary: f64,
}

#[derive(Debug, PartialEq)]
pub struct VectorInfo {
    pub name: String,
    pub num_samples: u64,
    pub num_dimensions: u32,
    pub dimension_lengths: Vec<u64>,
    pub sample_spacing: Vec<f64>,
    pub x_origins: Vec<f64>,
    pub unit_x_scale_factors: Vec<String>,
    pub unit_y: String,
}

#[derive(Debug, PartialEq)]
pub enum Vector {
    I8(VectorInfo, Vec<i8>),
    U8(VectorInfo, Vec<u8>),
    I16(VectorInfo, Vec<i16>),
    U16(VectorInfo, Vec<u16>),
    I32(VectorInfo, Vec<i32>),
    U32(VectorInfo, Vec<u32>),
    U64(VectorInfo, Vec<u64>),
    I64(VectorInfo, Vec<i64>),

    F32(VectorInfo, Vec<f32>),
    F64(VectorInfo, Vec<f64>),

    Strings(VectorInfo, Vec<String>),
    Complexes(VectorInfo, Vec<Complex>),
}

/// Version is the version of the GWF file.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Version {
    pub major: Major,
    pub minor: Minor,
}

impl From<[u8; 2]> for Version {
    fn from(v: [u8; 2]) -> Version {
        Version {
            major: Major::from(v[0]),
            minor: Minor::from(v[1]),
        }
    }
}

/// Major is the Frame library major version used to write this frame file.
/// Unsupported means that this software does not know how to parse the file.
/// Unsupported will return the value reported in the file.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Major {
    Release4,
    Release6,
    Release8,
    Unsupported(u8),
}

impl From<u8> for Major {
    fn from(m: u8) -> Major {
        match m {
            4 => Major::Release4,
            6 => Major::Release6,
            8 => Major::Release8,
            _ => Major::Unsupported(m),
        }
    }
}

/// Minor is the Frame library minor version used to write this frame file.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Minor {
    Minor(u8),
    /// Beta represents an unreleased or provisional version.
    Beta,
}

impl From<u8> for Minor {
    fn from(m: u8) -> Minor {
        match m {
            255 => Minor::Beta,
            _ => Minor::Minor(m),
        }
    }
}

#[derive(Debug, PartialEq)]
/// Checksum is the checksum type recorded in the end-of-file structure.
pub enum Checksum {
    SumNone,
    /// SumCRC indicates there is a POSIX.2 checksum.
    SumCRC,
}

impl From<u8> for Checksum {
    fn from(c: u8) -> Checksum {
        if c == 1 {
            return Checksum::SumCRC;
        }
        Checksum::SumNone
    }
}

impl Default for Checksum {
    fn default() -> Self {
        Checksum::SumNone
    }
}

/// Endian describes the endianness of the file.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Endian {
    Big,
    Little,
}

impl From<[u8; 2]> for Endian {
    fn from(e: [u8; 2]) -> Endian {
        match e {
            [0x12, 0x34] => Endian::Big,
            [0x34, 0x12] => Endian::Little,
            _ => Endian::Little,
        }
    }
}

impl Default for Endian {
    fn default() -> Self {
        Endian::Big
    }
}

/// Library indicates the software that produced the GWF file.
#[derive(Debug, PartialEq)]
pub enum Library {
    /// Library::LibUnknown indicates the GWF file was not producted by the C or CPP frame libraries.
    /// It's value is the byte from the file itself.
    LibUnknown(u8),
    /// LibFrameL is the C frame library. http://lappweb.in2p3.fr/virgo/FrameL/
    LibFrameL,
    /// LibFrameCPP is the CPP frame library.  http://software.ligo.org/lscsoft/source/
    LibFrameCPP,
}

impl From<u8> for Library {
    fn from(l: u8) -> Library {
        match l {
            1 | b'A' => Library::LibFrameL, // version 6 of the C library would use ascii 'A'.
            2 => Library::LibFrameCPP,
            _ => Library::LibUnknown(l),
        }
    }
}

impl Default for Library {
    fn default() -> Self {
        Library::LibUnknown(0)
    }
}

#[derive(Debug, PartialEq)]
/// Header is the header of a GWF file containing metadata, notably version, endianness, and checksums
pub struct Header {
    pub version: Version,
    pub library: Library,
    pub endian: Endian,
    pub machine_data_model: data_models::DataModel,
    pub checksum: Checksum,
}

impl Header {
    pub const fn size_of() -> usize {
        40
    }
}

#[derive(EnumString, Copy, Clone, PartialEq)]
#[repr(u8)]
pub(crate) enum Structures {
    StructureUnknown = 0,
    FrSH = 1,     // 4.3.2.1: dictionary header structure; always 1 in gwf.
    FrSE = 2,     // 4.3.2.2: describes fields within dictionaries; always 2 in gwf.
    FrameH,       // 4.3.2.3:
    FrAdcData,    // 4.3.2.4:
    FrDetector,   // 4.3.2.5:
    FrEndOfFile,  // 4.3.2.6:
    FrEndOfFrame, // 4.3.2.7:
    FrEvent,      // 4.3.2.8:
    FrHistory,    // 4.3.2.9:
    FrMsg,        // 4.3.2.10:
    FrProcData,   // 4.3.2.11:
    FrRawData,    // 4.3.2.12:
    FrSerData,    // 4.3.2.13:
    FrSimData,    // 4.3.2.14:
    FrSimEvent,   // 4.3.2.15:
    FrStatData,   // 4.3.2.16:
    FrSummary,    // 4.3.2.17:
    FrTable,      // 4.3.2.18:
    FrTOC,        // 4.3.2.19
    FrVect,       // 4.3.2.20
}

use std::{collections::HashMap, error::Error};

// StructureLookup caches the file's internal mapping of structure strings to
// class IDs.  Each file can have different mappings and this allows the code
// to have stable names.
pub(crate) struct StructureLookup {
    lookup: HashMap<u8, Structures>,
}

impl StructureLookup {
    pub(crate) fn new() -> StructureLookup {
        // FrSH and FrSE are always 1 & 2 according to specification, 4.3.2
        let mut s = StructureLookup {
            lookup: HashMap::new(),
        };
        s.lookup.insert(1, Structures::FrSH);
        s.lookup.insert(2, Structures::FrSE);
        s
    }

    pub(crate) fn insert(
        &mut self,
        name: &str,
        id: u16,
    ) -> ::std::result::Result<(), Box<dyn Error>> {
        if id > u8::MAX as u16 {
            return std::result::Result::Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "invalid id: out of range",
            )));
        }
        let r = Structures::from_str(&name);
        match r {
            Err(e) => ::std::result::Result::Err(Box::new(e)),
            Ok(s) => {
                self.lookup.insert(id as u8, s);
                Ok(())
            }
        }
    }

    pub(crate) fn structure(&self, id: u8) -> Structures {
        match self.lookup.get(&id) {
            Some(s) => *s,
            None => Structures::StructureUnknown,
        }
    }
}

#[cfg(test)]
mod struct_tests {
    use std::io::SeekFrom;

    use super::*;
    #[test]
    fn test_common_size() {
        assert_eq!(Common::size_of(), 14);
    }

    #[test]
    fn test_struct_length() {
        let common = Common::new(78, 1);
        assert_eq!(common.struct_length(), 64);
    }

    #[test]
    fn test_seek_past() {
        let common = Common::new(78, 1);
        assert_eq!(common.seek_past(), SeekFrom::Current(64));
    }
}
