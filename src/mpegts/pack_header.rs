
#[derive(Debug)]
pub struct PackHeader {
  pub system_clock_reference_base: u64,
  pub system_clock_reference_extension: u16,
  pub program_mux_rate: u32,
  pub stuffing_size: u8,
  // pub system_header: Option<SystemHeader>
}
