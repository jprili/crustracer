use std::rc::Rc;

use crustracer::constants::*;
use crustracer::hittable_list::HittableList;
use crustracer::material::{ Dielectric, Lambertian, Material, Metal };
use crustracer::sphere::Sphere;
use crustracer::camera::Camera;


fn main() {
    let mut world: HittableList = HittableList::new();

    let m_ground: Rc<dyn Material> = Rc::new(
        Lambertian::new(Vec3::new(0.8, 0.8, 0.))
    );
    let m_centre: Rc<dyn Material> = Rc::new(
        Lambertian::new(
            Vec3::new(0.1, 0.2, 0.5)
        )
    );
    let m_left: Rc<dyn Material> = Rc::new(
        Dielectric::new(
            1.5
        )
    );
    let m_bubble: Rc<dyn Material> = Rc::new(
        Dielectric::new(
            1. / 1.50
        )
    );
    let m_right: Rc<dyn Material> = Rc::new(
        Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.)
    );

    world.add(Box::new(Sphere::new(
        Vec3::new(0.,-100.5, -1.), 100.,
        m_ground
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.,0., -1.2), 0.5,
        m_centre
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.,0., -1.), 0.5,
        m_left
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.,0., -1.), 0.4,
        m_bubble
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.,0., -1.), 0.5,
        m_right
    )));

    let camera: Camera = Camera::new(
        16. / 9., 400, 100
    );
    camera.render(&world);
}
