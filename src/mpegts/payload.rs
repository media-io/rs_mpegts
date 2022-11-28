use mpegts::packetized_elementary_stream::PacketizedElementaryStream;
use mpegts::program_association::ProgramAssociation;
use mpegts::program_map::ProgramMap;

#[derive(Debug, Clone)]
pub struct Payload {
    pub pat: Option<ProgramAssociation>,
    pub pmt: Option<ProgramMap>,
    pub pes: Option<PacketizedElementaryStream>,
}
