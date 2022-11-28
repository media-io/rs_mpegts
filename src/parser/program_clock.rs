
use bitstream_io::{BigEndian, BitReader};
use mpegts::program_clock::ProgramClock;

pub fn parse_program_clock(stream: &mut BitReader<BigEndian>) -> ProgramClock {
  let base = stream.read::<u64>(33).unwrap();
  let _reserved = stream.read::<u8>(6).unwrap();
  let extension = stream.read::<u16>(9).unwrap();

  ProgramClock{
    base,
    extension
  }
}
