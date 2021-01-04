use crate::handler::*;
use crate::strings::*;
use crate::structures::*;
use crate::{decompress::Decompress, endian::*};
use core::ptr::copy_nonoverlapping;
use std::io::Read;

pub(crate) fn common<R: Read>(
    header: &Header,
    reader: &mut R,
    common: &mut [u8; Common::size_of()],
) -> std::io::Result<Common> {
    reader.read_exact(common)?;
    let length = u64::from_bytes(&common[0..8], header.endian);
    let class = common[9];

    Ok(Common::new(length, class))
}

pub(crate) fn frameheader<R: Read, T: Handler>(
    header: &Header,
    reader: &mut R,
    handler: &mut T,
    buf16: &mut [u8; core::mem::size_of::<u16>()],
    buf32: &mut [u8; core::mem::size_of::<u32>()],
    buf64: &mut [u8; core::mem::size_of::<u64>()],
) -> std::io::Result<()> {
    let header = FrameHeader {
        name: string(header, reader)?,
        run: i32::read_into(buf32, reader, header.endian)?,
        frame: u32::read_into(buf32, reader, header.endian)?,
        data_quality: u32::read_into(buf32, reader, header.endian)?,
        gps_start_time_s: u32::read_into(buf32, reader, header.endian)?,
        gps_residual_time_ns: u32::read_into(buf32, reader, header.endian)?,
        gps_leap_s: { u16::read_into(buf16, reader, header.endian)? },
        frame_length_s: { f64::read_into(buf64, reader, header.endian)? },
    };
    if let Some(begin_frame) = handler.begin_frame() {
        begin_frame(handler, header);
    }
    Ok(())
}

pub(crate) fn structure_header<R: Read>(
    header: &Header,
    reader: &mut R,
    lookup: &mut StructureLookup,
    buf16: &mut [u8; core::mem::size_of::<u16>()],
) -> std::io::Result<()> {
    let name = string(header, reader)?;
    let class = u16::read_into(buf16, reader, header.endian)?;
    let _comment = string(header, reader)?;

    match lookup.insert(&name, class) {
        Ok(_) => Ok(()),
        Err(e) => std::result::Result::Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        )),
    }
}

pub(crate) fn detector<R: Read, T: Handler>(
    header: &Header,
    reader: &mut R,
    handler: &mut T,
    buf32: &mut [u8; core::mem::size_of::<f32>()],
    buf64: &mut [u8; core::mem::size_of::<f64>()],
) -> std::io::Result<()> {
    let d = Detector {
        name: string(header, reader)?,
        prefix: {
            let mut buf: [u8; 2] = [0, 0];
            reader.read_exact(&mut buf)?;
            [buf[0] as i8, buf[1] as i8]
        },
        longitude_radians: f64::read_into(buf64, reader, header.endian)?,
        latitude_radians: f64::read_into(buf64, reader, header.endian)?,
        elevation_meters: f32::read_into(buf32, reader, header.endian)?,
        arm_x_azimuth_radians: f32::read_into(buf32, reader, header.endian)?,
        arm_y_azimuth_radians: f32::read_into(buf32, reader, header.endian)?,
        arm_x_altitude_radians: f32::read_into(buf32, reader, header.endian)?,
        arm_y_altitude_radians: f32::read_into(buf32, reader, header.endian)?,
        arm_x_midpoint_meters: f32::read_into(buf32, reader, header.endian)?,
        arm_y_midpoint_meters: f32::read_into(buf32, reader, header.endian)?,
        local_time_utc_offset_s: i32::read_into(buf32, reader, header.endian)?,
    };
    if let Some(handle_detector) = handler.detector() {
        handle_detector(handler, d);
    }
    Ok(())
}

