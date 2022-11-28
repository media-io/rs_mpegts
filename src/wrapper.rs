use mpegts::packet::Packet;
use mpegts::program_association::*;
use mpegts::program_descriptor::*;
use mpegts::program_map::*;
use mpegts::stream_id::StreamId;

#[derive(Debug)]
pub struct Wrapper {
    pub programs: Vec<Program>,
}

impl Wrapper {
    pub fn append_data(self, _data: Vec<u8>) -> Vec<Packet> {
        let program_map_pid = 256;
        let program_number = 1;
        let video_pid = 257;

        let pat = ProgramAssociation {
            transport_stream_id: 0,
            table: vec![Association {
                program_number,
                program_map_pid,
            }],
        };

        let pmt = ProgramMap {
            program_number,
            pcr_pid: video_pid,
            programs: vec![Program {
                stream_id: StreamId::Itu_T_H265_Video,
                elementary_pid: video_pid,
                es_info: EsInfo {
                    descriptor: ProgramDescriptor::Reserved,
                    hevc: None,
                    data: vec![],
                },
            }],
        };

        let pat_packet = Packet::new_pat(pat);

        let pmt_packet = Packet::new_pmt(program_map_pid, pmt);

        let mut result = vec![];

        result.push(pat_packet);
        result.push(pmt_packet);

        for _i in 1..6 {
            result.push(Packet::new_null());
        }

        result
    }
}
