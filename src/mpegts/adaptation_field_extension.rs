
#[derive(Debug, Clone)]
pub struct AdaptationFieldExtension {
  pub legal_time_window: Option<u16>,
  pub piecewise_rate: Option<u32>,
  pub seamless_splice: Option<u64>
}
