
use bitstream_io::{BigEndian, BitReader};
use mpegts::adaptation_field_extension::AdaptationFieldExtension;

pub fn parse_adaptation_field_extension(mut stream: &mut BitReader<BigEndian>, mut count: &mut usize) -> Option<AdaptationFieldExtension> {
  let _adaptation_extension_length = stream.read::<u8>(8).unwrap();

  let legal_time_window_flag = stream.read_bit().unwrap();
  let piecewise_rate_flag = stream.read_bit().unwrap();
  let seamless_splice_flag = stream.read_bit().unwrap();
  let _reserved = stream.read::<u8>(5).unwrap();
  *count += 2;

  let mut legal_time_window = None;
  let mut piecewise_rate = None;
  let mut seamless_splice = None;

  if legal_time_window_flag {
    legal_time_window = Some(stream.read::<u16>(16).unwrap());
    *count += 2;
  }
  if piecewise_rate_flag {
    piecewise_rate = Some(stream.read::<u32>(24).unwrap());
    *count += 3;
  }
  if seamless_splice_flag {
    let splice_type = stream.read::<u8>(4).unwrap();
    let dts_high = stream.read::<u8>(3).unwrap();
    let _marker_bit = stream.read_bit().unwrap();
    let dts_medium = stream.read::<u16>(15).unwrap();
    let _marker_bit = stream.read_bit().unwrap();
    let dts_low = stream.read::<u16>(15).unwrap();
    let _marker_bit = stream.read_bit().unwrap();

    seamless_splice = Some(
      ((splice_type as u64) << 33) +
      ((dts_high as u64) << 30) +
      ((dts_medium as u64) << 15) +
      dts_low as u64
    );
    *count += 5;
  }

  Some(AdaptationFieldExtension{
    legal_time_window: legal_time_window,
    piecewise_rate: piecewise_rate,
    seamless_splice: seamless_splice,
  })
}
