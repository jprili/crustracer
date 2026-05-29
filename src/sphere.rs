use std::rc::Rc;

use crate::constants::*;
use crate::hittable::{ HitRecord, Hittable };
use crate::material::Material;

pub struct Sphere {
    centre: Vec3,
    rad:    f64,
    mat:    Rc<dyn Material>
}

impl Sphere {
    pub fn new(centre: Vec3, rad: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            centre: centre, 
            rad: if rad > 0. { rad } else { 0. },
            mat: mat
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self, 
        ray:    Ray, 
        interval: Interval,
        rec:    &mut HitRecord
    ) -> bool {
        let oc: Vec3 = self.centre - ray.org; 
        let a: f64   = ray.dir.mag_sq();
        let h: f64   = ray.dir.dot(oc);
        let c: f64   = oc.mag_sq() - (self.rad * self.rad);

        let d: f64 = h * h - (a * c);
        if d < 0. { return false }

        let dsqrt = d.sqrt();
        let mut root = (h - dsqrt) / a;
        if !interval.surrounds(root) {
            root = (h + dsqrt) / a;
            if !interval.surrounds(root) {
                return false
            }
        } 

        rec.t = root;
        rec.p = ray.at(rec.t);
        rec.set_face_normal(
            ray,
            (rec.p - self.centre) / self.rad
        );
        rec.mat = self.mat.clone();
        return true
    }
}
