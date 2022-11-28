use mpegts::descriptor::hevc::*;
use mpegts::program_descriptor::ProgramDescriptor;
use mpegts::stream_id::StreamId;

#[derive(Debug, Clone)]
pub struct EsInfo {
    pub descriptor: ProgramDescriptor,
    pub hevc: Option<Hevc>,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub stream_id: StreamId,
    pub elementary_pid: u16,
    pub es_info: EsInfo,
}

#[derive(Debug, Clone)]
pub struct ProgramMap {
    pub program_number: u16,
    pub pcr_pid: u16,
    pub programs: Vec<Program>,
}
