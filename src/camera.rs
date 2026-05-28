use std::io;
use std::io::Write;

use log::info;

use crate::hittable::{ Hittable, HitRecord };
use crate::colour;
use crate::constants::*;

pub struct Camera {
    pub aspect_ratio: f64,
    pub img_w:        i32,       // px
    pub sample_per_px: i32,      // 1 / px
    px_sample_scale:   f64,      // scale factor
    img_h: i32,                  // px
    centre: Vec3,                // camera centre
    origin: Vec3,                // camera origin
    px_du:  Vec3,                // width-normalised  px size
    px_dv:  Vec3                 // height-normalised px size
}

impl Camera {
    fn ray_colour<T: Hittable>(&self, ray: Ray, world: &T) -> Vec3 {
        let rec: &mut HitRecord = &mut HitRecord::new();
        if world.hit(ray, Interval::new(0., INF), rec) {
            let dir = Vec3::rand_on_hemisphere(rec.norm);
            return 0.5 * self.ray_colour(
                Ray::new(rec.p, dir), world
            )
        }

    let a = (Vec3::unit_vec(&ray.dir).y() + 1.) * 0.5;
    (Vec3::new(1., 1., 1.) * (1. - a))
        + (Vec3::new(0.5, 0.7, 1.) * a)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(
            rand_unit() - 0.5, 
            rand_unit() - 0.5, 
            0.
        )
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let sample = Self::sample_square();
        let px_sample = self.origin 
            + ((i as f64 + sample.x()) * self.px_du)
            + ((j as f64 + sample.y()) * self.px_dv);

        Ray::new(
            self.centre, 
            px_sample - self.centre
        )
    } 

    pub fn new(asp: f64, img_w: i32, s_p_px: i32) -> Self {
        let img_h = (img_w as f64 / asp) as i32;
        let focal_length: f64 = 1.;
        let v_h: f64 = 2.;
        let v_w: f64 = v_h * (img_w as f64 / img_h as f64);
        let v_u: Vec3 = Vec3::new(v_w, 0., 0.);
        let v_v: Vec3 = Vec3::new(0., -v_h, 0.);

        let centre: Vec3 = Vec3::new(0., 0., 0.);

        let v_ul = 
            centre 
            - Vec3::new(0., 0., focal_length)
            - (v_u / 2.) - (v_v / 2.);

        let px_du = v_u / (img_w as f64); 
        let px_dv = v_v / (img_h as f64);

        Self {
            aspect_ratio: asp,
            img_w: img_w,
            img_h: img_h,
            centre: centre,
            px_du: px_du,
            px_dv: px_dv,
            origin: v_ul + (0.5 * (px_du + px_dv)),
            sample_per_px: s_p_px,
            px_sample_scale: 1. / s_p_px as f64
        }
    }

    pub fn render<T: Hittable>(&self, world: &T) {
        // region RENDER
        println!("P3\n{} {}\n255", self.img_w, self.img_h);

        for j in 0..self.img_h {
            info!("\rScanlines remaining: {}", self.img_h - j);
            match io::stderr().flush() { _ => () };
            for i in 0..self.img_w { 
                let mut px_colour: Vec3 = Vec3::new(0., 0., 0.);
                for _ in 0..self.sample_per_px {
                    let ray = self.get_ray(i, j);
                    px_colour += self.ray_colour(ray, world);
                }
                colour::write_colour(self.px_sample_scale * px_colour);
            }
        }

        info!("\rDone. \n")
        // endregion
    }
}