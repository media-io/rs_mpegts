
use bitstream_io::{BigEndian, BitWriter};
use mpegts::adaptation_field_extension::AdaptationFieldExtension;

pub fn write(afe: &AdaptationFieldExtension) -> Vec<u8> {
  let mut data = Vec::new();
  {
    let mut afe_writer = BitWriter::<BigEndian>::new(&mut data);
    afe_writer.write_bit(afe.legal_time_window.is_some()).unwrap();
    afe_writer.write_bit(afe.piecewise_rate.is_some()).unwrap();
    afe_writer.write_bit(afe.seamless_splice.is_some()).unwrap();

    match afe.legal_time_window {
      None => {},
      Some(ref legal_time_window) => {
        afe_writer.write(16, *legal_time_window).unwrap();
      }
    }
    match afe.piecewise_rate {
      None => {},
      Some(ref piecewise_rate) => {
        afe_writer.write(24, *piecewise_rate).unwrap();
      }
    }
    match afe.seamless_splice {
      None => {},
      Some(ref seamless_splice) => {
        afe_writer.write(4, (*seamless_splice & (0xF << 33)) >> 33).unwrap();
        afe_writer.write(3, (*seamless_splice & (0b111 << 30)) >> 30).unwrap();
        afe_writer.write_bit(true).unwrap();
        afe_writer.write(15, (*seamless_splice & (0x7FFF << 15)) >> 15).unwrap();
        afe_writer.write_bit(true).unwrap();
        afe_writer.write(15, *seamless_splice & 0x7FFF).unwrap();
        afe_writer.write_bit(true).unwrap();
      }
    }
  }
  data
}

pub fn write_adaptation_field_extension(writer: &mut BitWriter<BigEndian>, adaptation_field_extension: &Option<AdaptationFieldExtension>) {

  match *adaptation_field_extension {
    None => {},
    Some(ref afe) => {
      let data = write(afe);
      println!("AFE len = {:?}", data.len());
      writer.write(8, data.len() as u8).unwrap();
      writer.write_bytes(&data).unwrap();
    }
  }
}
