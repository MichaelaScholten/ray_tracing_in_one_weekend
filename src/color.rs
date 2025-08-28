use std::array;

use image::Rgb;

use crate::vec3::Vec3;

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

impl From<Color> for Rgb<u8> {
    fn from(value: Color) -> Self {
        Self(array::from_fn(|i| {
            (linear_to_gamma(value[i]).clamp(0.0, 0.999) * 256.0) as u8
        }))
    }
}
