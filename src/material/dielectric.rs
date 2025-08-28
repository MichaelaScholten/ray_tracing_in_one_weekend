use crate::{color::Color, material::Material, ray::Ray};

pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media.
    refraction_index: f64,
}

impl Dielectric {
    pub const fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &crate::ray::Ray,
        record: &crate::hittable::HitRecord,
    ) -> Option<(crate::ray::Ray, crate::color::Color)> {
        let refraction_index = if record.front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction().unit_vector();
        let refracted = unit_direction.refract(*record.normal(), refraction_index);

        let attenuation = Color::new([1.0; 3]);
        let scattered = Ray::new(*record.point(), refracted);
        Some((scattered, attenuation))
    }
}
