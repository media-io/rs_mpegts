#[derive(Debug, Clone)]
pub struct Association {
    pub program_number: u16,
    pub program_map_pid: u16,
}

#[derive(Debug, Clone)]
pub struct ProgramAssociation {
    pub transport_stream_id: u16,
    pub table: Vec<Association>,
}
