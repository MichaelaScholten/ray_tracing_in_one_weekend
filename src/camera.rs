use std::array;

use image::{ImageBuffer, Rgb};
use rand::random;

use crate::{
    color::Color,
    hittable::{Hittable as _, list::List as HittableList},
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    image_width: u32,
    image_height: u32,
    center: Point3,
    pixel_origin_location: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u8,
    pixel_samples_scale: f64,
    max_depth: u8,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(1.0, 100, 10, 10)
    }
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u8, max_depth: u8) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        if image_height < 1 {
            image_height = 1;
        }

        let center = Point3::new([0.0; 3]);

        // Determine viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new([viewport_width, 0.0, 0.0]);
        let viewport_v = Vec3::new([0.0, -viewport_height, 0.0]);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            center - Vec3::new([0.0, 0.0, focal_length]) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_origin_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        Self {
            image_width,
            image_height,
            center,
            pixel_origin_location,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / f64::from(samples_per_pixel),
            max_depth,
        }
    }

    fn sample_square() -> Vec3 {
        Vec3::new([random::<f64>() - 0.5, random::<f64>() - 0.5, 0.0])
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location x, y.

        let offset = Self::sample_square();
        let pixel_sample = self.pixel_origin_location
            + (x as f64 + offset.x()) * self.pixel_delta_u
            + (y as f64 + offset.y()) * self.pixel_delta_v;

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn ray_color(ray: &Ray, world: &HittableList, depth: u8) -> Color {
        if depth == 0 {
            return Color::default();
        }
        if let Some(record) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            return record.material().scatter(ray, &record).map_or_else(
                Color::default,
                |(scattered, attenuation)| {
                    attenuation * Self::ray_color(&scattered, world, depth - 1)
                },
            );
        }
        let unit_direction = ray.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(array::from_fn(|_| 1.0)) + a * Color::new([0.5, 0.7, 1.0])
    }

    pub fn render(&self, world: &HittableList) {
        ImageBuffer::from_par_fn(self.image_width, self.image_height, |x, y| {
            if x == 0 {
                eprint!("{:02}%\r", y * 100 / self.image_height);
            }
            let pixel_color = (0..self.samples_per_pixel)
                .map(|_| Self::ray_color(&self.get_ray(x, y), world, self.max_depth))
                .sum::<Color>()
                * self.pixel_samples_scale;
            Rgb::from(pixel_color)
        })
        .save("image.png")
        .unwrap();
    }
}
