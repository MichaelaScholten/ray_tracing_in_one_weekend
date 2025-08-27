use std::{
    array,
    io::{BufWriter, Write as _, stdout},
};

use crate::{
    color::{Color, write_color},
    hittable::{Hittable as _, list::List as HittableList},
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    image_width: usize,
    image_height: usize,
    center: Point3,
    pixel_origin_location: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(1.0, 100)
    }
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as usize;
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
        }
    }

    fn ray_color(ray: &Ray, world: &HittableList) -> Color {
        if let Some(record) = world.hit(ray, Interval::new(0.0, f64::INFINITY)) {
            return 0.5 * (*record.normal() + Color::new([1.0; 3]));
        }
        let unit_direction = ray.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(array::from_fn(|_| 1.0)) + a * Color::new([0.5, 0.7, 1.0])
    }

    pub fn render(&self, world: &HittableList) {
        let mut out = BufWriter::new(stdout().lock());
        writeln!(out, "P3\n{} {}\n255\n", self.image_width, self.image_height).unwrap();
        for y in 0..self.image_height {
            eprint!("{:02}%\r", y * 100 / self.image_height);
            for x in 0..self.image_width {
                let pixel_center = self.pixel_origin_location
                    + x as f64 * self.pixel_delta_u
                    + y as f64 * self.pixel_delta_v;
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let pixel_color = Self::ray_color(&ray, world);
                write_color(&mut out, &pixel_color);
            }
        }
    }
}
