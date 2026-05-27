use std::vec::{ Vec };
use crate::ray::{ Ray };
use crate::hittable::{ HitRecord, Hittable };

pub struct HittableList<'a, T: Hittable> {
    objects: Vec<&'a T> 
}

impl<'a, T: Hittable> HittableList<'a, T> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, h: &'a T) {
        self.objects.push(h);
    }
}

impl<'a, T: Hittable> Hittable for HittableList<'a, T> {
    fn hit(
        &self, 
        ray: Ray, 
        r_tmin: f64, 
        r_tmax: f64, 
        rec: &mut HitRecord
        ) -> bool {
        let record: &mut HitRecord = rec;
        let mut hit_anything:   bool = false; 
        let mut closest_so_far: f64  = r_tmax;

        for obj in &self.objects {
            if obj.hit(ray, r_tmin, closest_so_far, record) {
                hit_anything   = true;
                closest_so_far = record.t;
                // TODO: fix the borrowing problem here
                rec.set(record.clone());
            }
        }
        hit_anything
    }
}
