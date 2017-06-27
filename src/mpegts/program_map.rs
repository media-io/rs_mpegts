
use mpegts::stream_id::StreamId;



#[derive(Debug)]
pub struct Program {
  pub stream_id: StreamId,
  pub elementary_pid: u16,
  pub es_info: Vec<u8>
}

#[derive(Debug)]
pub struct ProgramMap {
  pub program_number: u16,
  pub pcr_pid: u16,
  pub programs: Vec<Program>
}
