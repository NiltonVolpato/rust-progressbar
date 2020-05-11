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