pub(crate) fn adc<R: Read, T: Handler>(
    header: &Header,
    reader: &mut R,
    handler: &mut T,
    buf16: &mut [u8; core::mem::size_of::<u16>()],
    buf32: &mut [u8; core::mem::size_of::<u32>()],
    buf64: &mut [u8; core::mem::size_of::<u64>()],
) -> std::io::Result<()> {
    let a = ADC {
        name: string(header, reader)?,
        comment: string(header, reader)?,
        channel_group: u32::read_into(buf32, reader, header.endian)?,
        channel_number: u32::read_into(buf32, reader, header.endian)?,
        num_bits: u32::read_into(buf32, reader, header.endian)?,
        bias: f32::read_into(buf32, reader, header.endian)?,
        slope: f32::read_into(buf32, reader, header.endian)?,
        units: {
            let u = string(header, reader)?;
            match u.as_ref() {
                "NONE" => None,
                _ => Some(u),
            }
        },
        sample_rate: f64::read_into(buf64, reader, header.endian)?,
        time_offset_s: f64::read_into(buf64, reader, header.endian)?,
        f_shift: f64::read_into(buf64, reader, header.endian)?,
        phase: f32::read_into(buf32, reader, header.endian)?,
        data_valid: {
            let valid = u16::read_into(buf16, reader, header.endian)?;
            valid == 0
        },
    };
    if let Some(handle_adc) = handler.adc() {
        handle_adc(handler, a);
    }
    Ok(())
}

pub(crate) fn message<R: Read, T: Handler>(
    header: &Header,
    reader: &mut R,
    handler: &mut T,
    buf32: &mut [u8; core::mem::size_of::<u32>()],
) -> std::io::Result<()> {
    let msg = Message {
        alarm: string(header, reader)?,
        message: string(header, reader)?,
        severity: u32::read_into(buf32, reader, header.endian)?,
        gps_time_s: u32::read_into(buf32, reader, header.endian)?,
        gps_residual_time_ns: u32::read_into(buf32, reader, header.endian)?,
    };
    if let Some(handle_message) = handler.message() {
        handle_message(handler, msg);
    }
    Ok(())
}

pub(crate) fn history<R: Read, T: Handler>(
    header: &Header,
    reader: &mut R,
    handler: &mut T,
    buf32: &mut [u8; core::mem::size_of::<u32>()],
) -> std::io::Result<()> {
    let h = History {
        name: string(header, reader)?,
        gps_time_s: u32::read_into(buf32, reader, header.endian)?,
        comment: string(header, reader)?,
    };
    if let Some(handle_history) = handler.history() {
        handle_history(handler, h);
    }
    Ok(())
}

pub(crate) fn raw<R: Read, T: Handler>(
    header: &Header,
    reader: &mut R,
    handler: &mut T,
) -> std::io::Result<()> {
    let r = RawData {
        name: string(header, reader)?,
    };
    if let Some(handle_raw) = handler.raw() {
        handle_raw(handler, r);
    }
    Ok(())
}

pub(crate) fn post_processed<R: Read, T: Handler>(
    header: &Header,
    reader: &mut R,
    handler: &mut T,
    buf16: &mut [u8; core::mem::size_of::<u16>()],
    buf32: &mut [u8; core::mem::size_of::<u32>()],
    buf64: &mut [u8; core::mem::size_of::<u64>()],
) -> std::io::Result<()> {
    let mut ppd = PostProcessed {
        name: string(header, reader)?,
        comment: string(header, reader)?,
        // TODO(goller): convert to enum.
        data_type: u16::read_into(buf16, reader, header.endian)?,
        sub_type: u16::read_into(buf16, reader, header.endian)?,
        time_offset_s: f64::read_into(buf64, reader, header.endian)?,
        time_range_s: f64::read_into(buf64, reader, header.endian)?,
        f_shift: f64::read_into(buf64, reader, header.endian)?,
        phase: f32::read_into(buf32, reader, header.endian)?,
        frequency_range: f64::read_into(buf64, reader, header.endian)?,
        bandwidth: f64::read_into(buf64, reader, header.endian)?,
        auxiliary_parameters: Vec::with_capacity(
            u16::read_into(buf16, reader, header.endian)? as usize
        ),
    };

    let num_aux = ppd.auxiliary_parameters.capacity();

    let mut values = vec![0.0; num_aux];
    for value in values.iter_mut() {
        *value = f64::read_into(buf64, reader, header.endian)?;
    }

    for value in values.into_iter() {
        let param = AuxiliaryParameter {
            value,
            name: string(header, reader)?,
        };
        ppd.auxiliary_parameters.push(param);
    }

    if let Some(post_process) = handler.post_processed() {
        post_process(handler, ppd);
    }
    Ok(())
}

