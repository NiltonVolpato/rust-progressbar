mod state;
mod terminal;
mod widgets;

pub use crate::state::State;
use crate::terminal::get_terminal_width;
pub use crate::widgets::{
  Bar, FixedWidthWidget, HorizontalFillWidget, Literal, Percentage, WidgetHolder,
};
use std::cell::RefCell;
use std::io;
use std::io::Write;
use std::rc::Rc;

pub struct ProgressBar {
  state: State,
  widgets: Vec<WidgetHolder>,
  out: Rc<RefCell<dyn std::io::Write>>,
  terminal_width: usize,
  finished: bool,
}

impl ProgressBar {
  pub fn new() -> ProgressBar {
    let out = io::stdout();
    let terminal_width = get_terminal_width(&out);
    ProgressBar {
      state: State::new(100),
      widgets: vec![Percentage::new(), Literal::new(" "), Bar::new()],
      out: Rc::new(RefCell::new(out)),
      terminal_width: terminal_width,
      finished: false,
    }
  }

  fn update_internal(&mut self, value: usize) {
    self.state.current_value = value;
    write!(self.out.borrow_mut(), "{}\r", self.render()).unwrap();
  }

  pub fn update(&mut self, value: usize) {
    if self.finished {
      panic!("trying to call update on a finished progress bar");
    }
    if value == self.state.maximum_value {
      self.finish();
      return;
    }
    self.update_internal(value);
    io::stdout().flush().unwrap();
  }

  pub fn finish(&mut self) {
    if self.finished {
      return;
    }
    self.update_internal(self.state.maximum_value);
    write!(self.out.borrow_mut(), "\n").unwrap();
    self.finished = true;
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
  use crate::*;
  use std::io::Cursor;

  #[test]
  fn works() {
    let out: Rc<RefCell<_>> = Rc::new(RefCell::new(Cursor::new(Vec::<u8>::new())));
    {
      let mut bar = ProgressBar::new();
      bar.state.maximum_value = 2;
      bar.terminal_width = 10;
      bar.out = out.clone();
      for i in 0..=2 {
        bar.update(i);
      }
      bar.finish();
    }
    let r = out.borrow();
    let output = std::str::from_utf8(r.get_ref()).unwrap();
    assert_eq!(
      output,
      ["  0% [  ]\r", " 50% [# ]\r", "100% [##]\r\n",].concat()
    );
  }

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
