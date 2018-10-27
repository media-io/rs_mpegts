
use std::fmt;

use mpegts::adaptation_field::AdaptationField;
use mpegts::payload::Payload;
use mpegts::program_association::*;
use mpegts::program_map::*;

#[derive(Debug, Clone)]
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

impl Packet {
  pub fn new() -> Packet {
    Packet {
      transport_error_indicator: false,
      transport_priority: false,
      program_id: 0,
      transport_scrambling_control: 0x00,
      continuity_counter: 0x00,
      payload_presence: false,
      adaptation_field: None,
      payload: None,
      data: vec![],
    }
  }

  pub fn new_pat(pat: ProgramAssociation) -> Packet {
    let mut p = Packet::new();
    p.payload_presence = true;
    p.payload = Some(
      Payload{
        pat: Some(pat),
        pmt: None,
        pes: None,
    });

    p
  }

  pub fn new_pmt(id: u16, pmt: ProgramMap) -> Packet {
    let mut p = Packet::new();
    p.program_id = id;
    p.payload_presence = true;
    p.payload = Some(
      Payload{
        pat: None,
        pmt: Some(pmt),
        pes: None,
    });

    p
  }

  pub fn new_null() -> Packet {
    let mut p = Packet::new();
    p.program_id = 0x1FFF;
    p
  }
}
