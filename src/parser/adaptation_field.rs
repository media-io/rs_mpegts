use bitstream_io::{BigEndian, BitReader};
use mpegts::adaptation_field::AdaptationField;

use parser::program_clock::parse_program_clock;
use parser::adaptation_field_extension::parse_adaptation_field_extension;

pub fn parse_adaptation_field(mut stream: &mut BitReader<BigEndian>, mut count: &mut usize) -> Option<AdaptationField> {
  let length = stream.read::<u8>(8).unwrap();
  *count = 1;
  // println!("adapt length {:?}", length);

  if length == 0 {
    let adaptation_field = AdaptationField {
      length: length,
      discontinuity_indicator: false,
      random_access_indicator: false,
      elementary_stream_priority_indicator: false,
      pcr: None,
      opcr: None,
      splice_countdown: None,
      transport_private_data: vec![],
      adaptation_field_extension: None,
    };
    return Some(adaptation_field)
  }

  let discontinuity_indicator = stream.read_bit().unwrap();
  let random_access_indicator = stream.read_bit().unwrap();
  let elementary_stream_priority_indicator = stream.read_bit().unwrap();
  let pcr_flag = stream.read_bit().unwrap();
  let opcr_flag = stream.read_bit().unwrap();
  let splicing_point_flag = stream.read_bit().unwrap();
  let transport_private_data_flag = stream.read_bit().unwrap();
  let adaptation_field_extension_flag = stream.read_bit().unwrap();

  *count += 1;

  let mut pcr = None;
  let mut opcr = None;
  let mut splice_countdown = None;
  let mut transport_private_data = vec![];
  let mut adaptation_field_extension = None;

  if pcr_flag {
    pcr = Some(parse_program_clock(&mut stream));
    *count += 6;
  }
  if opcr_flag {
    opcr = Some(parse_program_clock(&mut stream));
    *count += 6;
  }
  if splicing_point_flag {
    let splice_countdown_two_complement = stream.read::<u8>(8).unwrap();
    
    splice_countdown =
      if splice_countdown_two_complement & 0x80 == 0x80 {
        Some(- ((splice_countdown_two_complement & 0x7f) as i8))
      } else {
        Some(splice_countdown_two_complement as i8)
      };
    
    *count += 1;
  }
  if transport_private_data_flag {
    let transport_private_data_length = stream.read::<u8>(8).unwrap();

    transport_private_data = vec![0; transport_private_data_length as usize];

    stream.read_bytes(&mut transport_private_data).unwrap();
    *count += 1 + transport_private_data_length as usize;
  }
  if adaptation_field_extension_flag {
    adaptation_field_extension = parse_adaptation_field_extension(&mut stream, &mut count);
  }

  if *count < length as usize {
    for _i in 0..(length + 1 - *count as u8){
      let data = stream.read::<u8>(8).unwrap();
      if data != 0xff {
        panic!("some data is not parse for AdaptationField");
      }
      *count += 1;
    }
  }

  let adaptation_field = AdaptationField {
    length: length,
    discontinuity_indicator: discontinuity_indicator,
    random_access_indicator: random_access_indicator,
    elementary_stream_priority_indicator: elementary_stream_priority_indicator,
    pcr: pcr,
    opcr: opcr,
    splice_countdown: splice_countdown,
    transport_private_data: transport_private_data,
    adaptation_field_extension: adaptation_field_extension,
  };

  Some(adaptation_field)
}
