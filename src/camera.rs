use std::io;
use std::io::Write;

use log::info;

use crate::hittable::{ Hittable, HitRecord };
use crate::colour;
use crate::constants::*;

pub struct Camera {
    pub aspect_ratio: f64,
    pub img_w:        i32, // px
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
            return 0.5 * (rec.norm + Vec3::new(1., 1., 1.))
        }

    let a = (Vec3::unit_vec(&ray.dir).y() + 1.) * 0.5;
    (Vec3::new(1., 1., 1.) * (1. - a))
        + (Vec3::new(0.5, 0.7, 1.) * a)
    }

    pub fn new(asp: f64, img_w: i32) -> Self {
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
            origin: v_ul + (0.5 * (px_du + px_dv))
        }
    }

    pub fn render<T: Hittable>(&self, world: &T) {
        // region RENDER
        println!("P3\n{} {}\n255", self.img_w, self.img_h);

        for j in 0..self.img_h {
            info!("\rScanlines remaining: {}", self.img_h - j);
            match io::stderr().flush() { _ => () };
            for i in 0..self.img_w { 
                let px_to_write: Vec3 = 
                    self.origin
                    + (self.px_du * (i as f64)) 
                    + (self.px_dv * (j as f64));

                let ray_dir = px_to_write - self.centre;

                colour::write_colour(
                    self.ray_colour(
                        Ray::new( self.centre, ray_dir),
                        world
                    )
                );
            }
        }

        info!("\rDone. \n")
        // endregion
    }
}