pub(crate) fn simulated<R: Read, T: Handler>(
    header: &Header,
    reader: &mut R,
    handler: &mut T,
    buf32: &mut [u8; core::mem::size_of::<u32>()],
    buf64: &mut [u8; core::mem::size_of::<u64>()],
) -> std::io::Result<()> {
    let s = Simulation {
        name: string(header, reader)?,
        comment: string(header, reader)?,
        sample_rate: f64::read_into(buf64, reader, header.endian)?,
        time_offset_s: f64::read_into(buf64, reader, header.endian)?,
        f_shift: f64::read_into(buf64, reader, header.endian)?,
        phase: f32::read_into(buf32, reader, header.endian)?,
    };
    if let Some(handle_simulated) = handler.simulated() {
        handle_simulated(handler, s);
    }
    Ok(())
}

pub(crate) fn simulated_event<R: Read, T: Handler>(
    header: &Header,
    reader: &mut R,
    handler: &mut T,
    buf16: &mut [u8; core::mem::size_of::<u16>()],
    buf32: &mut [u8; core::mem::size_of::<u32>()],
    buf64: &mut [u8; core::mem::size_of::<u64>()],
) -> std::io::Result<()> {
    let mut event = SimulatedEvent {
        name: string(header, reader)?,
        comment: string(header, reader)?,
        inputs: string(header, reader)?,
        gps_event_max_time_s: u32::read_into(buf32, reader, header.endian)?,
        gps_residual_time_ns: u32::read_into(buf32, reader, header.endian)?,
        duration_before_s: f32::read_into(buf32, reader, header.endian)?,
        duration_after_s: f32::read_into(buf32, reader, header.endian)?,
        amplitude: f32::read_into(buf32, reader, header.endian)?,
        parameters: Vec::with_capacity(u16::read_into(buf16, reader, header.endian)? as usize),
    };

    let cap = event.parameters.capacity();
    let mut values = vec![0.0; cap];
    for value in values.iter_mut() {
        *value = f64::read_into(buf64, reader, header.endian)?;
    }

    for value in values.into_iter() {
        let param = EventParameter {
            value,
            name: string(header, reader)?,
        };
        event.parameters.push(param);
    }

    if let Some(handle_sim_event) = handler.simulated_event() {
        handle_sim_event(handler, event);
    }
    Ok(())
}

pub(crate) fn serial<R: Read, T: Handler>(
    header: &Header,
    reader: &mut R,
    handler: &mut T,
    buf32: &mut [u8; core::mem::size_of::<u32>()],
    buf64: &mut [u8; core::mem::size_of::<u64>()],
) -> std::io::Result<()> {
    let s = Serial {
        name: string(header, reader)?,
        gps_time_s: u32::read_into(buf32, reader, header.endian)?,
        gps_residual_time_ns: u32::read_into(buf32, reader, header.endian)?,
        sample_rate: f64::read_into(buf64, reader, header.endian)?,
        data: string(header, reader)?,
    };
    if let Some(handle_serial) = handler.serial() {
        handle_serial(handler, s);
    }
    Ok(())
}

pub(crate) fn static_data<R: Read, T: Handler>(
    header: &Header,
    reader: &mut R,
    handler: &mut T,
    buf32: &mut [u8; core::mem::size_of::<u32>()],
) -> std::io::Result<()> {
    let s = StaticData {
        name: string(header, reader)?,
        comment: string(header, reader)?,
        representation: string(header, reader)?,
        gps_time_start_s: u32::read_into(buf32, reader, header.endian)?,
        gps_time_end_s: u32::read_into(buf32, reader, header.endian)?,
        version: u32::read_into(buf32, reader, header.endian)?,
    };
    if let Some(handle_static) = handler.static_data() {
        handle_static(handler, s);
    }
    Ok(())
}

pub(crate) fn summary<R: Read, T: Handler>(
    header: &Header,
    reader: &mut R,
    handler: &mut T,
    buf32: &mut [u8; core::mem::size_of::<u32>()],
) -> std::io::Result<()> {
    let s = Summary {
        name: string(header, reader)?,
        comment: string(header, reader)?,
        test: string(header, reader)?,
        gps_time_s: u32::read_into(buf32, reader, header.endian)?,
        gps_residual_time_ns: u32::read_into(buf32, reader, header.endian)?,
    };
    if let Some(handle_summary) = handler.summary() {
        handle_summary(handler, s);
    }
    Ok(())
}

