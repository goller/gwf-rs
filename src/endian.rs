use byteorder::{BigEndian, ByteOrder, LittleEndian};
use std::convert::TryInto;
use std::io::Read;

use crate::structures::*;

pub(crate) trait EndianRead {
    type Array;
    fn read_into<R>(
        array: &mut Self::Array,
        reader: &mut R,
        endian: Endian,
    ) -> std::io::Result<Self>
    where
        R: Read,
        Self: Sized;
    fn from_bytes(bytes: &[u8], endian: Endian) -> Self;
}

macro_rules! impl_EndianRead_for_numbers (( $($int:ident),* ) => {
    $(
        impl EndianRead for $int {
            type Array = [u8; core::mem::size_of::<Self>()];

            fn read_into<R: Read> (array: &mut Self::Array, reader: &mut R, endian: Endian) -> std::io::Result<Self>{
                reader.read_exact(array)?;
                match endian {
                    Endian::Little => Ok(Self::from_le_bytes(*array)),
                    Endian::Big => Ok(Self::from_be_bytes(*array)),
                }
            }
            fn from_bytes(bytes: &[u8], endian: Endian) -> Self {
                assert!(bytes.len() == core::mem::size_of::<Self>());
                match endian {
                    Endian::Little => Self::from_le_bytes(bytes.try_into().unwrap()),
                    Endian::Big => Self::from_be_bytes(bytes.try_into().unwrap()),
                }
            }
        }
    )*
});

impl_EndianRead_for_numbers!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);

pub(crate) fn read_into_slice_u64(src: &[u8], dst: &mut [u64], endian: Endian) {
    match endian {
        Endian::Little => LittleEndian::read_u64_into(src, dst),
        Endian::Big => BigEndian::read_u64_into(src, dst),
    }
}

pub(crate) fn read_into_slice_f32(src: &[u8], dst: &mut [f32], endian: Endian) {
    match endian {
        Endian::Little => LittleEndian::read_f32_into(src, dst),
        Endian::Big => BigEndian::read_f32_into(src, dst),
    }
}

pub(crate) fn read_into_slice_f64(src: &[u8], dst: &mut [f64], endian: Endian) {
    match endian {
        Endian::Little => LittleEndian::read_f64_into(src, dst),
        Endian::Big => BigEndian::read_f64_into(src, dst),
    }
}

macro_rules! transmute_vec_ints {
    ($src:expr, $ty:ty, $endian:expr) => {{
        use std::mem;
        // https://users.rust-lang.org/t/safe-way-to-convert-vec-u8-of-len-n-4-to-a-vec-rgba-of-len-n/36116/5
        // make sure to set capacity to len
        let slice: Box<[u8]> = $src.into();
        let v: Vec<u8> = slice.into();

        const K: usize = mem::size_of::<$ty>() / mem::size_of::<u8>();

        assert_eq!(v.len() % K, 0);
        assert_eq!(v.capacity() % K, 0);

        let (ptr, len, cap): (*mut u8, _, _) = v.into_raw_parts();
        let mut u = unsafe {
            // # Safety
            //
            //  - It is safe to transmute between `[u8; K]` and `Vec<U>`,
            //   - Guaranteed the same memory alignment and that the capacity % K is 0.
            //
            //   - `Layout::<[u8; K * N]>::new() == Layout::<[U; N]>::new()`
            Vec::from_raw_parts(ptr.cast::<$ty>(), len / K, cap / K)
        };

        match $endian {
            Endian::Big => {
                for v in u.iter_mut() {
                    *v = v.to_be();
                }
            },
            Endian::Little => {
                for v in u.iter_mut() {
                    *v = v.to_le();
                }
            },
        }

        u
    }};
}

pub(crate) trait TransmuteVec {
    fn transmute(src: Vec<u8>, endian: Endian) -> Self;
}

macro_rules! impl_TransmuteVec_for_numbers (( $($int:ident),* ) => {
    $(
        impl TransmuteVec for Vec<$int> {
            fn transmute(src: Vec<u8>, endian: Endian) -> Self {
                transmute_vec_ints!(src, $int, endian)
            }
        }
    )*
});

