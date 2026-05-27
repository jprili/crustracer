use crate::vec3::{ Vec3 };
use crate::ray::{ Ray };

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p:    Vec3,
    pub norm: Vec3,

    //  at which t off the ray was the hit recorded
    pub t:    f64, 
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: Ray, out_norm: Vec3) {
        self.front_face = ray.dir.dot(out_norm) < 0.;
        self.norm = 
            if self.front_face { out_norm } else { -out_norm };
    }

    pub fn set(&mut self, record: HitRecord) {
        self.p = record.p;
        self.norm = record.norm;
    }
}

pub trait Hittable {
    fn hit(
        &self,
        ray: Ray, 
        r_tmin: f64, 
        r_tmax: f64, 
        rec: &mut HitRecord
    ) -> bool; 
}
