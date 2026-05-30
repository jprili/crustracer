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
    fuzz:   f64
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo: albedo,
            fuzz: if fuzz < 1. { fuzz } else { 1. }
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
        let dir = 
            Vec3::reflect(r_in.dir, rec.norm).unit_vec()
            + (self.fuzz * Vec3::rand_vec3_unit());
        r_out.set_dir(dir);
        r_out.set_org(rec.p);
        att.set_vec(self.albedo);
        r_out.dir.dot(rec.norm) > 0.
    }
}

pub struct Dielectric {
    ref_idx: f64
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self {
            ref_idx: ref_idx
        }
    }

    pub fn get_ri(&self) -> f64 {
        self.ref_idx
    }

    fn reflectance(cos: f64, ri: f64) -> f64 {
        let r0 = (1. - ri) / (1. + ri);
        let r1 = r0 * r0;
        r1 + ((1. - r1) * (1. - cos).powi(5))
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: Ray,
        rec:  HitRecord,
        att:  &mut Vec3,
        r_out: &mut Ray
    ) -> bool
    {
        att.set(1.0, 1.0, 1.0);
        let ri: f64 = if rec.front_face {
            1. / self.ref_idx
        } else { self.ref_idx };
        let unit_dir: Vec3 = r_in.dir.unit_vec();
        let cos_th: f64 = (-unit_dir).dot(rec.norm).min(1.);
        let sin_th: f64 = (1. - cos_th * cos_th).sqrt();
        let rf = if 
            (ri * sin_th > 1.) 
            || (Self::reflectance(cos_th, ri) > rand_unit())
        {
            Vec3::reflect(unit_dir, rec.norm)
        } else {
            Vec3::refract( unit_dir,
                rec.norm,            // normal of hit surf
                ri               // ref idx
            )
        };

        *r_out =  Ray { org: rec.p, dir: rf };
        true
    }
}