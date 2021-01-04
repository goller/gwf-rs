#![feature(test)]

extern crate gwf;
#[cfg(test)]
mod tests {
    use gwf::{parse, parse_file};
    use std::error::Error;
    use std::fs::File;

    struct VersionChecker {
        version: gwf::structures::Version,
    }

    impl gwf::handler::Handler for VersionChecker {
        fn version(&mut self) -> Option<fn(&mut Self, gwf::structures::Version)> {
            Some(
                |v: &mut VersionChecker, version: gwf::structures::Version| {
                    v.version = version;
                },
            )
        }
    }

    #[test]
    fn test_version_6() -> Result<(), Box<dyn Error>> {
        let filename = "assets/F-TEST-600000000-60.gwf";
        let mut handler = VersionChecker {
            version: gwf::structures::Version {
                major: gwf::structures::Major::Unsupported(0),
                minor: gwf::structures::Minor::Beta,
            },
        };
        parse_file(filename, &mut handler)?;

        assert_eq!(handler.version.major, gwf::structures::Major::Release6);
        assert_eq!(handler.version.minor, gwf::structures::Minor::Minor(20));
        Ok(())
    }

    #[test]
    fn test_version_8() -> Result<(), Box<dyn Error>> {
        let filename = "assets/PyCBC_T3_0.gwf";

        let mut handler = VersionChecker {
            version: gwf::structures::Version {
                major: gwf::structures::Major::Unsupported(0),
                minor: gwf::structures::Minor::Beta,
            },
        };
        parse_file(filename, &mut handler)?;

        assert_eq!(handler.version.major, gwf::structures::Major::Release8);
        assert_eq!(handler.version.minor, gwf::structures::Minor::Minor(1));
        Ok(())
    }

    #[test]
    fn test_gw_open_data_workshop_t3_0() -> Result<(), Box<dyn Error>> {
        let filename = "assets/PyCBC_T3_0.gwf";
        struct Checker {}

        impl gwf::handler::Handler for Checker {
            fn version(&mut self) -> Option<fn(&mut Self, gwf::structures::Version)> {
                Some(|_: &mut Checker, version: gwf::structures::Version| {
                    assert_eq!(version.major, gwf::structures::Major::Release8);
                    assert_eq!(version.minor, gwf::structures::Minor::Minor(1));
                })
            }

            fn post_processed(
                &mut self,
            ) -> Option<fn(&mut Self, data: gwf::structures::PostProcessed)> {
                Some(|_: &mut Checker, data: gwf::structures::PostProcessed| {
                    assert_eq!(
                        data,
                        gwf::structures::PostProcessed {
                            name: "H1:TEST-STRAIN".to_string(),
                            comment: "None".to_string(),
                            data_type: 1,
                            sub_type: 0,
                            time_offset_s: 0.0,
                            time_range_s: 128.0,
                            f_shift: 0.0,
                            phase: 0.0,
                            frequency_range: 0.0,
                            bandwidth: 0.0,
                            auxiliary_parameters: vec![],
                        }
                    );
                })
            }

            fn begin_frame(
                &mut self,
            ) -> Option<fn(&mut Self, header: gwf::structures::FrameHeader)> {
                Some(|_: &mut Checker, header: gwf::structures::FrameHeader| {
                    match header.frame {
                        0 => assert_eq!(
                            header,
                            gwf::structures::FrameHeader {
                                name: "gwpy".to_string(),
                                run: 0,
                                frame: 0,
                                data_quality: 4005673240,
                                gps_start_time_s: 0,
                                gps_residual_time_ns: 0,
                                gps_leap_s: 19,
                                frame_length_s: 128.0,
                            }
                        ),
                        32650 => assert_eq!(
                            header,
                            gwf::structures::FrameHeader {
                                name: "gwpy".to_string(),
                                run: 0,
                                frame: 32650,
                                data_quality: 4005673240,
                                gps_start_time_s: 0,
                                gps_residual_time_ns: 0,
                                gps_leap_s: 19,
                                frame_length_s: 128.0,
                            }
                        ),
                        _ => unreachable!(), // unexpected frame
                    }
                })
            }
            fn vector(&mut self) -> Option<fn(&mut Self, data: gwf::structures::Vector)> {
                Some(
                    |_: &mut Checker, data: gwf::structures::Vector| match data {
                        gwf::structures::Vector::F64(info, values) => {
                            assert_eq!(
                                info,
                                gwf::structures::VectorInfo {
                                    name: String::from(""),
                                    num_samples: 524288,
                                    num_dimensions: 1,
                                    dimension_lengths: vec![524288],
                                    sample_spacing: vec![0.000244140625],
                                    x_origins: vec![0.0],
                                    unit_x_scale_factors: vec![String::from("s")],
                                    unit_y: String::from(""),
                                }
                            );
                            assert!(
                                (values[values.len() - 1] - 0.00000000000000000005645729203487291)
                                    .abs()
                                    < f64::EPSILON
                            );
                        }
                        _ => unreachable!(),
                    },
                )
            }
        }

        let mut handler = Checker {};
        parse_file(filename, &mut handler)?;
        Ok(())
    }

