use crate::structures::*;
use crate::v6;
use crate::v8;
use crate::{decompress::Decompress, handler::*};

use std::io::{Read, Seek, SeekFrom, Take};

pub struct Parser {
    common_buf: [u8; Common::size_of()],
    buf16: [u8; core::mem::size_of::<u16>()],
    buf32: [u8; core::mem::size_of::<u32>()],
    buf64: [u8; core::mem::size_of::<u64>()],
    dec: Decompress,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            common_buf: [0; Common::size_of()],
            buf16: [0; core::mem::size_of::<u16>()],
            buf32: [0; core::mem::size_of::<u32>()],
            buf64: [0; core::mem::size_of::<u64>()],
            dec: Decompress::new(),
        }
    }

    pub(crate) fn parse<R, T>(
        &mut self,
        header: &Header,
        reader: &mut R,
        handler: &mut T,
    ) -> std::io::Result<()>
    where
        R: Read + Seek,
        T: Handler,
    {
        let mut struct_lookup = StructureLookup::new();

        reader.seek(SeekFrom::Start(Header::size_of() as u64))?;
        loop {
            let common = match header.version.major {
                Major::Release8 => v8::common(header, reader, &mut self.common_buf)?,
                Major::Release6 => v6::common(header, reader)?,
                _ => unimplemented!("no support for version: {:?}", header.version),
            };

            let structure = struct_lookup.structure(common.class());

            if structure == Structures::FrEndOfFile {
                break;
            }

            if !self.handles(&structure, handler) {
                reader.seek(common.seek_past())?;
                continue;
            }
            {
                let mut struct_reader = reader.by_ref().take(common.struct_length());
                self.handle(
                    structure,
                    header,
                    &mut struct_reader,
                    handler,
                    &mut struct_lookup,
                )?;

                // try to consume the rest of the struct reader.
                let limit = struct_reader.limit() as i64;
                struct_reader.into_inner().seek(SeekFrom::Current(limit))?;
            }
        }

        if let Some(eof) = handler.eof() {
            eof(handler);
        }

        Ok(())
    }

    fn handles<T: Handler>(&self, class: &Structures, handler: &mut T) -> bool {
        match class {
            Structures::FrSH => true,
            Structures::FrameH => handler.begin_frame().is_some(),
            Structures::FrDetector => handler.detector().is_some(),
            Structures::FrAdcData => handler.adc().is_some(),
            Structures::FrMsg => handler.message().is_some(),
            Structures::FrHistory => handler.history().is_some(),
            Structures::FrRawData => handler.raw().is_some(),
            Structures::FrProcData => handler.post_processed().is_some(),
            Structures::FrSimData => handler.simulated().is_some(),
            Structures::FrSimEvent => handler.simulated_event().is_some(),
            Structures::FrSerData => handler.serial().is_some(),
            Structures::FrStatData => handler.static_data().is_some(),
            Structures::FrSummary => handler.summary().is_some(),
            Structures::FrTable => handler.table().is_some(),
            Structures::FrVect => handler.vector().is_some(),
            Structures::FrEvent => handler.event().is_some(),
            _ => false,
        }
    }

    fn handle<R: Read, T: Handler>(
        &mut self,
        class: Structures,
        header: &Header,
        reader: &mut Take<R>,
        handler: &mut T,
        lookup: &mut StructureLookup,
    ) -> std::io::Result<()> {
        match class {
            Structures::FrSH => v8::structure_header(header, reader, lookup, &mut self.buf16),
            Structures::FrameH => v8::frameheader(
                header,
                reader,
                handler,
                &mut self.buf16,
                &mut self.buf32,
                &mut self.buf64,
            ),
            Structures::FrDetector => {
                v8::detector(header, reader, handler, &mut self.buf32, &mut self.buf64)
            }
            Structures::FrAdcData => v8::adc(
                header,
                reader,
                handler,
                &mut self.buf16,
                &mut self.buf32,
                &mut self.buf64,
            ),
            Structures::FrMsg => v8::message(header, reader, handler, &mut self.buf32),
            Structures::FrHistory => v8::history(header, reader, handler, &mut self.buf32),
            Structures::FrRawData => v8::raw(header, reader, handler),
            Structures::FrProcData => v8::post_processed(
                header,
                reader,
                handler,
                &mut self.buf16,
                &mut self.buf32,
                &mut self.buf64,
            ),
            Structures::FrSimData => match header.version.major {
                Major::Release8 => {
                    v8::simulated(header, reader, handler, &mut self.buf32, &mut self.buf64)
                }
                Major::Release6 => v6::simulated(header, reader, handler),
                _ => unreachable!(),
            },
            Structures::FrSimEvent => match header.version.major {
                Major::Release8 => v8::simulated_event(
                    header,
                    reader,
                    handler,
                    &mut self.buf16,
                    &mut self.buf32,
                    &mut self.buf64,
                ),
                Major::Release6 => v6::simulated_event(header, reader, handler),
                _ => unreachable!(),
            },
            Structures::FrSerData => match header.version.major {
                Major::Release8 => {
                    v8::serial(header, reader, handler, &mut self.buf32, &mut self.buf64)
                }
                Major::Release6 => v6::serial(header, reader, handler),
                _ => unreachable!(),
            },
            Structures::FrStatData => v8::static_data(header, reader, handler, &mut self.buf32),
            Structures::FrSummary => v8::summary(header, reader, handler, &mut self.buf32),
            Structures::FrTable => {
                v8::table(header, reader, handler, &mut self.buf16, &mut self.buf32)
            }
            Structures::FrVect => v8::vector(
                header,
                reader,
                handler,
                &mut self.buf16,
                &mut self.buf32,
                &mut self.buf64,
                &mut self.dec,
            ),
            Structures::FrEvent => match header.version.major {
                Major::Release8 => v8::event(
                    header,
                    reader,
                    handler,
                    &mut self.buf16,
                    &mut self.buf32,
                    &mut self.buf64,
                ),
                Major::Release6 => v6::event(header, reader, handler),
                _ => unreachable!(),
            },
            _ => Ok(()),
        }
    }
}
