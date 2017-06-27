
use bitstream_io::{BigEndian, BitWriter};
use crc::{crc32, Hasher32};

use mpegts::table_id::*;
use mpegts::program_association::ProgramAssociation;
use mpegts::program_map::ProgramMap;
use mpegts::packetized_elementary_stream::PacketizedElementaryStream;
use mpegts::payload::Payload;
use mpegts::stream_id::StreamId;
use writer::table_id::get_table_id;
use writer::stream_id::get_stream_id;

pub fn write_payload(mut writer: &mut BitWriter<BigEndian>, payload: &Option<Payload>) {
  match *payload {
    None => {},
    Some(ref p) => {

      match p.pes {
        None => {},
        Some(ref pes) => {
          writer.write(16, 0).unwrap();
          writer.write(8, 1).unwrap();
          writer.write(8, get_stream_id(pes.stream_id.clone())).unwrap();

          let data = write_packetized_elementary_stream(&pes);

          match pes.stream_id {
            StreamId::VideoStream{id: _id} => {
              writer.write(16, 0).unwrap();
            },
            _ => {
              writer.write(16, data.len() as u16).unwrap();
            },
          }
          writer.write_bytes(&data).unwrap();
        },
      }

      match p.pat {
        None => {},
        Some(ref pat) => {
          writer.write(8, 0).unwrap();
          writer.write(8, get_table_id(TableId::ProgramAssociation)).unwrap();
          writer.write_bit(true).unwrap();
          writer.write_bit(false).unwrap();
          writer.write(2, 0b11).unwrap();
          let data = write_program_association(&pat);
          writer.write(12, data.len() as u16 + 4).unwrap();
          writer.write_bytes(&data).unwrap();

          let mut digest = crc32::Digest::new(crc32::IEEE);
          digest.write(&data);
          let crc32 = digest.sum32();
          writer.write(32, crc32).unwrap();
        },
      }
      match p.pmt {
        None => {},
        Some(ref pmt) => {
          writer.write(8, 0).unwrap();
          writer.write(8, get_table_id(TableId::ProgramMap)).unwrap();
          writer.write_bit(true).unwrap();
          writer.write_bit(false).unwrap();
          writer.write(2, 0b11).unwrap();
          let data = write_program_map(&pmt);
          writer.write(12, data.len() as u16 + 4).unwrap();
          writer.write_bytes(&data).unwrap();
          let mut digest = crc32::Digest::new(crc32::IEEE);
          digest.write(&data);
          let crc32 = digest.sum32();
          writer.write(32, crc32).unwrap();
        },
      }
    }
  }
}

fn write_program_association(pat: &ProgramAssociation) -> Vec<u8> {
  let mut data = Vec::new();
  {
    let mut writer = BitWriter::<BigEndian>::new(&mut data);
    writer.write(16, pat.transport_stream_id).unwrap();
    writer.write(2, 0b11).unwrap();
    let version_number = 0x00;
    writer.write(5, version_number).unwrap();
    writer.write_bit(true).unwrap();
    writer.write(8, 0x00).unwrap();
    writer.write(8, 0x00).unwrap();
    for association in &pat.table {
      writer.write(16, association.program_number).unwrap();
      writer.write(3, 0b111).unwrap();
      writer.write(13, association.program_map_pid).unwrap();
    }
  }
  data
}

fn write_program_map(pmt: &ProgramMap) -> Vec<u8> {
  let mut data = Vec::new();
  {
    let mut writer = BitWriter::<BigEndian>::new(&mut data);
    writer.write(16, pmt.program_number).unwrap();
    writer.write(2, 0b11).unwrap();
    let version_number = 0x00;
    writer.write(5, version_number).unwrap();
    writer.write_bit(true).unwrap();
    writer.write(8, 0x00).unwrap();
    writer.write(8, 0x00).unwrap();
    writer.write(3, 0b111).unwrap();
    writer.write(13, pmt.pcr_pid).unwrap();
    writer.write(4, 0b1111).unwrap();
    writer.write(12, 0x0).unwrap();

    for program in &pmt.programs {
      writer.write(8, get_stream_id(program.stream_id.clone())).unwrap();
      writer.write(3, 0b111).unwrap();
      writer.write(13, program.elementary_pid).unwrap();
      writer.write(4, 0b1111).unwrap();
      writer.write(12, program.es_info.len() as u16).unwrap();
      writer.write_bytes(&program.es_info).unwrap();
    }
  }
  data
}

