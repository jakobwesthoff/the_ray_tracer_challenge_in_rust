use core::slice;
use std::cmp::min;

use crate::F;

pub struct LinearScale {
  domain: (F, F),
  range: Vec<F>,
}

impl LinearScale {
  pub fn new() -> Self {
    Self {
      domain: (0.0, 100.0),
      range: vec![0.0, 1.0],
    }
  }

  pub fn with_domain(mut self, start: F, end: F) -> Self {
    self.domain = (start, end);
    self
  }

  pub fn with_range(mut self, range: Vec<F>) -> Self {
    self.range = range;
    self
  }

  pub fn scale(&self, input: F) -> F {
    let clamped_input = input.clamp(self.domain.0, self.domain.1);

    let normalized_input = self.normalize(clamped_input);
    let output = self.interpolate(normalized_input);

    output.clamp(self.minimum_of_range(), self.maximum_of_range())
  }

  fn minimum_of_range(&self) -> F {
    let mut minimum = F::INFINITY;
    for value in self.range.iter() {
      minimum = minimum.min(*value);
    }

    minimum
  }

  fn maximum_of_range(&self) -> F {
    let mut maximum = -F::INFINITY;
    for value in self.range.iter() {
      maximum = maximum.max(*value);
    }
    maximum
  }

  fn normalize(&self, input: F) -> F {
    let offsetted_input = input - self.domain.0;
    offsetted_input / (self.domain.1 - self.domain.0)
  }

  fn interpolate(&self, normalized_input: F) -> F {
    let slice_count = self.range.len() - 1;
    let slice_index = (normalized_input * slice_count as F).floor() as usize;
    let slice_normalized_input = normalized_input * slice_count as F - slice_index as F;

    let offseted_output =
      slice_normalized_input * (self.range[slice_index + 1] - self.range[slice_index]).abs();

    if self.range[slice_index] > self.range[slice_index + 1] {
      return self.range[slice_index] - offseted_output;
    } else {
      return self.range[slice_index] + offseted_output;
    }
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
