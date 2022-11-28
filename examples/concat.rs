extern crate mpegts;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::process;

use mpegts::parser::packet::parse_next_packets;
use mpegts::writer::continuity_counter::ContinuityCounter;
use mpegts::writer::packet::write_packets;

fn main() {
    if env::args().count() != 4 {
        println!("ERROR: missing filepath argument.");
        println!("usage:");
        println!("       dump [input1.ts] [input2.ts] [output.ts]");
        process::exit(0x0f00);
    }

    let source1_path = env::args().nth(1).unwrap();

    let mut sources = vec![env::args().nth(2).unwrap()];

    let output_path = env::args().nth(3).unwrap();

    let mut output_file = File::create(output_path).unwrap();

    let file = File::open(source1_path).unwrap();
    let mut stream = BufReader::new(file);

    let mut cc = ContinuityCounter { streams: vec![] };
    let mut count = 0;
    loop {
        match parse_next_packets(&mut stream) {
            Ok(packets) => {
                write_packets(&mut output_file, &packets, &mut cc);
                count += packets.len();

                print!("{:?}    \r", count);
            }
            Err(_msg) => match sources.pop() {
                Some(source) => {
                    let file = File::open(source).unwrap();
                    stream = BufReader::new(file);
                }
                None => {
                    println!("No more source");
                    return;
                }
            },
        }
    }
}
