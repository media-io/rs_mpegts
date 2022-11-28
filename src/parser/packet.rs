use mpegts::packet::Packet;

use parser::adaptation_field::parse_adaptation_field;
use parser::payload::parse_payload;

use bitstream_io::{BigEndian, BitReader};
use std::io::{Cursor, Read};

pub fn parse_next_packets<R: Read>(stream: &mut R) -> Result<Vec<Packet>, String> {
    let mut buffer = [0u8; 1316];
    let data = stream.read_exact(&mut buffer);
    match data {
        Err(message) => Err(message.to_string()),
        Ok(_) => Ok(parse_some_packets(&buffer)),
    }
}

fn parse_some_packets(packet: &[u8]) -> Vec<Packet> {
    let mut position = 0;

    let mut cursor = Cursor::new(&packet);
    let mut reader = BitReader::<BigEndian>::new(&mut cursor);

    let mut packets = vec![];

    while position + 187 < packet.len() {
        let sync_byte = reader.read::<u8>(8).unwrap();
        if sync_byte != 0x47 {
            panic!("MPEGTS: bad sync byte: 0x{:x} != 0x47", sync_byte);
        }
        let transport_error_indicator = reader.read_bit().unwrap();
        let payload_unit_start_indicator = reader.read_bit().unwrap();
        let transport_priority = reader.read_bit().unwrap();
        let program_id = reader.read::<u16>(13).unwrap();
        let transport_scrambling_control = reader.read::<u8>(2).unwrap();
        let adaptation_field_presence = reader.read_bit().unwrap();
        let payload_presence = reader.read_bit().unwrap();
        let continuity_counter = reader.read::<u8>(4).unwrap();

        let mut packet = Packet {
            transport_error_indicator,
            transport_priority,
            program_id,
            transport_scrambling_control,
            continuity_counter,
            payload_presence,
            adaptation_field: None,
            payload: None,
            data: vec![],
        };

        let mut count = 0;
        if adaptation_field_presence {
            packet.adaptation_field = parse_adaptation_field(&mut reader, &mut count);
        }
        let tmp = count;
        if payload_unit_start_indicator {
            packet.payload = parse_payload(&mut reader, &mut count);
        }
        if count > 184 {
            println!("packet {:?}", packet);
            println!("readed data = {:?} // {}", count, tmp);
        }
        let data_length: usize = 184 - count as usize;

        if payload_presence {
            let mut data = vec![0; data_length];
            reader.read_bytes(&mut data).unwrap();

            let mut fill_data = true;
            for byte in data.clone() {
                if byte != 0xff {
                    fill_data = false;
                }
            }
            if !fill_data && program_id != 8191 {
                packet.data = data;
            }
        } else {
            let mut data = vec![0; data_length];
            reader.read_bytes(&mut data).unwrap();
            packet.data = data;

            // reader.skip((data_length * 8) as u32).unwrap();
        }
        // println!("LOAD DATA {:?}", data_length);

        position += 188;
        packets.push(packet);
    }

    packets
}
