use std::vec::{ Vec };
use crate::constants::*;
use crate::hittable::{ HitRecord, Hittable };

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>> 
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, h: Box<dyn Hittable>) {
        self.objects.push(h);
    }
}

impl Hittable for HittableList {
    fn hit(
        &self, 
        ray: Ray, 
        interval: Interval,
        rec: &mut HitRecord
        ) -> bool {
        let record: &mut HitRecord = &mut HitRecord::new();
        let mut hit_anything:   bool = false; 
        let mut closest_so_far: f64  = interval.max;

        for obj in &self.objects {
            if obj.hit(ray, Interval::new(interval.min, closest_so_far), record) {
                hit_anything   = true;
                closest_so_far = record.t;
                rec.set(record.clone());
            }
        }
        hit_anything
    }
}
