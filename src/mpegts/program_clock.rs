
#[derive(Debug, Clone)]
pub struct ProgramClock {
  pub base: u64,
  pub extension: u16
}

impl ProgramClock {
  pub fn get(&self) -> u64 {
    return self.base * 300 + self.extension as u64;
  }
}
