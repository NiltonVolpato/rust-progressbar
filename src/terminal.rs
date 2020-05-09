use std::env;
use std::io;
use std::os::unix::io::AsRawFd;

const DEFAULT_TERMINAL_WIDTH: usize = 80;

fn get_terminal_width_fallback() -> usize {
  match env::var("COLUMNS") {
    Ok(value) => value.parse::<usize>().unwrap_or(DEFAULT_TERMINAL_WIDTH),
    Err(_) => DEFAULT_TERMINAL_WIDTH,
  }
}

pub fn get_terminal_width(out: &io::Stdout) -> usize {
  let fd = out.as_raw_fd();
  if unsafe { libc::isatty(fd) != 1 } {
    return get_terminal_width_fallback();
  }

  let mut size = libc::winsize {
    ws_row: 0,
    ws_col: 0,
    ws_xpixel: 0,
    ws_ypixel: 0,
  };

  if unsafe { libc::ioctl(fd, libc::TIOCGWINSZ.into(), &mut size) } == -1 || size.ws_col <= 0 {
    return get_terminal_width_fallback();
  }

  size.ws_col as usize
}
