
use std::cmp;

use std::io::{Read, Cursor};
use bitstream_io::{BigEndian, BitReader};
use mpegts::payload::Payload;
use mpegts::table_id::TableId;
use mpegts::program_association::{ProgramAssociation, Association};
use mpegts::program_map::*;
use mpegts::program_descriptor::*;
// use mpegts::descriptor::hevc::*;
use mpegts::packetized_elementary_stream::*;
use parser::stream_id::get_stream_id;
use parser::table_id::get_table_id;
use parser::program_descriptor::get_descriptor_id;
use parser::descriptor::hevc::*;

fn parse_table(stream: &mut BitReader<BigEndian>, mut count: &mut usize, length: u16) -> (u16, Vec<u8>) {
  let table_id_extension = stream.read::<u16>(16).unwrap();
  let _reserved = stream.read::<u8>(2).unwrap();
  let _version = stream.read::<u8>(5).unwrap();
  let _current_next_indicator = stream.read_bit().unwrap();
  let _section_number = stream.read::<u8>(8).unwrap();
  let _last_section_number = stream.read::<u8>(8).unwrap();

  *count += 5;
  let mut data = vec![0; (length - 5 - 4) as usize];
  stream.read_bytes(&mut data).unwrap();

  *count += data.len();

  let _crc32 = stream.read::<u32>(32).unwrap();
  *count += 4;
  (table_id_extension, data)
}

pub fn parse_program_association(mut stream: &mut BitReader<BigEndian>, mut count: &mut usize, length: u16) -> ProgramAssociation {
  let (transport_stream_id, data) = parse_table(&mut stream, &mut count, length);

  let mut associations = vec![];

  for i in 0..data.len() / 4 {
    let program_number = ((data[0 + i * 4] as u16) << 8) + data[1 + i * 4] as u16;
    let program_pid = ((data[2 + i * 4] as u16 & 0x001f) << 8) + data[3 + i * 4] as u16;

    associations.push(Association{
      program_number: program_number,
      program_map_pid: program_pid
    });
  }

  ProgramAssociation{
    transport_stream_id: transport_stream_id,
    table: associations
  }
}

pub fn parse_program_map(mut stream: &mut BitReader<BigEndian>, mut count: &mut usize, length: u16) -> ProgramMap {
  let (table_id_extension, data) = parse_table(&mut stream, &mut count, length);

  // println!("{:?}", data);

  let pcr_pid = ((data[0] as u16 & 0x001f) << 8) + data[1] as u16;
  let pi_length = ((data[2] as u16 & 0x0003) << 8) + data[3] as u16;

  if pi_length != 0 {
    unimplemented!();
  }

  let mut programs = vec![];

  let mut offset = 4;
  loop {
    if offset >= data.len() {
      break;
    }

    let stream_type = data[offset];
    let elementary_pid = ((data[offset+1] as u16 & 0x001f) << 8) + data[offset+2] as u16;
    let es_info_length = ((data[offset+3] as u16 & 0x0003) << 8) + data[offset+4] as u16;

    let start = offset + 5;
    let end = offset + 5 + es_info_length as usize;
    let es_info = &data[start .. end];

    let descriptor = get_descriptor_id(es_info[0]);

    let mut cursor = Cursor::new(&es_info);
    let mut es_info_reader = BitReader::<BigEndian>::new(&mut cursor);

    let hevc_descriptor =
      match descriptor {
        ProgramDescriptor::HEVC_Video => Some(parse_descriptor(&mut es_info_reader)),
        // ProgramDescriptor::UserPrivate => Some(parse_descriptor(&mut es_info_reader)),
        _ => None
      };

    println!("{:?}", get_descriptor_id(es_info[0]));

    offset += 5 + es_info_length as usize;

    programs.push(Program{
      stream_id: get_stream_id(stream_type),
      elementary_pid: elementary_pid,
      es_info: EsInfo{
        descriptor: descriptor,
        hevc: hevc_descriptor,
        data: es_info.to_vec()
      }
    })
  }

  ProgramMap {
    program_number: table_id_extension,
    pcr_pid: pcr_pid,
    programs: programs
  }
}

