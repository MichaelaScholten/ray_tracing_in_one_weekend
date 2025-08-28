use std::time::Instant;

use crate::{
    camera::Camera,
    hittable::{Hittable, list::List as HittableList, sphere::Sphere},
    vec3::Point3,
};

pub mod camera;
pub mod color;
pub mod hittable;
pub mod interval;
pub mod ray;
pub mod vec3;

fn main() {
    let start = Instant::now();

    {
        // Create the world
        let world: Vec<Box<dyn Hittable + Sync>> = vec![
            Box::new(Sphere::new(Point3::new([0.0, 0.0, -1.0]), 0.5)),
            Box::new(Sphere::new(Point3::new([0.0, -100.5, -1.0]), 100.0)),
        ];
        let world = HittableList::from(world);

        // Create the camera
        let camera = Camera::new(16.0 / 9.0, 512, 128);

        // Use the camera to make a picture of the world
        camera.render(&world);
    }

    eprintln!("{:?}", start.elapsed());
}
