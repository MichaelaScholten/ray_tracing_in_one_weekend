use crate::{color::Color, hittable::HitRecord, material::Material, ray::Ray, vec3::Vec3};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub const fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = *record.normal() + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = *record.normal();
        }
        Some((Ray::new(*record.point(), scatter_direction), self.albedo))
    }
}
