#[derive(Debug, Clone)]
pub struct Aac {
    pub profile_and_level: u8,
    pub aac_type: Option<u8>,
    pub additional_info: Vec<u8>,
}
