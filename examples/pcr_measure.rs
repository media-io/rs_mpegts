
extern crate mpegts;

use std::io::BufReader;
use std::fs::File;
use std::process;
use std::env;

use mpegts::parser::packet::parse_next_packets;
use mpegts::writer::continuity_pcr::ContinuityPcr;

const TS_PACKET_SIZE: usize = 188;
const SYSTEM_CLOCK_FREQUENCY: usize = 27000000;

fn main() {

  if env::args().count() != 2 {
    println!("ERROR: missing filepath argument.");
    println!("usage:");
    println!("       pcr_measure [filepath.ts]");
    process::exit(0x0f00);
  }

  let path = env::args().last().unwrap();

  let file = File::open(path).unwrap();
  let mut stream = BufReader::new(file);

  let mut ts_packet_count = 0;

  let mut continuity_pcr = ContinuityPcr{streams: vec![]};

  loop {
    let packets = parse_next_packets(&mut stream).unwrap();

    for packet in packets {
      if packet.program_id == 0 {
        continue;
      }
      if packet.adaptation_field.is_some() {
        let af = packet.adaptation_field.unwrap();
        if af.pcr.is_some() {
          let pcr = af.pcr.unwrap();

          let new_pcr = pcr.get();
          let new_pcr_index = (ts_packet_count * TS_PACKET_SIZE) + 10;

          match continuity_pcr.get(packet.program_id) {
            None => {},
            Some(pcr_stream) => {
              let instant_bitrate = ((new_pcr_index as i64 - pcr_stream.index as i64) * 8 * SYSTEM_CLOCK_FREQUENCY as i64) as f64 / (new_pcr as i64 - pcr_stream.pcr as i64) as f64;

              println!("{} bitrate = {:?}", packet.program_id, instant_bitrate as i64);
            },
          }

          continuity_pcr.update(packet.program_id, new_pcr, new_pcr_index);
        }
      }

      ts_packet_count += 1;
    }
  }
}