fn write_packetized_elementary_stream(pes: &PacketizedElementaryStream) -> Vec<u8> {
  let mut data = Vec::new();
  {
    let mut writer = BitWriter::<BigEndian>::new(&mut data);

    match pes.header {
      None => {},
      Some(ref header) => {
        writer.write(2, 0b10).unwrap();
        
        writer.write(2, header.scrambling_control).unwrap();
        writer.write_bit(header.priority).unwrap();
        writer.write_bit(header.data_alignment_indicator).unwrap();
        writer.write_bit(header.copyright).unwrap();
        writer.write_bit(header.original).unwrap();
        writer.write_bit(header.pts.is_some()).unwrap();
        writer.write_bit(header.dts.is_some()).unwrap();
        writer.write_bit(header.escr.is_some()).unwrap();
        writer.write_bit(header.es_rate.is_some()).unwrap();
        writer.write_bit(header.dsm_trick_mode.is_some()).unwrap();
        writer.write_bit(header.additional_copy_info.is_some()).unwrap();
        writer.write_bit(header.previous_pes_packet_crc.is_some()).unwrap();
        writer.write_bit(header.pes_extension.is_some()).unwrap();
        writer.write(8, header.pes_header_length).unwrap();

        match header.pts {
          None => {},
          Some(ref pts) => {
            match header.dts {
              Some(_) => {
                writer.write(4, 0b0011).unwrap();
              },
              None => {
                writer.write(4, 0b0010).unwrap();
              },
            }

            writer.write(3, (pts & (0b111 << 30)) >> 30).unwrap();
            writer.write_bit(true).unwrap();
            writer.write(15, (pts & (0x7FFF << 15)) >> 15).unwrap();
            writer.write_bit(true).unwrap();
            writer.write(15, pts & 0x7FFF).unwrap();
            writer.write_bit(true).unwrap();
          }
        }
        match header.dts {
          None => {},
          Some(ref dts) => {
            writer.write(4, 0b0001).unwrap();
            
            writer.write(3, (dts & (0b111 << 30)) >> 30).unwrap();
            writer.write_bit(true).unwrap();
            writer.write(15, (dts & (0x7FFF << 15)) >> 15).unwrap();
            writer.write_bit(true).unwrap();
            writer.write(15, dts & 0x7FFF).unwrap();
            writer.write_bit(true).unwrap();
          }
        }
        match header.escr {
          None => {},
          Some(ref escr) => {
            writer.write(3, (escr & (0b111 << 39)) >> 39).unwrap();
            writer.write_bit(true).unwrap();
            writer.write(15, (escr & (0x7FFF << 24)) >> 24).unwrap();
            writer.write_bit(true).unwrap();
            writer.write(15, (escr & (0x7FFF << 9)) >> 9).unwrap();
            writer.write_bit(true).unwrap();
            writer.write(9, escr & 0x1FF).unwrap();
            writer.write_bit(true).unwrap();
          }
        }
        match header.es_rate {
          None => {},
          Some(ref es_rate) => {
            writer.write_bit(true).unwrap();
            writer.write(22, es_rate & 0x3FFFFF).unwrap();
            writer.write_bit(true).unwrap();
          }
        }

        match header.additional_copy_info {
          None => {},
          Some(copy_info) => {
            writer.write_bit(true).unwrap();
            writer.write(7, copy_info).unwrap();
          },
        }
        match header.previous_pes_packet_crc {
          None => {},
          Some(crc) => {
            writer.write(16, crc).unwrap();
          },
        }
      }
    }

    if pes.additional_data.len() > 0 {
      let _res = writer.write_bytes(&pes.additional_data);
    }
  }
  data
}