    #[test]
    fn test_gw_open_data_workshop_t2_0() -> Result<(), Box<dyn Error>> {
        let filename = "assets/PyCBC_T2_0.gwf";
        struct Checker {}

        impl gwf::handler::Handler for Checker {
            fn version(&mut self) -> Option<fn(&mut Self, gwf::structures::Version)> {
                Some(|_: &mut Checker, version: gwf::structures::Version| {
                    assert_eq!(version.major, gwf::structures::Major::Release8);
                    assert_eq!(version.minor, gwf::structures::Minor::Minor(1));
                })
            }

            fn post_processed(
                &mut self,
            ) -> Option<fn(&mut Self, data: gwf::structures::PostProcessed)> {
                Some(|_: &mut Checker, data: gwf::structures::PostProcessed| {
                    assert_eq!(
                        data,
                        gwf::structures::PostProcessed {
                            name: "H1:TEST-STRAIN".to_string(),
                            comment: "None".to_string(),
                            data_type: 1,
                            sub_type: 0,
                            time_offset_s: 0.0,
                            time_range_s: 128.0,
                            f_shift: 0.0,
                            phase: 0.0,
                            frequency_range: 0.0,
                            bandwidth: 0.0,
                            auxiliary_parameters: vec![],
                        }
                    );
                })
            }

            fn begin_frame(
                &mut self,
            ) -> Option<fn(&mut Self, header: gwf::structures::FrameHeader)> {
                Some(|_: &mut Checker, header: gwf::structures::FrameHeader| {
                    match header.frame {
                        0 => assert_eq!(
                            header,
                            gwf::structures::FrameHeader {
                                name: "gwpy".to_string(),
                                run: 0,
                                frame: 0,
                                data_quality: 11,
                                gps_start_time_s: 0,
                                gps_residual_time_ns: 0,
                                gps_leap_s: 19,
                                frame_length_s: 128.0,
                            }
                        ),
                        32650 => assert_eq!(
                            header,
                            gwf::structures::FrameHeader {
                                name: "gwpy".to_string(),
                                run: 0,
                                frame: 32650,
                                data_quality: 4005673240,
                                gps_start_time_s: 0,
                                gps_residual_time_ns: 0,
                                gps_leap_s: 19,
                                frame_length_s: 128.0,
                            }
                        ),
                        _ => unreachable!(), // unexpected frame
                    }
                })
            }
            fn vector(&mut self) -> Option<fn(&mut Self, data: gwf::structures::Vector)> {
                Some(
                    |_: &mut Checker, data: gwf::structures::Vector| match data {
                        gwf::structures::Vector::F64(info, values) => {
                            assert_eq!(
                                info,
                                gwf::structures::VectorInfo {
                                    name: String::from(""),
                                    num_samples: 524288,
                                    num_dimensions: 1,
                                    dimension_lengths: vec![524288],
                                    sample_spacing: vec![0.000244140625],
                                    x_origins: vec![0.0],
                                    unit_x_scale_factors: vec![String::from("s")],
                                    unit_y: String::from(""),
                                }
                            );
                            assert!(
                                (values[values.len() - 1] - 0.00000000000000000005645729203487293)
                                    .abs()
                                    < f64::EPSILON
                            );
                        }
                        _ => unreachable!(),
                    },
                )
            }
        }

        let mut handler = Checker {};
        parse_file(filename, &mut handler)?;
        Ok(())
    }