pub(crate) fn table<R: Read, T: Handler>(
    header: &Header,
    reader: &mut R,
    handler: &mut T,
    buf16: &mut [u8; core::mem::size_of::<u16>()],
    buf32: &mut [u8; core::mem::size_of::<u32>()],
) -> std::io::Result<()> {
    let mut tbl = Table {
        name: string(header, reader)?,
        comment: string(header, reader)?,
        num_rows: 0,
        column_names: Vec::with_capacity(0),
    };

    let num_columns = u16::read_into(buf16, reader, header.endian)?;
    tbl.num_rows = u32::read_into(buf32, reader, header.endian)?;
    tbl.column_names.reserve(num_columns as usize);

    for _ in 0..num_columns {
        tbl.column_names.push(string(header, reader)?);
    }

    if let Some(handle_table) = handler.table() {
        handle_table(handler, tbl);
    }
    Ok(())
}

pub(crate) fn event<R: Read, T: Handler>(
    header: &Header,
    reader: &mut R,
    handler: &mut T,
    buf16: &mut [u8; core::mem::size_of::<u16>()],
    buf32: &mut [u8; core::mem::size_of::<u32>()],
    buf64: &mut [u8; core::mem::size_of::<u64>()],
) -> std::io::Result<()> {
    let mut event = Event {
        name: string(header, reader)?,
        comment: string(header, reader)?,
        inputs: string(header, reader)?,
        gps_time_s: u32::read_into(buf32, reader, header.endian)?,
        gps_residual_time_ns: u32::read_into(buf32, reader, header.endian)?,
        duration_before_s: f32::read_into(buf32, reader, header.endian)?,
        duration_after_s: f32::read_into(buf32, reader, header.endian)?,
        event_status: u32::read_into(buf32, reader, header.endian)?,
        amplitude: f32::read_into(buf32, reader, header.endian)?,
        probability: {
            let p = f32::read_into(buf32, reader, header.endian)?;
            if p < 0.0 {
                None
            } else {
                Some(p)
            }
        },
        statistics: string(header, reader)?,
        parameters: Vec::with_capacity(u16::read_into(buf16, reader, header.endian)? as usize),
    };

    let cap = event.parameters.capacity();

    let mut values = vec![0.0; cap];
    for value in values.iter_mut() {
        *value = f64::read_into(buf64, reader, header.endian)?;
    }

    for value in values.into_iter() {
        let param = EventParameter {
            value,
            name: string(header, reader)?,
        };
        event.parameters.push(param);
    }

    if let Some(handle_event) = handler.event() {
        handle_event(handler, event);
    }
    Ok(())
}

