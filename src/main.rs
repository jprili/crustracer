use log::{ info };
use std::io;
use std::io::Write;

use crustracer::constants::*;
use crustracer::hittable::{ HitRecord, Hittable };
use crustracer::hittable_list::HittableList;
use crustracer::sphere::Sphere;
use crustracer::colour;

fn ray_colour<T: Hittable>(ray: Ray, world: &T) -> Vec3 {
    let rec: &mut HitRecord = &mut HitRecord::new();
    if world.hit(ray, 0., INF, rec) {
        return 0.5 * (rec.norm + Vec3::new(1., 1., 1.))
    }

    let a = (Vec3::unit_vec(&ray.dir).y() + 1.) * 0.5;
    (Vec3::new(1., 1., 1.) * (1. - a))
        + (Vec3::new(0.5, 0.7, 1.) * a)
}

fn main() {
    // image properties
    let aspect_ratio: f64 = 16. / 9.;
    let img_w:  i32 = 400;
    let _aux: i32 = (img_w as f64 / aspect_ratio) as i32;
    let img_h: i32 = if _aux > 1 { _aux } else { 1 };

    // camera properties
    let focal_length = 1.0;
    let viewport_h: f64 = 2.0;
    let viewport_w: f64 = 
        viewport_h  * ((img_w as f64) / (img_h as f64));
    let camera_centre: Vec3 = Vec3::new(0., 0., 0.);

    // viewport coordinates
    let v_u = Vec3::new(viewport_w, 0., 0.);
    let v_v = Vec3::new(0., -viewport_h, 0.);
    let px_du = v_u.clone() / (img_w as f64);
    let px_dv = v_v.clone() / (img_h as f64);

    let v_upper_left = camera_centre 
        - Vec3::new(0., 0., focal_length)
        - (v_u.clone() / 2.) - (v_v.clone() / 2.);

    let v_origin = v_upper_left.clone()
        + (
            (px_du.clone() + px_dv.clone()) * 0.5
        );

    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::new(
        Vec3::new(0.,0., -1.), 0.5
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.,-100.5, -1.), 100.
    )));

    // region RENDER
    println!("P3\n{img_w} {img_h}\n255");

    for j in 0..img_h {
        info!("\rScanlines remaining: {}", img_h - j);
        match io::stderr().flush() { _ => () };
        for i in 0..img_w { 
            let px_to_write: Vec3 = 
                v_origin 
                + (px_du.clone() * (i as f64)) 
                + (px_dv.clone() * (j as f64));

            let ray_dir = px_to_write - camera_centre.clone();

            colour::write_colour(
                ray_colour(
                    Ray::new(camera_centre, ray_dir),
                    &world
                )
            );
        }
    }

    info!("\rDone. \n")
    // endregion
}
