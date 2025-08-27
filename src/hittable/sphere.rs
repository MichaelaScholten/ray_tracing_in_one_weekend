use std::ops::RangeInclusive;

use super::{HitRecord, Hittable};
use crate::{ray::Ray, vec3::Point3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub const fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_time: RangeInclusive<f64>) -> Option<HitRecord> {
        let origin_center = self.center - *ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(&origin_center);
        let c = origin_center.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (h - sqrt_discriminant) / a;
        if root <= *ray_time.start() || *ray_time.end() <= root {
            root = (h + sqrt_discriminant) / a;
            if root <= *ray_time.start() || *ray_time.end() <= root {
                return None;
            }
        }

        let point = ray.at(root);
        Some(HitRecord::new(
            point,
            (point - self.center) / self.radius,
            root,
            ray,
        ))
    }
}
