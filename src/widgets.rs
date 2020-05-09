use crate::state::State;

pub trait FixedWidthWidget {
  fn render(&self, state: &State) -> String;
}

pub trait HorizontalFillWidget {
  fn render(&self, state: &State, size: usize) -> String;
}

pub enum WidgetHolder {
  Literal(String),
  FixedWidth(Box<dyn FixedWidthWidget>),
  HorizontalFill(Box<dyn HorizontalFillWidget>),
}

pub struct Literal {
  value: String,
}

impl Literal {
  pub fn new(value: &'static str) -> WidgetHolder {
    WidgetHolder::FixedWidth(Box::new(Literal {
      value: String::from(value),
    }))
  }
}

impl FixedWidthWidget for Literal {
  fn render(&self, _state: &State) -> String {
    self.value.clone()
  }
}

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
      left: Box::new(Literal {
        value: String::from("["),
      }),
      marker: Box::new(Literal {
        value: String::from("#"),
      }),
      fill: Box::new(Literal {
        value: String::from(" "),
      }),
      right: Box::new(Literal {
        value: String::from("]"),
      }),
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
