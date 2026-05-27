use std::vec::{ Vec };
use crate::ray::{ Ray };
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
        r_tmin: f64, 
        r_tmax: f64, 
        rec: &mut HitRecord
        ) -> bool {
        let record: &mut HitRecord = &mut HitRecord::new();
        let mut hit_anything:   bool = false; 
        let mut closest_so_far: f64  = r_tmax;

        for obj in &self.objects {
            if obj.hit(ray, r_tmin, closest_so_far, record) {
                hit_anything   = true;
                closest_so_far = record.t;
                rec.set(record.clone());
            }
        }
        hit_anything
    }
}
