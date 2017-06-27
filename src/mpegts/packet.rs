
use std::fmt;

use mpegts::adaptation_field::AdaptationField;
use mpegts::payload::Payload;

#[derive(Debug)]
pub struct Packet {
  pub transport_error_indicator: bool,
  pub transport_priority: bool,
  pub program_id: u16,
  pub transport_scrambling_control: u8,
  pub continuity_counter: u8,
  pub payload_presence: bool,
  pub adaptation_field: Option<AdaptationField>,
  pub payload: Option<Payload>,
  pub data: Vec<u8>,
}


impl fmt::Display for Packet {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.data.len() > 0 {
      write!(f, "Packet with PID: {:04} (data size = {}), payload {:?}", self.program_id, self.data.len(), self.payload)
    } else {
      // write!(f, "PID: {:04}", self.program_id)
      write!(f, "Packet: {:?}", self)
    }
  }
}
