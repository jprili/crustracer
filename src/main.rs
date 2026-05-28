use crustracer::constants::*;
use crustracer::hittable_list::HittableList;
use crustracer::sphere::Sphere;
use crustracer::camera::Camera;


fn main() {
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::new(
        Vec3::new(0.,0., -1.), 0.5
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.,-100.5, -1.), 100.
    )));

    let camera: Camera = Camera::new(
        16. / 9., 400, 100
    );
    camera.render(&world);
}
