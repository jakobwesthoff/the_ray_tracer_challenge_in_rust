use crate::canvas::to_rgba32::ToRGBA32;
use crate::canvas::Sized;

pub trait ToPPM {
  fn create_ppm_header(&self) -> Vec<u8>
  where
    Self: Sized,
  {
    let mut header = Vec::new();
    header.extend(String::from("P3\n").into_bytes());
    header.extend(format!("{} {}\n", self.width(), self.height()).into_bytes());
    header.extend(format!("{}\n", 255).into_bytes());

    return header;
  }

  fn to_ppm(&self) -> Vec<u8>;
}

impl<T> ToPPM for T
where
  T: ToRGBA32,
  T: Sized,
{
  fn to_ppm(&self) -> Vec<u8> {
    let mut last_image_row: usize = 0;
    let mut column_count: usize = 0;
    let pixel_data = self
      .to_rgba32()
      .into_iter()
      .map(|byte| format!("{}", byte))
      .enumerate()
      .filter(|(i, _)| (i + 1) % 4 != 0)
      .enumerate()
      .flat_map(|(i, (_, pixel_string))| {
        let mut data: Vec<u8> = Vec::new();
        // Line break for each row
        let current_image_row = i / (self.width() * 3);
        if current_image_row != last_image_row {
          last_image_row = current_image_row;
          data.extend(String::from("\n").into_bytes());
          column_count = 0;
        }

        let mut needed_space: usize = 0;

        if column_count != 0 {
          needed_space += 1; // space
        }
        needed_space += pixel_string.len();

        // Do not exceed 70 characters per line
        if column_count + needed_space > 70 {
          data.extend(String::from("\n").into_bytes());
          column_count = 0;
        }

        if column_count != 0 {
          data.extend(String::from(" ").into_bytes());
          column_count += 1;
        }

        data.extend(pixel_string.clone().into_bytes());
        column_count += pixel_string.len();

        data
      });

    self
      .create_ppm_header()
      .into_iter()
      .chain(pixel_data)
      .chain(String::from("\n").into_bytes())
      .collect()
  }
}
