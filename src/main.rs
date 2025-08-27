use std::{
    array,
    io::{BufWriter, Write as _, stdout},
    time::Instant,
};

use crate::{
    color::{Color, write_color},
    hittable::{Hittable, list::List as HittableList, sphere::Sphere},
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub mod color;
pub mod hittable;
pub mod ray;
pub mod vec3;

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 4096;
const IMAGE_HEIGHT: usize = {
    let height = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    if height < 1 { 1 } else { height }
};

// Camera
const FOCAL_LENGTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
const CAMERA_CENTER: Point3 = Point3::new([0.0; 3]);

// Calculate the vectors across the horizontal and down the vertical viewport edges
const VIEWPORT_U: Vec3 = Vec3::new([VIEWPORT_WIDTH, 0.0, 0.0]);
const VIEWPORT_V: Vec3 = Vec3::new([0.0, -VIEWPORT_HEIGHT, 0.0]);

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    if let Some(record) = world.hit(ray, 0.0..=f64::INFINITY) {
        return 0.5 * (*record.normal() + Color::new([1.0; 3]));
    }
    let unit_direction = ray.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(array::from_fn(|_| 1.0)) + a * Color::new([0.5, 0.7, 1.0])
}

fn main() {
    let start = Instant::now();

    // Render
    {
        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = VIEWPORT_U / IMAGE_WIDTH as f64;
        let pixel_delta_v = VIEWPORT_V / IMAGE_HEIGHT as f64;
        let viewport_upper_left = CAMERA_CENTER
            - Vec3::new([0.0, 0.0, FOCAL_LENGTH])
            - VIEWPORT_U / 2.0
            - VIEWPORT_V / 2.0;
        let pixel_origin_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let world: Vec<Box<dyn Hittable>> = vec![
            Box::new(Sphere::new(Point3::new([0.0, 0.0, -1.0]), 0.5)),
            Box::new(Sphere::new(Point3::new([0.0, -100.5, -1.0]), 100.0)),
        ];
        let world = HittableList::from(world);

        let mut out = BufWriter::new(stdout().lock());
        writeln!(out, "P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").unwrap();
        for y in 0..IMAGE_HEIGHT {
            eprint!("{:02}%\r", y * 100 / IMAGE_HEIGHT);
            for x in 0..IMAGE_WIDTH {
                let pixel_center =
                    pixel_origin_location + x as f64 * pixel_delta_u + y as f64 * pixel_delta_v;
                let ray_direction = pixel_center - CAMERA_CENTER;
                let ray = Ray::new(CAMERA_CENTER, ray_direction);

                let pixel_color = ray_color(&ray, &world);
                write_color(&mut out, &pixel_color);
            }
        }
    }

    eprintln!("{:?}", start.elapsed());
}
