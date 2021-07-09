use super::Canvas;
use num_traits::Float;

pub trait ToRGBA32 {
  fn to_rgba32(&self) -> Vec<u8>;
}

impl<T> ToRGBA32 for Canvas<T>
where
  T: Float,
{
  fn to_rgba32(&self) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();
    for pixel in self.pixels.iter() {
      let clamped_color = pixel.clamp(T::zero(), T::one());
      let r: u8 = (clamped_color.red * T::from(255.0).unwrap())
        .round()
        .to_u8()
        .unwrap();
      let g: u8 = (clamped_color.green * T::from(255.0).unwrap())
        .round()
        .to_u8()
        .unwrap();
      let b: u8 = (clamped_color.blue * T::from(255.0).unwrap())
        .round()
        .to_u8()
        .unwrap();
      let a: u8 = 255;

      data.push(r);
      data.push(g);
      data.push(b);
      data.push(a);
    }
    data
  }
}
