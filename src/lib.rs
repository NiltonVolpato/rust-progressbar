pub struct ProgressBar {
  current_value: u32,
  maximum_value: u32,
}

impl ProgressBar {
  pub fn new() -> ProgressBar {
    ProgressBar { maximum_value: 100 }
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
