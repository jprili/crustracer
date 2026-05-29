use crate::constants::*;
use crate::hittable::HitRecord;

pub struct Scatter {
    pub ray: Ray,
    pub att: Vec3,
}

pub trait Material {
    fn scatter(
        &self,
        r_in: Ray,
        rec:  HitRecord,
        att:  &mut Vec3,
        r_out: &mut Ray
    ) -> bool;
}

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self {
            albedo: albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: Ray,
        rec:  HitRecord,
        att:  &mut Vec3,
        r_out: &mut Ray
    ) -> bool
    {
        let dir: Vec3 = rec.norm + Vec3::rand_vec3_unit();
        r_out.set_dir(
            if dir.is_near_zero() { rec.norm } else {dir}
        );
        r_out.set_org(rec.p);
        att.set_vec(self.albedo);
        true
    }
}

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self {
            albedo: albedo
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: Ray,
        rec:  HitRecord,
        att:  &mut Vec3,
        r_out: &mut Ray
    ) -> bool
    {
        let dir = Vec3::reflect(r_in.dir, rec.norm);
        r_out.set_dir(dir);
        r_out.set_org(rec.p);
        att.set_vec(self.albedo);
        true
    }
}