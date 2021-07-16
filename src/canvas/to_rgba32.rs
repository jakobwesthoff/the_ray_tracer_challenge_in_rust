use super::Canvas;

pub trait ToRGBA32 {
  fn to_rgba32(&self) -> Vec<u8>;
}

impl ToRGBA32 for Canvas {
  fn to_rgba32(&self) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();
    for pixel in self.pixels.iter() {
      let clamped_color = pixel.clamp(0.0, 1.0);
      let r: u8 = (clamped_color.red * 255.0).round() as u8;
      let g: u8 = (clamped_color.green * 255.0).round() as u8;
      let b: u8 = (clamped_color.blue * 255.0).round() as u8;
      let a: u8 = 255;

      data.push(r);
      data.push(g);
      data.push(b);
      data.push(a);
    }
    data
  }
}