pub fn parse_payload(mut stream: &mut BitReader<BigEndian>, mut count: &mut usize) -> Option<Payload> {
  let mut header : [u8; 3] = [0; 3];
  let _ret = stream.read_bytes(&mut header);
  // println!("{:?}", header);
  *count += 3;

  let mut pat = None;
  let mut pmt = None;
  let mut pes = None;

  match (header[0], header[1], header[2]) {
    (0x00, 0x00, 0x01) => {
      let es_id = stream.read::<u8>(8).unwrap();
      *count += 1;

      let pes_packet_length = stream.read::<u16>(16).unwrap();
      *count += 2;

      let mut header = None;
      let mut additional_data = vec![];

      if pes_packet_length == 0 {
        let optional_pes_header = stream.read::<u8>(2).unwrap();

        if optional_pes_header == 0x02 {
          let mut pts = None;
          let mut dts = None;
          let mut escr = None;
          let mut es_rate = None;
          let mut dsm_trick_mode = None;
          let mut additional_copy_info = None;
          let mut previous_pes_packet_crc = None;
          let pes_extension = None;

          let scrambling_control = stream.read::<u8>(2).unwrap();
          let priority = stream.read_bit().unwrap();
          let data_alignment_indicator = stream.read_bit().unwrap();
          let copyright = stream.read_bit().unwrap();
          let original = stream.read_bit().unwrap();

          let pts_presence = stream.read_bit().unwrap();
          let dts_presence = stream.read_bit().unwrap();
          let escr_flag = stream.read_bit().unwrap();
          let es_rate_flag = stream.read_bit().unwrap();
          let dsm_trick_mode_flag = stream.read_bit().unwrap();
          let additional_copy_info_flag = stream.read_bit().unwrap();
          let crc_flag = stream.read_bit().unwrap();
          let extension_flag = stream.read_bit().unwrap();
          let pes_header_length = stream.read::<u8>(8).unwrap();

          *count += 3;

          if pts_presence {
            let _pts_tag = stream.read::<u8>(4).unwrap();
            let pts_high = stream.read::<u8>(3).unwrap();
            let _marker = stream.read::<u8>(1).unwrap();
            let pts_middle = stream.read::<u16>(15).unwrap();
            let _marker = stream.read::<u8>(1).unwrap();
            let pts_low = stream.read::<u16>(15).unwrap();
            let _marker = stream.read::<u8>(1).unwrap();

            let pts_value = ((pts_high as u64) << 30) + 
                      ((pts_middle as u64) << 15) + 
                      ((pts_low as u64));

            pts = Some(pts_value);
            *count += 5;
          }

          if dts_presence {
            let _dts_tag = stream.read::<u8>(4).unwrap();
            let dts_high = stream.read::<u8>(3).unwrap();
            let _marker = stream.read::<u8>(1).unwrap();
            let dts_middle = stream.read::<u16>(15).unwrap();
            let _marker = stream.read::<u8>(1).unwrap();
            let dts_low = stream.read::<u16>(15).unwrap();
            let _marker = stream.read::<u8>(1).unwrap();
            let dts_value = ((dts_high as u64) << 30) + 
                      ((dts_middle as u64) << 15) + 
                      ((dts_low as u64));
            dts = Some(dts_value);
            *count += 5;
          }

          if escr_flag {
            let _reserved = stream.read::<u8>(2).unwrap();
            let escr_high = stream.read::<u8>(3).unwrap();
            let _market_bit = stream.read_bit().unwrap();
            let escr_middle = stream.read::<u16>(15).unwrap();
            let _market_bit = stream.read_bit().unwrap();
            let escr_low = stream.read::<u16>(15).unwrap();
            let _market_bit = stream.read_bit().unwrap();
            let escr_extension = stream.read::<u16>(9).unwrap();
            let _market_bit = stream.read_bit().unwrap();

            let escr_value = ((escr_high as u64) << 39) + 
                      ((escr_middle as u64) << 24) + 
                      ((escr_low as u64) << 9) + 
                      ((escr_extension as u64));

            escr = Some(escr_value);
            *count += 6;
          }

          if es_rate_flag {
            let _market_bit = stream.read_bit().unwrap();
            es_rate = Some(stream.read::<u32>(22).unwrap());
            let _market_bit = stream.read_bit().unwrap();
            *count += 3;
          }

          if dsm_trick_mode_flag {
            let mode =
              match stream.read::<u8>(3).unwrap() {
                0b000 => TrickModeControl::FastForward,
                0b001 => TrickModeControl::SlowMotion,
                0b010 => TrickModeControl::FreezeFrame,
                0b011 => TrickModeControl::FastReverse,
                0b100 => TrickModeControl::SlowReverse,
                    _ => TrickModeControl::Reserved,
              };

            let info = stream.read::<u8>(5).unwrap();

            dsm_trick_mode = Some(DsmTrickMode{
              trick_mode_control: mode,
              info: info
            });
          }
          if crc_flag {
            previous_pes_packet_crc = Some(stream.read::<u16>(16).unwrap())
          }
          if additional_copy_info_flag {
            let _market_bit = stream.read_bit().unwrap();
            additional_copy_info = Some(stream.read::<u8>(7).unwrap());
          }
          if extension_flag {
            // let pes_private_data_flag = stream.read_bit().unwrap();
            // let pack_header_field_flag = stream.read_bit().unwrap();
            // let program_packet_sequence_counter_flag = stream.read_bit().unwrap();
            // let p_std_buffer_flag = stream.read_bit().unwrap();
            // let _reserved  = stream.read::<u8>(2).unwrap();
            // let pes_extension_flag_2 = stream.read_bit().unwrap();

            // let mut pes_private_data = vec![];
            // if pes_private_data_flag {
            //   pes_private_data = vec![0; 16];
            //   stream.read_bytes(&mut pes_private_data).unwrap();
            // }

            // if pack_header_field_flag {
            //   let pack_header_length = stream.read::<u8>(8).unwrap();

            //   let mut some_data = vec![0; pack_header_length as usize];
            //   stream.read_bytes(&mut some_data).unwrap();
            // }

            // pes_extension = Some(PesExtension{
            //   pes_private_data: pes_private_data
            // });

            unimplemented!();
          }


          header = Some(PesHeader{
            scrambling_control: scrambling_control,
            priority: priority,
            data_alignment_indicator: data_alignment_indicator,
            copyright: copyright,
            original: original,
            pts: pts,
            dts: dts,
            escr: escr,
            es_rate: es_rate,
            dsm_trick_mode: dsm_trick_mode,
            additional_copy_info: additional_copy_info,
            previous_pes_packet_crc: previous_pes_packet_crc,
            pes_extension: pes_extension,
            pes_header_length: pes_header_length
          });
        } else {
          let _more = stream.read::<u8>(6).unwrap();
          *count += 1;
        }
      } else {
        unimplemented!();
      }

      if pes_packet_length > 0 {

        let l = cmp::min(pes_packet_length, 184 - *count as u16);
        additional_data = vec![0; l as usize];
        stream.read_bytes(&mut additional_data).unwrap();

        *count += l as usize;
      }

      pes = Some(PacketizedElementaryStream{
        stream_id: get_stream_id(es_id),
        header: header,
        additional_data: additional_data,
      });
    },
    (0xFF, _, _) => {
      return None
    },
    (pointer_size, table, next) => {
      if pointer_size > 0x00 {
        println!("POINTER {}", pointer_size);
        return None
      }

      let mut syntax_section_length = stream.read::<u8>(8).unwrap() as u16;
      *count += 1;
      syntax_section_length += ((next & 0x03) as u16) << 8;

      match get_table_id(table) {
        TableId::ProgramAssociation => {
          pat = Some(parse_program_association(&mut stream, &mut count, syntax_section_length));
        },
        TableId::ProgramMap => {
          pmt = Some(parse_program_map(&mut stream, &mut count, syntax_section_length));
        },
        _ => {}
      }
    },
  }

  Some(Payload{
    pat: pat,
    pmt: pmt,
    pes: pes
  })
}
