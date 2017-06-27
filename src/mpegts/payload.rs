
use mpegts::program_association::ProgramAssociation;
use mpegts::program_map::ProgramMap;
use mpegts::packetized_elementary_stream::PacketizedElementaryStream;

#[derive(Debug)]
pub struct Payload {
  pub pat: Option<ProgramAssociation>,
  pub pmt: Option<ProgramMap>,
  pub pes: Option<PacketizedElementaryStream>,
}
