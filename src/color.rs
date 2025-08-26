use std::{array, io::Write};

use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color: &Color) {
    let color: [u8; 3] = array::from_fn(|i| (pixel_color[i] * 255.999).clamp(0.0, 255.0) as u8);
    writeln!(out, "{} {} {}", color[0], color[1], color[2]).unwrap();
}
