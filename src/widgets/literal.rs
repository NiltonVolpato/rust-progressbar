use super::{FixedWidthWidget, WidgetHolder};
use crate::state::State;

pub struct Literal {
  value: String,
}

impl Literal {
  pub fn new(value: &'static str) -> WidgetHolder {
    WidgetHolder::FixedWidth(Box::new(Literal {
      value: String::from(value),
    }))
  }
  pub fn new_box(value: String) -> Box<Literal> {
    Box::new(Literal { value: value })
  }
}

impl FixedWidthWidget for Literal {
  fn render(&self, _state: &State) -> String {
    self.value.clone()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_fixed_width() {
    assert!(if let WidgetHolder::FixedWidth(_) = Literal::new("abc") {
      true
    } else {
      false
    });
  }

  #[test]
  fn works() {
    assert_eq!(
      Literal {
        value: String::from("abc")
      }
      .render(&State::new(100)),
      "abc"
    );
  }
}
