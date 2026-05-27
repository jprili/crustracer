use crate::vec3::{ Vec3 };
use crate::ray::{ Ray };
use crate::hittable::{ HitRecord, Hittable };

pub struct Sphere {
    centre: Vec3,
    rad:    f64
}

impl Sphere {
    pub fn new(centre: Vec3, rad: f64) -> Self {
        Self {
            centre: centre, 
            rad: if rad > 0. { rad } else { 0. }
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self, 
        ray:    Ray, 
        r_tmin: f64,
        r_tmax: f64, 
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
        let not_contains = |r| r <= r_tmin || r_tmax <= r;
        if not_contains(root) {
            root = (h + dsqrt) / a;
            if not_contains(root) {
                return false
            }
        } 

        rec.t = root;
        rec.p = (*(&ray)).at(rec.t);
        rec.set_face_normal(
            ray,
            (rec.p - self.centre) / self.rad
        );
        return true
    }
}
