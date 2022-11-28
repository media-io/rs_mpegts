use mpegts::packet::Packet;
use writer::adaptation_field::*;
use writer::continuity_counter::*;
use writer::payload::*;

use bitstream_io::{BigEndian, BitWriter};
use std::io::{Seek, SeekFrom, Write};

pub fn write_packets<W: Write + Seek>(
    stream: &mut W,
    packets: &Vec<Packet>,
    cc: &mut ContinuityCounter,
) {
    for packet in packets {
        let origin_position = stream.seek(SeekFrom::Current(0)).unwrap();

        write_packet(stream, packet, cc);

        let end_position = stream.seek(SeekFrom::Current(0)).unwrap();

        if end_position - origin_position != 188 {
            println!("packet size = {:?}", (end_position - origin_position));
            println!("{:?}", packet);
        }
        let fill_count = 188 - (end_position - origin_position);

        if fill_count > 0 {
            println!(
                "MpegTS Writer: wrong packet (with PID {}) length {:?}",
                packet.program_id,
                end_position - origin_position
            );
            println!("{:?}", packet);
            for _i in 0..fill_count {
                let mut fill = vec![0xFF];
                let _res = stream.write(&mut fill);
            }
        }
    }
}

pub fn write_packet<W: Write + Seek>(
    mut stream: &mut W,
    packet: &Packet,
    cc: &mut ContinuityCounter,
) {
    let mut writer = BitWriter::<BigEndian>::new(&mut stream);

    let mut continuity_counter = 0;

    match get_stream(cc, packet.program_id) {
        Some(counter) => {
            continuity_counter = counter;
        }
        None => cc.streams.push(Stream {
            id: packet.program_id,
            counter: continuity_counter,
        }),
    }

    // println!("Stream {} - cc {}", packet.program_id, continuity_counter);

    writer.write(8, 0x47).unwrap();

    writer.write_bit(packet.transport_error_indicator).unwrap();
    writer.write_bit(packet.payload.is_some()).unwrap();
    writer.write_bit(packet.transport_priority).unwrap();
    writer.write(13, packet.program_id).unwrap();
    writer
        .write(2, packet.transport_scrambling_control)
        .unwrap();
    writer.write_bit(packet.adaptation_field.is_some()).unwrap();
    writer.write_bit(packet.payload_presence).unwrap();
    writer.write(4, continuity_counter).unwrap();

    write_adaptation_field(&mut writer, &packet.adaptation_field);
    write_payload(&mut writer, &packet.payload);

    writer.write_bytes(&packet.data).unwrap();
}

fn get_stream(cc: &mut ContinuityCounter, program_id: u16) -> Option<u8> {
    let mut iter = cc.streams.iter_mut();

    loop {
        match iter.next() {
            Some(ref mut stream) => {
                if stream.id == program_id {
                    stream.counter += 1;
                    if stream.counter > 15 {
                        stream.counter = 0;
                    }
                    return Some(stream.counter);
                }
            }
            None => return None,
        }
    }
}