impl_TransmuteVec_for_numbers!(u16, i16, u32, i32, u64, i64);

/// f64 are special in that endian conversions go through u64 first.
impl TransmuteVec for Vec<f64> {
    fn transmute(src: Vec<u8>, endian: Endian) -> Self {
        let u = Vec::<u64>::transmute(src, endian);

        let (ptr, len, cap): (*mut u64, _, _) = u.into_raw_parts();
        let u = unsafe {
            // # Safety
            //
            //  - converting from u64 endian converted bits to f64 is safe.
            Vec::from_raw_parts(ptr.cast::<f64>(), len, cap)
        };

        u
    }
}

/// f32 are special in that endian conversions go through u32 first.
impl TransmuteVec for Vec<f32> {
    fn transmute(src: Vec<u8>, endian: Endian) -> Self {
        let u = Vec::<u32>::transmute(src, endian);

        let (ptr, len, cap): (*mut u32, _, _) = u.into_raw_parts();
        let u = unsafe {
            // # Safety
            //
            //  - converting from u32 endian converted bits to f32 is safe.
            Vec::from_raw_parts(ptr.cast::<f32>(), len, cap)
        };

        u
    }
}

#[cfg(test)]
mod endian_tests {
    use super::*;
    #[test]
    fn test_u8_endian() {
        let got = u8::from_bytes(&[0xFF], Endian::Big);
        assert_eq!(got, 255);
        let got = u8::from_bytes(&[0xFF], Endian::Little);
        assert_eq!(got, 255);
    }

    #[test]
    fn test_i8_endian() {
        let got = i8::from_bytes(&[0xFF], Endian::Big);
        assert_eq!(got, -1);
        let got = i8::from_bytes(&[0xFF], Endian::Little);
        assert_eq!(got, -1);
    }

    #[test]
    fn test_u16_endian() {
        let got = u16::from_bytes(&[0x12, 0x34], Endian::Big);
        assert_eq!(got, 0x1234);
        let got = u16::from_bytes(&[0x12, 0x34], Endian::Little);
        assert_eq!(got, 0x3412);
    }

    #[test]
    fn test_i16_endian() {
        let got = i16::from_bytes(&[0xFF, 0x00], Endian::Big);
        assert_eq!(got, -256);
        let got = i16::from_bytes(&[0x00, 0xFF], Endian::Little);
        assert_eq!(got, -256);
    }

    #[test]
    fn test_u32_endian() {
        let got = u32::from_bytes(&[0x12, 0x34, 0x56, 0x78], Endian::Big);
        assert_eq!(got, 0x12345678);
        let got = u32::from_bytes(&[0x12, 0x34, 0x56, 0x78], Endian::Little);
        assert_eq!(got, 0x78563412);
    }

    #[test]
    fn test_i32_endian() {
        let got = i32::from_bytes(&[0x01, 0x23, 0x45, 0x67], Endian::Big);
        assert_eq!(got, 0x01234567);
        let got = i32::from_bytes(&[0x01, 0x23, 0x45, 0x67], Endian::Little);
        assert_eq!(got, 0x67452301);
    }

    #[test]
    fn test_u64_endian() {
        let got = u64::from_bytes(
            &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef],
            Endian::Big,
        );
        assert_eq!(got, 0x0123456789abcdef);
        let got = u64::from_bytes(
            &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef],
            Endian::Little,
        );
        assert_eq!(got, 0xefcdab8967452301);
    }

    #[test]
    fn test_f64_endian() {
        let got = f64::from_bytes(
            &[0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18],
            Endian::Big,
        );
        assert!((got - std::f64::consts::PI).abs() < f64::EPSILON);
        let got = f64::from_bytes(
            &[0x18, 0x2d, 0x44, 0x54, 0xfb, 0x21, 0x09, 0x40],
            Endian::Little,
        );
        assert!((got - std::f64::consts::PI).abs() < f64::EPSILON);
    }
}
