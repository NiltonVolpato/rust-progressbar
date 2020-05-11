pub struct State {
  pub current_value: usize,
  pub maximum_value: usize, // XXX: Use Option<usize> for unknown length
}

impl State {
  pub fn new(maximum_value: usize) -> State {
    State {
      current_value: 0,
      maximum_value: maximum_value,
    }
  }

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

#[cfg(test)]
mod tests {
  use super::*;

  fn state(value: usize) -> State {
    let mut s = State::new(100);
    s.current_value = value;
    s
  }

  #[test]
  fn fraction_works() {
    assert_eq!(state(0).fraction(), 0.0);
    assert_eq!(state(50).fraction(), 0.5);
    assert_eq!(state(100).fraction(), 1.0);
    assert_eq!(state(101).fraction(), 1.0);
    assert_eq!(state(200).fraction(), 1.0);
  }

  #[test]
  fn percentage_works() {
    assert_eq!(state(0).percentage(), 0.0);
    assert_eq!(state(50).percentage(), 50.0);
    assert_eq!(state(100).percentage(), 100.0);
    assert_eq!(state(101).percentage(), 100.0);
    assert_eq!(state(200).percentage(), 100.0);
  }
}
