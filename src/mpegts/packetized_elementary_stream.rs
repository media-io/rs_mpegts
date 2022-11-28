use mpegts::stream_id::StreamId;

#[derive(Debug, Clone)]
pub enum TrickModeControl {
    FastForward,
    SlowMotion,
    FreezeFrame,
    FastReverse,
    SlowReverse,
    Reserved,
}

#[derive(Debug, Clone)]
pub struct DsmTrickMode {
    pub trick_mode_control: TrickModeControl,
    pub info: u8,
}

#[derive(Debug, Clone)]
pub struct PesExtension {
    pub pes_private_data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct PesHeader {
    pub scrambling_control: u8,
    pub priority: bool,
    pub data_alignment_indicator: bool,
    pub copyright: bool,
    pub original: bool,
    pub pts: Option<u64>,
    pub dts: Option<u64>,
    pub escr: Option<u64>,
    pub es_rate: Option<u32>,
    pub dsm_trick_mode: Option<DsmTrickMode>,
    pub additional_copy_info: Option<u8>,
    pub previous_pes_packet_crc: Option<u16>,
    pub pes_extension: Option<PesExtension>,
    pub pes_header_length: u8,
}

#[derive(Debug, Clone)]
pub struct PacketizedElementaryStream {
    pub stream_id: StreamId,
    pub header: Option<PesHeader>,
    pub additional_data: Vec<u8>,
}