    #[test]
    fn test_gw_open_data_workshop_t2_1() -> Result<(), Box<dyn Error>> {
        let filename = "assets/PyCBC_T2_1.gwf";
        struct Checker {}

        impl gwf::handler::Handler for Checker {
            fn version(&mut self) -> Option<fn(&mut Self, gwf::structures::Version)> {
                Some(|_: &mut Checker, version: gwf::structures::Version| {
                    assert_eq!(version.major, gwf::structures::Major::Release8);
                    assert_eq!(version.minor, gwf::structures::Minor::Minor(1));
                })
            }

            fn post_processed(
                &mut self,
            ) -> Option<fn(&mut Self, data: gwf::structures::PostProcessed)> {
                Some(|_: &mut Checker, data: gwf::structures::PostProcessed| {
                    assert_eq!(
                        data,
                        gwf::structures::PostProcessed {
                            name: "H1:TEST-STRAIN".to_string(),
                            comment: "None".to_string(),
                            data_type: 1,
                            sub_type: 0,
                            time_offset_s: 0.0,
                            time_range_s: 128.0,
                            f_shift: 0.0,
                            phase: 0.0,
                            frequency_range: 0.0,
                            bandwidth: 0.0,
                            auxiliary_parameters: vec![],
                        }
                    );
                })
            }

            fn begin_frame(
                &mut self,
            ) -> Option<fn(&mut Self, header: gwf::structures::FrameHeader)> {
                Some(|_: &mut Checker, header: gwf::structures::FrameHeader| {
                    match header.frame {
                        0 => assert_eq!(
                            header,
                            gwf::structures::FrameHeader {
                                name: "gwpy".to_string(),
                                run: 0,
                                frame: 0,
                                data_quality: 11,
                                gps_start_time_s: 0,
                                gps_residual_time_ns: 0,
                                gps_leap_s: 19,
                                frame_length_s: 128.0,
                            }
                        ),
                        32650 => assert_eq!(
                            header,
                            gwf::structures::FrameHeader {
                                name: "gwpy".to_string(),
                                run: 0,
                                frame: 32650,
                                data_quality: 4005673240,
                                gps_start_time_s: 0,
                                gps_residual_time_ns: 0,
                                gps_leap_s: 19,
                                frame_length_s: 128.0,
                            }
                        ),
                        _ => unreachable!(), // unexpected frame
                    }
                })
            }
            fn vector(&mut self) -> Option<fn(&mut Self, data: gwf::structures::Vector)> {
                Some(
                    |_: &mut Checker, data: gwf::structures::Vector| match data {
                        gwf::structures::Vector::F64(info, values) => {
                            assert_eq!(
                                info,
                                gwf::structures::VectorInfo {
                                    name: String::from(""),
                                    num_samples: 524288,
                                    num_dimensions: 1,
                                    dimension_lengths: vec![524288],
                                    sample_spacing: vec![0.000244140625],
                                    x_origins: vec![0.0],
                                    unit_x_scale_factors: vec![String::from("s")],
                                    unit_y: String::from(""),
                                }
                            );
                            assert!(
                                (values[values.len() - 1] - 0.00000000000000000005645729203487293)
                                    .abs()
                                    < f64::EPSILON
                            )
                        }
                        _ => unreachable!(),
                    },
                )
            }
        }

        let mut handler = Checker {};
        parse_file(filename, &mut handler)?;
        Ok(())
    }

