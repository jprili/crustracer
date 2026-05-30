use std::rc::Rc;

use crate::constants::*;
use crate::material::{ Lambertian, Material };

#[derive(Clone)]
pub struct HitRecord {
    pub p:    Vec3,
    pub norm: Vec3,

    //  at which t off the ray was the hit recorded
    pub t:    f64, 
    pub front_face: bool,
    pub mat: Rc<dyn Material>
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Vec3::new(0., 0., 0.),
            norm: Vec3::new(0., 0., 0.), 
            t: 0.,
            front_face: false,
            mat: Rc::new(Lambertian::new(Vec3::default()))
        }
    }

    pub fn set_face_normal(&mut self, ray: Ray, out_norm: Vec3) {
        self.front_face = ray.dir.dot(out_norm) < 0.;
        self.norm = 
            if self.front_face { out_norm } else { -out_norm };
    }

    pub fn set(&mut self, record: HitRecord) {
        self.p = record.p;
        self.norm = record.norm;
        self.t = record.t;
        self.front_face = record.front_face;
        self.mat  = record.mat;
    }
}

pub trait Hittable {
    fn hit(
        &self,
        ray: Ray, 
        interval: Interval,
        rec: &mut HitRecord
    ) -> bool; 
}