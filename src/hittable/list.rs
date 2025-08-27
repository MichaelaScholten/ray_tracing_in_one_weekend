use std::ops::RangeInclusive;

use crate::ray::Ray;

use super::Hittable;

#[derive(Default)]
pub struct List {
    objects: Vec<Box<dyn Hittable>>,
}

impl List {
    pub fn new(object: Box<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl From<Vec<Box<dyn Hittable>>> for List {
    fn from(value: Vec<Box<dyn Hittable>>) -> Self {
        Self { objects: value }
    }
}

impl Hittable for List {
    fn hit(&self, ray: &Ray, mut ray_time: RangeInclusive<f64>) -> Option<super::HitRecord> {
        let mut record = None;
        for object in &self.objects {
            if let Some(hit_record) = object.hit(ray, ray_time.clone()) {
                ray_time = *ray_time.start()..=hit_record.time;
                record = Some(hit_record);
            }
        }
        record
    }
}
