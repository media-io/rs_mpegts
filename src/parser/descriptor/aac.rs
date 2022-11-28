
use bitstream_io::{BigEndian, BitReader};
use mpegts::descriptor::aac::*;

pub fn parse_descriptor(stream: &mut BitReader<BigEndian>) -> Aac {
  let _descriptor_id = stream.read::<u8>(8).unwrap();
  let descriptor_length = stream.read::<u8>(8).unwrap();
  let profile_and_level = stream.read::<u8>(8).unwrap();
  let aac_type_flag = stream.read_bit().unwrap();
  let _reserved = stream.read_bit().unwrap();
  let _reserved = stream.read_bit().unwrap();
  let _reserved = stream.read_bit().unwrap();
  let _reserved = stream.read_bit().unwrap();
  let _reserved = stream.read_bit().unwrap();
  let _reserved = stream.read_bit().unwrap();

  let mut count = 2;
  let mut aac_type = None;
  if aac_type_flag {
    aac_type = Some(stream.read::<u8>(8).unwrap());
    count += 1;
  }

  let mut additional_info = vec![0; descriptor_length as usize - count];
  let _ = stream.read_bytes(&mut additional_info);

  Aac{
    profile_and_level,
    aac_type,
    additional_info,
  }
}