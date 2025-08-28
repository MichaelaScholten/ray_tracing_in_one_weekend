use std::{sync::Arc, time::Instant};

use crate::{
    camera::Camera,
    color::Color,
    hittable::{Hittable, list::List as HittableList, sphere::Sphere},
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    vec3::Point3,
};

pub mod camera;
pub mod color;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod ray;
pub mod vec3;

fn main() {
    let start = Instant::now();

    {
        // Create the world
        let world: Vec<Box<dyn Hittable + Sync>> = vec![
            // Ground
            Box::new(Sphere::new(
                Point3::new([0.0, -100.5, -1.0]),
                100.0,
                Arc::new(Lambertian::new(Color::new([0.8, 0.8, 0.0]))),
            )),
            // Center
            Box::new(Sphere::new(
                Point3::new([0.0, 0.0, -1.2]),
                0.5,
                Arc::new(Lambertian::new(Color::new([0.1, 0.2, 0.5]))),
            )),
            // Left
            Box::new(Sphere::new(
                Point3::new([-1.0, 0.0, -1.0]),
                0.5,
                Arc::new(Dielectric::new(1.5)),
            )),
            // Right
            Box::new(Sphere::new(
                Point3::new([1.0, 0.0, -1.0]),
                0.5,
                Arc::new(Metal::new(Color::new([0.8, 0.6, 0.2]), 1.0)),
            )),
        ];
        let world = HittableList::from(world);

        // Create the camera
        let camera = Camera::new(16.0 / 9.0, 1024, 128, 50);

        // Use the camera to make a picture of the world
        camera.render(&world);
    }

    eprintln!("{:?}", start.elapsed());
}
