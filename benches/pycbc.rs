use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use gwf::parse;
use std::fs::File;

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

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_PyCBC_T3_0.gwf");
    group.throughput(Throughput::Bytes(4_065_315)); // size of the file
    group.bench_function("parse_PyCBC_T3_0.gwf", |b| {
        b.iter(|| {
            let filename = "assets/PyCBC_T3_0.gwf";
            let mut file = File::open(filename).expect("unable to open file");
            let mut handler = Checker {};

            parse(&mut file, &mut handler).unwrap();
            //file.seek(std::io::SeekFrom::Start(0)).unwrap();
        })
    });
    group.finish();
}

fn criterion_benchmark_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_big_file");
    group.throughput(Throughput::Bytes(516_572_897)); // size of the file
    group.bench_function("parse_L-L1_GWOSC_16KHZ_R2-1239080215-4096.gwf", |b| {
        b.iter(|| {
            let filename = "assets/L-L1_GWOSC_16KHZ_R2-1239080215-4096.gwf";
            let mut file = File::open(filename).expect("unable to open file");
            let mut handler = Checker {};

            parse(&mut file, &mut handler).unwrap();
            //file.seek(std::io::SeekFrom::Start(0)).unwrap();
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark_large);
criterion_main!(benches);
