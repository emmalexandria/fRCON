use std::io;

pub struct RCONShell {
  prompt: String,

  input_buf: String,
}


impl RCONShell {
  pub fn new(prompt: &str) -> RCONShell {
    RCONShell {
      prompt: String::from(prompt),
      input_buf: String::new()
    }
  }

  fn read_line(&mut self) {
    io::stdin().read_line(&mut self.input_buf).unwrap();
  }
}