#![warn(clippy::missing_const_for_fn)]

use std::{sync::Arc, time::Instant};

use rand::{random, random_range};

use crate::{
    camera::Camera,
    color::Color,
    hittable::{Hittable, list::List as HittableList, sphere::Sphere},
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    vec3::{Point3, Vec3},
};

pub mod camera;
pub mod color;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod ray;
pub mod vec3;

const A_MAX: i8 = 11;
const B_MAX: i8 = 11;

fn main() {
    let start = Instant::now();

    // Create the world
    let big_spheres: [Box<dyn Hittable + Sync>; 4] = [
        // Ground
        Box::new(Sphere::new(
            Point3::new([0.0, -1000.0, 0.0]),
            1000.0,
            Arc::new(Lambertian::new(Color::new([0.5; 3]))),
        )),
        Box::new(Sphere::new(
            Point3::new([0.0, 1.0, 0.0]),
            1.0,
            Arc::new(Dielectric::new(1.5)),
        )),
        Box::new(Sphere::new(
            Point3::new([-4.0, 1.0, 0.0]),
            1.0,
            Arc::new(Lambertian::new(Color::new([0.4, 0.2, 0.1]))),
        )),
        Box::new(Sphere::new(
            Point3::new([4.0, 1.0, 0.0]),
            1.0,
            Arc::new(Metal::new(Color::new([0.7, 0.6, 0.5]), 0.0)),
        )),
    ];
    let world: Vec<Box<dyn Hittable + Sync>> = (-A_MAX..A_MAX)
        .flat_map(|a| (-B_MAX..B_MAX).map(move |b| (a, b)))
        .map(|(a, b)| {
            Point3::new([
                f64::from(a) + random_range(0.0..0.9),
                0.2,
                f64::from(b) + random_range(0.0..0.9),
            ])
        })
        .filter(|center| (*center - Point3::new([4.0, 0.2, 0.0])).length() > 0.9)
        .map(|center| -> Box<dyn Hittable + Sync> {
            Box::new(Sphere::new(
                center,
                0.2,
                match random_range(0.0..=1.0) {
                    // Diffuse
                    ..0.8 => Arc::new(Lambertian::new(random::<Color>() * random::<Color>())),

                    // Metal
                    0.8..0.95 => {
                        Arc::new(Metal::new(Color::random(0.5..1.0), random_range(0.0..0.5)))
                    }
                    // Glass
                    _ => Arc::new(Dielectric::new(1.5)),
                },
            ))
        })
        .chain(big_spheres)
        .collect();
    let world = HittableList::from(world);

    // Create the camera
    let camera = Camera::new(
        16.0 / 9.0,
        1920,
        500,
        50,
        20.0,
        Point3::new([13.0, 2.0, 3.0]),
        Point3::new([0.0; 3]),
        Vec3::new([0.0, 1.0, 0.0]),
        0.6,
        10.0,
    );

    // Use the camera to make a picture of the world
    camera.render(&world);

    eprintln!("{:?}", start.elapsed());
}
