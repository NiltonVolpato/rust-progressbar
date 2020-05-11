use super::{FixedWidthWidget, WidgetHolder};
use crate::state::State;

pub struct Percentage {}

impl Percentage {
  pub fn new() -> WidgetHolder {
    WidgetHolder::FixedWidth(Box::new(Percentage {}))
  }
}

impl FixedWidthWidget for Percentage {
  fn render(&self, state: &State) -> String {
    format!("{:3.0}%", state.percentage())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn state(value: usize) -> State {
    let mut state = State::new(100);
    state.current_value = value;
    state
  }

  #[test]
  fn is_fixed_width() {
    assert!(if let WidgetHolder::FixedWidth(_) = Percentage::new() {
      true
    } else {
      false
    });
  }

  #[test]
  fn works() {
    assert_eq!(Percentage {}.render(&state(0)), "  0%");
    assert_eq!(Percentage {}.render(&state(50)), " 50%");
    assert_eq!(Percentage {}.render(&state(100)), "100%");
  }
}
