extern crate mpegts;

use mpegts::wrapper::*;
use mpegts::writer::continuity_counter::ContinuityCounter;
use mpegts::writer::packet::write_packets;
use std::fs::File;

fn main() {
    let mut output_file = File::create("output.ts").unwrap();

    // let program = wrapper::Program{
    //   id: 301
    // };

    let wrapper = wrapper::Wrapper { programs: vec![] };

    let packets = wrapper.append_data(vec![0; 100]);

    let mut cc = ContinuityCounter { streams: vec![] };
    write_packets(&mut output_file, &packets, &mut cc);
}
