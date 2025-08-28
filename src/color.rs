use std::array;

use image::Rgb;

use crate::vec3::Vec3;

pub type Color = Vec3;

impl From<Color> for Rgb<u8> {
    fn from(value: Color) -> Self {
        Self(array::from_fn(|i| {
            (value[i].clamp(0.0, 0.999) * 256.0) as u8
        }))
    }
}