fn new_vector(data_buf: Vec<u8>, class: u16, endian: Endian, info: VectorInfo) -> Vector {
    match class {
        0 /* CHAR */ => {
            let mut v = vec![0; data_buf.len()];
            let src = data_buf.as_slice();
            let dst = v.as_mut_slice();
            // SAFETY: i8 and u8 arrays have the same size and alignment.
            unsafe {
                copy_nonoverlapping(src.as_ptr(), dst.as_ptr() as *mut u8, src.len());
            }
            Vector::I8(info, v)
        },
        1 /* i16 */ => {
            let v = Vec::<i16>::transmute(data_buf, endian);
            Vector::I16(info, v)
        },
        2 /* f64 */ => {
            let v = Vec::<f64>::transmute(data_buf, endian);
            Vector::F64(info, v)
        },
        3 /* f32 */ => {
            let v = Vec::<f32>::transmute(data_buf, endian);
            Vector::F32(info, v)
        },
        4 /* i32 */ => {
            let v = Vec::<i32>::transmute(data_buf, endian);
            Vector::I32(info, v)
        },
        5 /* i64 */ => {
            let v = Vec::<i64>::transmute(data_buf, endian);
            Vector::I64(info, v)
        },
        6 /* complex(f32, f32) */ =>{
            let mut v = vec![0.0; data_buf.len()/core::mem::size_of::<f32>()];
            read_into_slice_f32(data_buf.as_slice(), v.as_mut_slice(), endian);
            let mut c: Vec<Complex> = Vec::with_capacity(v.len()/2);
            for i in 0..c.len() {
                c.push(Complex{
                    real: v[i*2] as f64,
                    imaginary: v[i*2+1] as f64,
                });
            }
            Vector::Complexes(info, c)
        },
        7 /* complex(f64, f64) */ =>         {
           let mut v = vec![0.0; data_buf.len()/core::mem::size_of::<f64>()];
            read_into_slice_f64(data_buf.as_slice(), v.as_mut_slice(), endian);
            let mut c: Vec<Complex> = Vec::with_capacity(v.len()/2);
            for i in 0..c.len() {
                c.push(Complex{
                    real: v[i*2],
                    imaginary: v[i*2+1],
                });
            }
            Vector::Complexes(info, c)
        }
        // TODO(goller): create string vector
        // 8 NOTE: strings are not compressed
        //
        9 /* u16 */ =>{
            let v = Vec::<u16>::transmute(data_buf, endian);
            Vector::U16(info, v)
        },
        10 /* u32 */ =>{
            let v = Vec::<u32>::transmute(data_buf, endian);
            Vector::U32(info, v)
        },
        11 /* u64 */ => {
            let v = Vec::<u64>::transmute(data_buf, endian);
            Vector::U64(info, v)
        },
        12 /* u8 */ => {
            Vector::U8(info, data_buf)
        },
        _ => Vector::U8(info, data_buf),
    }
}

pub(crate) fn vector<R: Read, T: Handler>(
    header: &Header,
    reader: &mut R,
    handler: &mut T,
    buf16: &mut [u8; core::mem::size_of::<u16>()],
    buf32: &mut [u8; core::mem::size_of::<u32>()],
    buf64: &mut [u8; core::mem::size_of::<u64>()],
    dec: &mut Decompress,
) -> std::io::Result<()> {
    let name = string(header, reader)?;
    let compression = u16::read_into(buf16, reader, header.endian)?;
    let vector_class = u16::read_into(buf16, reader, header.endian)?;
    let num_samples = u64::read_into(buf64, reader, header.endian)?;

    let len = u64::read_into(buf64, reader, header.endian)?;
    let mut raw_buf = vec![0; len as usize];
    reader.read_exact(&mut raw_buf.as_mut_slice())?;

    let data_buf = match compression {
        0 | 256 => raw_buf,
        _ => dec.decompress(&raw_buf, compression, vector_class, num_samples)?,
    };

    let num_dimensions = u32::read_into(buf32, reader, header.endian)?;
    let mut raw_nx = vec![0; num_dimensions as usize * core::mem::size_of::<u64>()];
    reader.read_exact(&mut raw_nx.as_mut_slice())?;

    let mut dimension_lengths: Vec<u64> = vec![0; num_dimensions as usize];
    read_into_slice_u64(
        raw_nx.as_slice(),
        dimension_lengths.as_mut_slice(),
        header.endian,
    );

    reader.read_exact(&mut raw_nx.as_mut_slice())?;
    let mut sample_spacing: Vec<f64> = vec![0.0; num_dimensions as usize];
    read_into_slice_f64(
        raw_nx.as_slice(),
        sample_spacing.as_mut_slice(),
        header.endian,
    );

    reader.read_exact(&mut raw_nx.as_mut_slice())?;
    let mut x_origins: Vec<f64> = vec![0.0; num_dimensions as usize];
    read_into_slice_f64(raw_nx.as_slice(), x_origins.as_mut_slice(), header.endian);

    let mut unit_x_scale_factors: Vec<String> = Vec::with_capacity(num_dimensions as usize);
    for _ in 0..num_dimensions {
        unit_x_scale_factors.push(string(header, reader)?);
    }

    let unit_y = string(header, reader)?;

    let info = VectorInfo {
        name,
        num_samples,
        num_dimensions,
        dimension_lengths,
        sample_spacing,
        x_origins,
        unit_x_scale_factors,
        unit_y,
    };

    if let Some(vector) = handler.vector() {
        let v = new_vector(data_buf, vector_class, header.endian, info);
        vector(handler, v);
    }
    Ok(())
}
