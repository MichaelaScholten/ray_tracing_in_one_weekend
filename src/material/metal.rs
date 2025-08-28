use crate::{color::Color, hittable::HitRecord, material::Material, ray::Ray, vec3::Vec3};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub const fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(crate::ray::Ray, Color)> {
        let reflected = ray.direction().reflect(record.normal()).unit_vector()
            + (self.fuzz * Vec3::random_unit_vector());
        let scattered = Ray::new(*record.point(), reflected);
        (scattered.direction().dot(record.normal()) > 0.0).then_some((scattered, self.albedo))
    }
}
