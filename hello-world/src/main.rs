extern crate color;
use color::{Rgb, ToHsv};

fn main() {
  println!("Create RGB from HSV");
  let red = Rgb::new(255u8, 0, 0);
  println!("HSV {}", red.to_hsv::<f32>());
}
