use gwf::parse;
use std::fs::File;

fn main() {
    struct Checker {}

    impl gwf::handler::Handler for Checker {
        fn version(&mut self) -> Option<fn(&mut Self, gwf::structures::Version)> {
            Some(|_: &mut Checker, _: gwf::structures::Version| ())
        }

        fn begin_frame(&mut self) -> Option<fn(&mut Self, header: gwf::structures::FrameHeader)> {
            Some(|_: &mut Checker, _: gwf::structures::FrameHeader| ())
        }
        fn vector(&mut self) -> Option<fn(&mut Self, data: gwf::structures::Vector)> {
            Some(|_: &mut Checker, v: gwf::structures::Vector| {
                match v {
                    gwf::structures::Vector::F64(info, values) => {
                        println!("INFO: {:#?}", info);
                        let sum: f64 = values.iter().sum();
                        println!("SUM {}", sum);
                    }
                    gwf::structures::Vector::I64(info, values) => {
                        println!("INFO: {:#?}", info);
                        let sum: i64 = values.iter().sum();
                        println!("SUM {}", sum);
                    }
                    gwf::structures::Vector::I32(info, values) => {
                        println!("INFO: {:#?}", info);
                        let sum: i32 = values.iter().sum();
                        println!("SUM {}", sum);
                    }
                    _ => unreachable!(),
                };
            })
        }
    }

    let mut handler = Checker {};
    for _ in 0..1 {
        let filename = "assets/L-L1_GWOSC_16KHZ_R2-1239080215-4096.gwf";
        let mut file = File::open(filename).expect("unable to open file");
        parse(&mut file, &mut handler).unwrap();
    }
}
