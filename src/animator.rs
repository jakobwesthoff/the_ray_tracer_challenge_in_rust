use crate::F;

pub struct LinearScale {
  domain: (F, F),
  range: (F, F),
}

impl LinearScale {
  pub fn new() -> Self {
    Self {
      domain: (0.0, 100.0),
      range: (0.0, 1.0),
    }
  }

  pub fn with_domain(mut self, start: F, end: F) -> Self {
    self.domain = (start, end);
    self
  }

  pub fn with_range(mut self, start: F, end: F) -> Self {
    self.range = (start, end);
    self
  }

  pub fn scale(&self, input: F) -> F {
    let clamped_input = input.clamp(self.domain.0, self.domain.1);
    let offsetted_input = clamped_input - self.domain.0;
    let normalized_input = offsetted_input / (self.domain.1 - self.domain.0);
    let offseted_output = normalized_input * (self.range.1 - self.range.0);
    let output = self.range.0 + offseted_output;
    output.clamp(self.range.0, self.range.1)
  }
}

pub struct Animator {
  frame_count: usize,
}

pub struct Frame {
  current: usize,
  count: usize,
}

impl Frame {
  pub fn new(count: usize, current: usize) -> Self {
    Self { count, current }
  }

  pub fn filename(&self, path: &str, name: &str, suffix: &str) -> String {
    format!("{}/{}{:06}{}", path, name, self.current, suffix)
  }

  pub fn linear_scale(&self) -> LinearScale {
    LinearScale::new().with_domain(0.0, self.count as F)
  }

  pub fn current(&self) -> usize {
    self.current
  }

  pub fn current_as_float(&self) -> F {
    self.current as F
  }
}

impl Animator {
  pub fn new(frame_count: usize) -> Self {
    Self { frame_count }
  }

  pub fn animate(&self, animate: fn(frame: Frame)) {
    for current_frame in 0..self.frame_count {
      animate(Frame::new(self.frame_count, current_frame));
    }
  }
}
