use gwf::parse;
use pprof::protos::Message;
use std::fs::File;
use std::io::Write;

fn main() {
    let guard = pprof::ProfilerGuard::new(100).unwrap();
    struct Checker {}

    impl gwf::handler::Handler for Checker {
        fn version(&mut self) -> Option<fn(&mut Self, gwf::structures::Version)> {
            Some(|_: &mut Checker, _: gwf::structures::Version| ())
        }

        fn begin_frame(&mut self) -> Option<fn(&mut Self, header: gwf::structures::FrameHeader)> {
            Some(|_: &mut Checker, _: gwf::structures::FrameHeader| ())
        }
        fn vector(&mut self) -> Option<fn(&mut Self, data: gwf::structures::Vector)> {
            Some(|_: &mut Checker, _: gwf::structures::Vector| ())
        }
    }

    let mut handler = Checker {};
    for _ in 0..2 {
        let filename = "assets/PyCBC_T3_0.gwf";
        let mut file = File::open(filename).expect("unable to open file");
        parse(&mut file, &mut handler).unwrap();
    }

    if let Ok(report) = guard.report().build() {
        let mut file = File::create("profile.pb").unwrap();
        let profile = report.pprof().unwrap();

        let mut content = Vec::new();
        profile.encode(&mut content).unwrap();
        file.write_all(&content).unwrap();

        println!("report: {:?}", &report);
    };
}
