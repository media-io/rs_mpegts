pub struct Stream {
    pub id: u16,
    pub counter: u8,
}

pub struct ContinuityCounter {
    pub streams: Vec<Stream>,
}
