mod state;
mod terminal;
mod widgets;

pub use crate::state::State;
use crate::terminal::get_terminal_width;
pub use crate::widgets::{
  Bar, FixedWidthWidget, HorizontalFillWidget, Literal, Percentage, WidgetHolder,
};
use std::io;
use std::io::Write;

pub struct ProgressBar {
  state: State,
  widgets: Vec<WidgetHolder>,
  out: io::Stdout,
  terminal_width: usize,
}

impl ProgressBar {
  pub fn new() -> ProgressBar {
    let out = io::stdout();
    let terminal_width = get_terminal_width(&out);
    ProgressBar {
      state: State {
        current_value: 0,
        maximum_value: 100,
      },
      widgets: vec![Percentage::new(), Literal::new(" "), Bar::new()],
      out: out,
      terminal_width: terminal_width,
    }
  }

  pub fn update(&mut self, value: usize) {
    self.state.current_value = value;
    write!(self.out, "{}\r", self.render()).unwrap();
    io::stdout().flush().unwrap();
  }

  pub fn finish(&mut self) {
    self.update(self.state.maximum_value);
    write!(self.out, "\n").unwrap();
  }

  fn render(&self) -> String {
    let mut remaining_width = self.terminal_width - 1;
    let mut rendered: Vec<String> = Vec::new();
    let mut indexes: Vec<usize> = Vec::new();

    // First render all fixed width widgets.
    for (i, widget) in self.widgets.iter().enumerate() {
      rendered.push(match widget {
        WidgetHolder::Literal(s) => {
          remaining_width -= s.len();
          s.clone()
        }
        WidgetHolder::FixedWidth(w) => {
          let s = w.render(&self.state);
          remaining_width -= s.len();
          s
        }
        WidgetHolder::HorizontalFill(_) => {
          indexes.push(i);
          String::new()
        }
      })
    }

    if indexes.len() > 0 {
      // Now render all horizontal fill widgets
      let fill_width = remaining_width / indexes.len();
      for i in indexes {
        if let WidgetHolder::HorizontalFill(w) = &self.widgets[i] {
          rendered[i] = w.render(&self.state, fill_width);
        }
      }
    }

    rendered.join("")
  }
}

// TODO(nilton): Implement this later.
// impl Iterator for ProgressBar {
//   type Item = usize; // XXX
//   fn next(&mut self) -> Option<Self::Item> {
//     return Some(0);
//   }
// }

#[cfg(test)]
mod tests {
  use crate::ProgressBar;

  #[test]
  fn render_works() {
    let mut bar = ProgressBar::new();
    bar.state.current_value = 50;
    bar.terminal_width = 20;

    assert_eq!(bar.render(), " 50% [######      ]");
  }

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
