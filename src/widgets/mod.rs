mod bar;
mod literal;
mod percentage;

pub use self::bar::Bar;
pub use self::literal::Literal;
pub use self::percentage::Percentage;
use super::state::State;

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
