extern crate mpegts;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::process;

use mpegts::parser::packet::parse_next_packets;

fn main() {
    if env::args().count() != 2 {
        println!("ERROR: missing filepath argument.");
        println!("usage:");
        println!("       dump [filepath.ts]");
        process::exit(0x0f00);
    }

    let path = env::args().last().unwrap();

    let file = File::open(path).unwrap();
    let mut stream = BufReader::new(file);

    loop {
        let packets = parse_next_packets(&mut stream).unwrap();

        for packet in packets {
            // println!("{}", packet);
            println!("{:?}", packet);
        }
    }
}
