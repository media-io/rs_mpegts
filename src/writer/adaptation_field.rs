use bitstream_io::{BigEndian, BitWriter};

use mpegts::adaptation_field::AdaptationField;
use writer::adaptation_field_extension::*;
use writer::program_clock::*;

pub fn write(af: &AdaptationField) -> Vec<u8> {
    let mut data = Vec::new();
    {
        let mut af_writer = BitWriter::<BigEndian>::new(&mut data);

        af_writer.write_bit(af.discontinuity_indicator).unwrap();
        af_writer.write_bit(af.random_access_indicator).unwrap();
        af_writer
            .write_bit(af.elementary_stream_priority_indicator)
            .unwrap();
        af_writer.write_bit(af.pcr.is_some()).unwrap();
        af_writer.write_bit(af.opcr.is_some()).unwrap();
        af_writer.write_bit(af.splice_countdown.is_some()).unwrap();
        af_writer
            .write_bit(!af.transport_private_data.is_empty())
            .unwrap();
        af_writer
            .write_bit(af.adaptation_field_extension.is_some())
            .unwrap();

        match af.pcr {
            None => {}
            Some(ref pcr) => {
                write_program_clock(&mut af_writer, pcr);
            }
        }

        match af.opcr {
            None => {}
            Some(ref opcr) => {
                write_program_clock(&mut af_writer, opcr);
            }
        }

        match af.splice_countdown {
            None => {}
            Some(ref splice_countdown) => {
                if *splice_countdown < 0 {
                    af_writer
                        .write(8, (-splice_countdown) as u8 + 0x80)
                        .unwrap();
                } else {
                    af_writer.write(8, *splice_countdown as u8).unwrap();
                }
            }
        }

        if !af.transport_private_data.is_empty() {
            af_writer
                .write(8, af.transport_private_data.len() as u8)
                .unwrap();
            af_writer.write_bytes(&af.transport_private_data).unwrap();
        }

        write_adaptation_field_extension(&mut af_writer, &af.adaptation_field_extension)
    }
    data
}

pub fn write_adaptation_field(
    writer: &mut BitWriter<BigEndian>,
    adaptation_field: &Option<AdaptationField>,
) {
    match *adaptation_field {
        None => {}
        Some(ref af) => {
            if af.length == 0 {
                writer.write(8, 0_u8).unwrap();
                return;
            }

            let mut data = write(af);

            let fill_count = (af.length as usize) - data.len();
            // println!("fill count {:?}", fill_count);
            if fill_count > 1 {
                for _i in 0..fill_count {
                    data.push(0xFF);
                }
            }
            writer.write(8, data.len() as u8).unwrap();
            writer.write_bytes(&data).unwrap();
        }
    }
}
