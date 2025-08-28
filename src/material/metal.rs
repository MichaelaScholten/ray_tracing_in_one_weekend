use crate::{color::Color, hittable::HitRecord, material::Material, ray::Ray};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub const fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(crate::ray::Ray, Color)> {
        let reflected = ray.direction().reflect(record.normal());
        Some((Ray::new(*record.point(), reflected), self.albedo))
    }
}
