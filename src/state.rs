pub struct State {
  pub current_value: usize,
  pub maximum_value: usize, // XXX: Use Option<usize> for unknown length
}

impl State {
  pub fn fraction(&self) -> f64 {
    if self.current_value >= self.maximum_value {
      1.0
    } else {
      self.current_value as f64 / self.maximum_value as f64
    }
  }

  pub fn percentage(&self) -> f64 {
    100.0 * self.fraction()
  }
}
