use crate::structures::*;

/// Handler will be called when the parser sees one of the the data structures.
pub trait Handler {
    fn version(&mut self) -> Option<fn(&mut Self, version: Version)> {
        None
    }

    fn eof(&mut self) -> Option<fn(&mut Self)> {
        None
    }

    fn begin_frame(&mut self) -> Option<fn(&mut Self, header: FrameHeader)> {
        None
    }

    fn adc(&mut self) -> Option<fn(&mut Self, data: ADC)> {
        None
    }

    fn detector(&mut self) -> Option<fn(&mut Self, data: Detector)> {
        None
    }

    fn event(&mut self) -> Option<fn(&mut Self, data: Event)> {
        None
    }

    fn history(&mut self) -> Option<fn(&mut Self, data: History)> {
        None
    }

    fn message(&mut self) -> Option<fn(&mut Self, data: Message)> {
        None
    }

    fn post_processed(&mut self) -> Option<fn(&mut Self, data: PostProcessed)> {
        None
    }

    fn raw(&mut self) -> Option<fn(&mut Self, data: RawData)> {
        None
    }

    fn serial(&mut self) -> Option<fn(&mut Self, data: Serial)> {
        None
    }

    fn simulated(&mut self) -> Option<fn(&mut Self, data: Simulation)> {
        None
    }

    fn simulated_event(&mut self) -> Option<fn(&mut Self, data: SimulatedEvent)> {
        None
    }

    fn static_data(&mut self) -> Option<fn(&mut Self, data: StaticData)> {
        None
    }

    fn summary(&mut self) -> Option<fn(&mut Self, data: Summary)> {
        None
    }

    fn table(&mut self) -> Option<fn(&mut Self, data: Table)> {
        None
    }

    fn vector(&mut self) -> Option<fn(&mut Self, data: Vector)> {
        None
    }
}
