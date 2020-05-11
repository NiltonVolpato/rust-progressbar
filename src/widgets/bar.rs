use super::{FixedWidthWidget, HorizontalFillWidget, Literal, WidgetHolder};
use crate::state::State;

// Renders a customizable progress bar that fills the available width.
// For instance, a bar like:
//   Bar { left: "[", marker: "#", fill: " ", right: "]"}
// would render like:
//   [#################                         ]
pub struct Bar {
  left: Box<dyn FixedWidthWidget>,
  marker: Box<dyn FixedWidthWidget>,
  fill: Box<dyn FixedWidthWidget>,
  right: Box<dyn FixedWidthWidget>,
}

impl Bar {
  pub fn new() -> WidgetHolder {
    WidgetHolder::HorizontalFill(Box::new(Bar {
      left: Literal::new_box("[".into()),
      marker: Literal::new_box("#".into()),
      fill: Literal::new_box(" ".into()),
      right: Literal::new_box("]".into()),
    }))
  }
}

impl HorizontalFillWidget for Bar {
  fn render(&self, state: &State, size: usize) -> String {
    let left = self.left.render(state);
    let marker = self.marker.render(state);
    let fill = self.fill.render(state);
    let right = self.right.render(state);

    let remaining_size = size - left.len() - right.len();
    let marked_size = (state.fraction() * remaining_size as f64) as usize;
    [
      left,
      marker.repeat(marked_size),
      fill.repeat(remaining_size - marked_size),
      right,
    ]
    .concat()
  }
}
