use gwf::parse_file;
use gwf::structures::*;

fn main() {
    struct Checker {}

    impl gwf::handler::Handler for Checker {
        fn version(&mut self) -> Option<fn(&mut Self, Version)> {
            Some(|_: &mut Checker, data: Version| {
                println!("{:#?}", data);
            })
        }

        fn begin_frame(&mut self) -> Option<fn(&mut Self, header: FrameHeader)> {
            Some(|_: &mut Checker, data: FrameHeader| {
                println!("{:#?}", data);
            })
        }
        fn adc(&mut self) -> Option<fn(&mut Self, data: ADC)> {
            Some(|_: &mut Checker, data: ADC| {
                println!("{:#?}", data);
            })
        }
        fn detector(&mut self) -> Option<fn(&mut Self, data: Detector)> {
            Some(|_: &mut Checker, data: Detector| {
                println!("{:#?}", data);
            })
        }
        fn event(&mut self) -> Option<fn(&mut Self, data: Event)> {
            Some(|_: &mut Checker, data: Event| {
                println!("{:#?}", data);
            })
        }
        fn history(&mut self) -> Option<fn(&mut Self, data: History)> {
            Some(|_: &mut Checker, data: History| {
                println!("{:#?}", data);
            })
        }
        fn message(&mut self) -> Option<fn(&mut Self, data: Message)> {
            Some(|_: &mut Checker, data: Message| {
                println!("{:#?}", data);
            })
        }
        fn post_processed(&mut self) -> Option<fn(&mut Self, data: PostProcessed)> {
            Some(|_: &mut Checker, data: PostProcessed| {
                println!("{:#?}", data);
            })
        }
        fn raw(&mut self) -> Option<fn(&mut Self, data: RawData)> {
            Some(|_: &mut Checker, data: RawData| {
                println!("{:#?}", data);
            })
        }
        fn serial(&mut self) -> Option<fn(&mut Self, data: Serial)> {
            Some(|_: &mut Checker, data: Serial| {
                println!("{:#?}", data);
            })
        }
        fn simulated(&mut self) -> Option<fn(&mut Self, data: Simulation)> {
            Some(|_: &mut Checker, data: Simulation| {
                println!("{:#?}", data);
            })
        }
        fn simulated_event(&mut self) -> Option<fn(&mut Self, data: SimulatedEvent)> {
            Some(|_: &mut Checker, data: SimulatedEvent| {
                println!("{:#?}", data);
            })
        }
        fn static_data(&mut self) -> Option<fn(&mut Self, data: StaticData)> {
            Some(|_: &mut Checker, data: StaticData| {
                println!("{:#?}", data);
            })
        }
        fn summary(&mut self) -> Option<fn(&mut Self, data: Summary)> {
            Some(|_: &mut Checker, data: Summary| {
                println!("{:#?}", data);
            })
        }
        fn table(&mut self) -> Option<fn(&mut Self, data: Table)> {
            Some(|_: &mut Checker, data: Table| {
                println!("{:#?}", data);
            })
        }
        fn vector(&mut self) -> Option<fn(&mut Self, data: Vector)> {
            Some(|_: &mut Checker, v: Vector| {
                match v {
                    Vector::I8(info, _values) => println!("i8: {:?}", info),
                    Vector::I16(info, _values) => println!("i16: {:?}", info),
                    Vector::I32(info, _values) => println!("i32: {:?}", info),
                    Vector::I64(info, _values) => println!("i64: {:?}", info),
                    Vector::U8(info, _values) => println!("u8: {:?}", info),
                    Vector::U16(info, _values) => println!("u16: {:?}", info),
                    Vector::U32(info, _values) => println!("u32: {:?}", info),
                    Vector::U64(info, _values) => println!("u64: {:?}", info),
                    Vector::F32(info, _values) => println!("f32: {:?}", info),
                    Vector::F64(info, _values) => println!("f64: {:?}", info),
                    Vector::Strings(info, _values) => println!("strings: {:?}", info),
                    Vector::Complexes(info, _values) => println!("complex: {:?}", info),
                };
            })
        }
    }

    let mut handler = Checker {};
    let filename = "assets/L-L1_GWOSC_16KHZ_R2-1239080215-4096.gwf";
    parse_file(filename, &mut handler).unwrap();
}
