use crate::endian::*;
use crate::handler::*;
use crate::strings::*;
use crate::structures::*;
use std::io::Read;

pub(crate) fn common<R: Read>(header: &Header, reader: &mut R) -> std::io::Result<Common> {
    let mut common = [0; Common::size_of()];
    reader.read_exact(&mut common)?;

    let length = u64::from_bytes(&common[0..8], header.endian);
    let class = u16::from_bytes(&common[8..10], header.endian) as u8;
    Ok(Common::new(length, class))
}

pub(crate) fn simulated_event<R: Read, T: Handler>(
    header: &Header,
    reader: &mut R,
    handler: &mut T,
) -> std::io::Result<()> {
    let mut buf2 = [0; core::mem::size_of::<u16>()];
    let mut buf4 = [0; core::mem::size_of::<f32>()];

    let mut event = SimulatedEvent {
        name: string(header, reader)?,
        comment: string(header, reader)?,
        inputs: string(header, reader)?,
        gps_event_max_time_s: u32::read_into(&mut buf4, reader, header.endian)?,
        gps_residual_time_ns: u32::read_into(&mut buf4, reader, header.endian)?,
        duration_before_s: f32::read_into(&mut buf4, reader, header.endian)?,
        duration_after_s: f32::read_into(&mut buf4, reader, header.endian)?,
        amplitude: f32::read_into(&mut buf4, reader, header.endian)?,
        parameters: Vec::with_capacity(u16::read_into(&mut buf2, reader, header.endian)? as usize),
    };

    let cap = event.parameters.capacity();
    let mut values = vec![0.0; cap];
    for value in values.iter_mut() {
        *value = f32::read_into(&mut buf4, reader, header.endian)?;
    }

    for value in values.into_iter() {
        let param = EventParameter {
            value: value as f64,
            name: string(header, reader)?,
        };
        event.parameters.push(param);
    }

    if let Some(handle_sim_event) = handler.simulated_event() {
        handle_sim_event(handler, event);
    }
    Ok(())
}

pub(crate) fn event<R: Read, T: Handler>(
    header: &Header,
    reader: &mut R,
    handler: &mut T,
) -> std::io::Result<()> {
    let mut buf2 = [0; core::mem::size_of::<u16>()];
    let mut buf4 = [0; core::mem::size_of::<f32>()];

    let mut event = Event {
        name: string(header, reader)?,
        comment: string(header, reader)?,
        inputs: string(header, reader)?,
        gps_time_s: u32::read_into(&mut buf4, reader, header.endian)?,
        gps_residual_time_ns: u32::read_into(&mut buf4, reader, header.endian)?,
        duration_before_s: f32::read_into(&mut buf4, reader, header.endian)?,
        duration_after_s: f32::read_into(&mut buf4, reader, header.endian)?,
        event_status: u32::read_into(&mut buf4, reader, header.endian)?,
        amplitude: f32::read_into(&mut buf4, reader, header.endian)?,
        probability: {
            let p = f32::read_into(&mut buf4, reader, header.endian)?;
            if p < 0.0 {
                None
            } else {
                Some(p)
            }
        },
        statistics: string(header, reader)?,
        parameters: Vec::with_capacity(u16::read_into(&mut buf2, reader, header.endian)? as usize),
    };

    let cap = event.parameters.capacity();

    let mut values = vec![0.0; cap];
    for value in values.iter_mut() {
        *value = f32::read_into(&mut buf4, reader, header.endian)?;
    }

    for value in values.into_iter() {
        let param = EventParameter {
            value: value as f64,
            name: string(header, reader)?,
        };
        event.parameters.push(param);
    }

    if let Some(handle_event) = handler.event() {
        handle_event(handler, event);
    }
    Ok(())
}

pub(crate) fn serial<R: Read, T: Handler>(
    header: &Header,
    reader: &mut R,
    handler: &mut T,
) -> std::io::Result<()> {
    let mut buf4 = [0; core::mem::size_of::<u32>()];
    let s = Serial {
        name: string(header, reader)?,
        gps_time_s: u32::read_into(&mut buf4, reader, header.endian)?,
        gps_residual_time_ns: u32::read_into(&mut buf4, reader, header.endian)?,
        sample_rate: f32::read_into(&mut buf4, reader, header.endian)? as f64,
        data: string(header, reader)?,
    };
    if let Some(handle_serial) = handler.serial() {
        handle_serial(handler, s);
    }
    Ok(())
}

pub(crate) fn simulated<R: Read, T: Handler>(
    header: &Header,
    reader: &mut R,
    handler: &mut T,
) -> std::io::Result<()> {
    let mut buf4 = [0; core::mem::size_of::<f32>()];
    let mut buf8 = [0; core::mem::size_of::<f64>()];
    let s = Simulation {
        name: string(header, reader)?,
        comment: string(header, reader)?,
        sample_rate: f32::read_into(&mut buf4, reader, header.endian)? as f64,
        time_offset_s: f64::read_into(&mut buf8, reader, header.endian)?,
        f_shift: f64::read_into(&mut buf8, reader, header.endian)?,
        phase: f32::read_into(&mut buf4, reader, header.endian)?,
    };
    if let Some(handle_simulated) = handler.simulated() {
        handle_simulated(handler, s);
    }
    Ok(())
}
