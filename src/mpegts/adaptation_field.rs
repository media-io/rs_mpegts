use mpegts::adaptation_field_extension::AdaptationFieldExtension;
use mpegts::program_clock::ProgramClock;

#[derive(Debug, Clone)]
pub struct AdaptationField {
    pub length: u8,
    pub discontinuity_indicator: bool,
    pub random_access_indicator: bool,
    pub elementary_stream_priority_indicator: bool,
    pub pcr: Option<ProgramClock>,
    pub opcr: Option<ProgramClock>,
    pub splice_countdown: Option<i8>,
    pub transport_private_data: Vec<u8>,
    pub adaptation_field_extension: Option<AdaptationFieldExtension>,
}
