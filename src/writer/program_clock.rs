use bitstream_io::{BigEndian, BitWriter};
use mpegts::program_clock::ProgramClock;

pub fn write_program_clock(stream: &mut BitWriter<BigEndian>, pc: &ProgramClock) {
    stream.write(33, pc.base).unwrap();
    stream.write(6, 0b111111).unwrap();
    stream.write(9, pc.extension).unwrap();
}