    #[test]
    fn test_gw_open_data_workshop_t2_2() -> Result<(), Box<dyn Error>> {
        let filename = "assets/PyCBC_T2_2.gwf";
        struct Checker {}

        impl gwf::handler::Handler for Checker {
            fn version(&mut self) -> Option<fn(&mut Self, gwf::structures::Version)> {
                Some(|_: &mut Checker, version: gwf::structures::Version| {
                    assert_eq!(version.major, gwf::structures::Major::Release8);
                    assert_eq!(version.minor, gwf::structures::Minor::Minor(1));
                })
            }

            fn post_processed(
                &mut self,
            ) -> Option<fn(&mut Self, data: gwf::structures::PostProcessed)> {
                Some(|_: &mut Checker, data: gwf::structures::PostProcessed| {
                    assert_eq!(
                        data,
                        gwf::structures::PostProcessed {
                            name: "H1:TEST-STRAIN".to_string(),
                            comment: "None".to_string(),
                            data_type: 1,
                            sub_type: 0,
                            time_offset_s: 0.0,
                            time_range_s: 128.0,
                            f_shift: 0.0,
                            phase: 0.0,
                            frequency_range: 0.0,
                            bandwidth: 0.0,
                            auxiliary_parameters: vec![],
                        }
                    );
                })
            }

            fn begin_frame(
                &mut self,
            ) -> Option<fn(&mut Self, header: gwf::structures::FrameHeader)> {
                Some(|_: &mut Checker, header: gwf::structures::FrameHeader| {
                    match header.frame {
                        0 => assert_eq!(
                            header,
                            gwf::structures::FrameHeader {
                                name: "gwpy".to_string(),
                                run: 0,
                                frame: 0,
                                data_quality: 11,
                                gps_start_time_s: 0,
                                gps_residual_time_ns: 0,
                                gps_leap_s: 19,
                                frame_length_s: 128.0,
                            }
                        ),
                        32650 => assert_eq!(
                            header,
                            gwf::structures::FrameHeader {
                                name: "gwpy".to_string(),
                                run: 0,
                                frame: 32650,
                                data_quality: 4005673240,
                                gps_start_time_s: 0,
                                gps_residual_time_ns: 0,
                                gps_leap_s: 19,
                                frame_length_s: 128.0,
                            }
                        ),
                        _ => unreachable!(), // unexpected frame
                    }
                })
            }
            fn vector(&mut self) -> Option<fn(&mut Self, data: gwf::structures::Vector)> {
                Some(
                    |_: &mut Checker, data: gwf::structures::Vector| match data {
                        gwf::structures::Vector::F64(info, values) => {
                            assert_eq!(
                                info,
                                gwf::structures::VectorInfo {
                                    name: String::from(""),
                                    num_samples: 524288,
                                    num_dimensions: 1,
                                    dimension_lengths: vec![524288],
                                    sample_spacing: vec![0.000244140625],
                                    x_origins: vec![0.0],
                                    unit_x_scale_factors: vec![String::from("s")],
                                    unit_y: String::from(""),
                                }
                            );
                            assert!(
                                (values[values.len() - 1] - 0.00000000000000000005645729203487293)
                                    .abs()
                                    < f64::EPSILON
                            )
                        }
                        _ => unreachable!(),
                    },
                )
            }
        }

        let mut handler = Checker {};
        parse_file(filename, &mut handler)?;
        Ok(())
    }

