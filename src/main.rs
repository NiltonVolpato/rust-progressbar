use progressbar::{FixedWidthWidget, ProgressBar, State};
use std::{thread, time};

struct MyCustomWidget {}

impl FixedWidthWidget for MyCustomWidget {
  fn render(&self, _state: &State) -> String {
    return String::from("foo");
  }
}

fn main() {
  let mut bar = ProgressBar::new();
  for i in 1..100 {
    bar.update(i);
    thread::sleep(time::Duration::from_secs_f64(0.02));
  }
  bar.finish();
  println!("Done!");
}
