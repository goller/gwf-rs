pub struct Decompress {
    gunzipper: flate2::Decompress,
}

impl Decompress {
    pub fn new() -> Self {
        Decompress {
            gunzipper: flate2::Decompress::new(true /* zlib_header */),
        }
    }

    pub fn decompress(
        &mut self,
        raw_buf: &[u8],
        compression: u16,
        vector_class: u16,
        num_samples: u64,
    ) -> std::io::Result<Vec<u8>> {
        let size = Self::decompressed_size(vector_class, num_samples);
        let mut decompressed_buf = vec![0; size];
        self.dec(&raw_buf, compression, &mut decompressed_buf)?;
        Ok(decompressed_buf)
    }

    pub fn dec(
        &mut self,
        data_buf: &[u8],
        compression: u16,
        output: &mut [u8],
    ) -> std::io::Result<()> {
        match compression {
            1 | 257 => Ok(self.gunzip(data_buf, output)?),
            _ => std::result::Result::Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("compression {} not yet handled", compression),
            )),
        }
    }

    fn gunzip(&mut self, input: &[u8], output: &mut [u8]) -> std::io::Result<()> {
        match self
            .gunzipper
            .decompress(input, output, flate2::FlushDecompress::Finish)
        {
            Ok(_) => Ok(()),
            Err(e) => std::result::Result::Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            )),
        }
    }

    fn decompressed_size(class: u16, num_samples: u64) -> usize {
        match class{
        0 /* CHAR */ => num_samples as usize * core::mem::size_of::<i8>(),
        1 /* i16 */ => num_samples as usize * core::mem::size_of::<i16>(),
        2 /* f64 */ => num_samples as usize * core::mem::size_of::<f64>(),
        3 /* f32 */ => num_samples as usize * core::mem::size_of::<f32>(),
        4 /* i32 */ => num_samples as usize * core::mem::size_of::<i32>(),
        5 /* i64 */ => num_samples as usize * core::mem::size_of::<i64>(),
        6 /* complex(f32, f32) */ => 2 * num_samples as usize * core::mem::size_of::<f32>(),
        7 /* complex(f64, f64) */ => 2 * num_samples as usize * core::mem::size_of::<f64>(),
        // 8 NOTE: strings are not compressed
        9 /* u16 */ => num_samples as usize * core::mem::size_of::<u16>(),
        10 /* u32 */ => num_samples as usize * core::mem::size_of::<u32>(),
        11 /* u64 */ => num_samples as usize * core::mem::size_of::<u64>(),
        12 /* u8 */ => num_samples as usize * core::mem::size_of::<u8>(),
        _ => num_samples as usize * core::mem::size_of::<u8>(),

    }
    }
}