    #[test]
    fn test_gw_open_data_workshop_challenge1() -> Result<(), Box<dyn Error>> {
        let filename = "assets/challenge3.gwf";
        struct Checker {
            ppds: Vec<gwf::structures::PostProcessed>,
            vs: Vec<gwf::structures::Vector>,
        }

        impl gwf::handler::Handler for Checker {
            fn version(&mut self) -> Option<fn(&mut Self, gwf::structures::Version)> {
                Some(|_: &mut Checker, version: gwf::structures::Version| {
                    assert_eq!(version.major, gwf::structures::Major::Release8);
                    assert_eq!(version.minor, gwf::structures::Minor::Minor(30));
                })
            }
            fn post_processed(
                &mut self,
            ) -> Option<fn(&mut Self, data: gwf::structures::PostProcessed)> {
                Some(|c: &mut Checker, data: gwf::structures::PostProcessed| {
                    c.ppds.push(data);
                })
            }
            fn begin_frame(
                &mut self,
            ) -> Option<fn(&mut Self, header: gwf::structures::FrameHeader)> {
                Some(|_: &mut Checker, header: gwf::structures::FrameHeader| {
                    match header.frame {
                        1 => assert_eq!(
                            header,
                            gwf::structures::FrameHeader {
                                name: "".to_string(),
                                run: 1,
                                frame: 1,
                                data_quality: 0,
                                gps_start_time_s: 0,
                                gps_residual_time_ns: 0,
                                gps_leap_s: 19,
                                frame_length_s: 4096.0,
                            }
                        ),
                        _ => unreachable!(), // unexpected frame
                    }
                })
            }
            fn vector(&mut self) -> Option<fn(&mut Self, data: gwf::structures::Vector)> {
                Some(|c: &mut Checker, data: gwf::structures::Vector| {
                    c.vs.push(data);
                })
            }
        }

        let mut handler = Checker {
            ppds: Vec::with_capacity(2),
            vs: Vec::with_capacity(2),
        };
        parse_file(filename, &mut handler)?;
        for (i, v) in handler.vs.iter().enumerate() {
            match v {
                gwf::structures::Vector::F64(info, values) => {
                    match i {
                        0 => assert_eq!(
                            *info,
                            gwf::structures::VectorInfo {
                                name: "L1:CHALLENGE3".to_string(),
                                num_samples: 16777216,
                                num_dimensions: 1,
                                dimension_lengths: vec![16777216,],
                                sample_spacing: vec![0.000244140625,],
                                x_origins: vec![0.0,],
                                unit_x_scale_factors: vec!["s".to_string(),],
                                unit_y: "count".to_string(),
                            }
                        ),
                        1 => {
                            assert_eq!(
                                *info,
                                gwf::structures::VectorInfo {
                                    name: "H1:CHALLENGE3".to_string(),
                                    num_samples: 16777216,
                                    num_dimensions: 1,
                                    dimension_lengths: vec![16777216,],
                                    sample_spacing: vec![0.000244140625,],
                                    x_origins: vec![0.0,],
                                    unit_x_scale_factors: vec!["s".to_string(),],
                                    unit_y: "count".to_string(),
                                }
                            );
                            assert!(
                                (values[values.len() - 1] - 0.0000000000000000004986230158918658)
                                    .abs()
                                    < f64::EPSILON
                            )
                        }
                        _ => unreachable!(), // should only have 2 vectors
                    }
                }
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    extern crate test;
    use self::test::Bencher;
    #[bench]
    fn bench_gw_open_data_workshop_t3_0(b: &mut Bencher) {
        struct Checker {}

        impl gwf::handler::Handler for Checker {
            fn version(&mut self) -> Option<fn(&mut Self, gwf::structures::Version)> {
                Some(|_: &mut Checker, _: gwf::structures::Version| ())
            }

            fn begin_frame(
                &mut self,
            ) -> Option<fn(&mut Self, header: gwf::structures::FrameHeader)> {
                Some(|_: &mut Checker, _: gwf::structures::FrameHeader| ())
            }
            fn vector(&mut self) -> Option<fn(&mut Self, data: gwf::structures::Vector)> {
                Some(|_: &mut Checker, _: gwf::structures::Vector| ())
            }
        }

        let mut handler = Checker {};
        b.iter(|| {
            let filename = "assets/PyCBC_T3_0.gwf";
            let mut file = File::open(filename).expect("unable to open file");

            parse(&mut file, &mut handler).unwrap();
            //file.seek(std::io::SeekFrom::Start(0)).unwrap();
        });
    }
}
