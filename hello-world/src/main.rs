extern crate color;
extern crate semver;

use color::{Rgb, ToHsv};
use semver::Version;

fn main() {
  println!("Create RGB from HSV");
  let red = Rgb::new(255u8, 0, 0);
  println!("HSV {}", red.to_hsv::<f32>());

  assert!(Version::parse("1.2.3") == Ok(Version {
  	major: 1u,
  	minor: 2u,
  	patch: 3u,
  	pre: vec!(),
  	build: vec!(),
  }));
  println!("Version comparision success!");
}
