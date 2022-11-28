#[derive(Debug, Clone)]
pub struct PcrStream {
    pub program_id: u16,
    pub pcr: u64,
    pub index: usize,
}

#[derive(Debug)]
pub struct ContinuityPcr {
    pub streams: Vec<PcrStream>,
}

impl ContinuityPcr {
    pub fn get(&mut self, program_id: u16) -> Option<PcrStream> {
        self.streams
            .iter()
            .find_map(|stream| (stream.program_id == program_id).then(|| stream.clone()))
    }

    pub fn update(&mut self, program_id: u16, pcr: u64, index: usize) {
        for stream in &mut self.streams {
            if stream.program_id == program_id {
                stream.pcr = pcr;
                stream.index = index;
                return;
            }
        }
        self.streams.push(PcrStream {
            program_id,
            pcr,
            index,
        })
    }
}
