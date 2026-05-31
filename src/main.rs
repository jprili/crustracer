use std::rc::Rc;

use crustracer::constants::*;
use crustracer::hittable_list::HittableList;
use crustracer::material::{ Dielectric, Lambertian, Material, Metal };
use crustracer::sphere::Sphere;
use crustracer::camera::Camera;


fn main() {
    let mut world: HittableList = HittableList::new();

    let ground_mat: Rc<dyn Material> = Rc::new(
        Lambertian::new(
            Vec3::new(0.5, 0.5, 0.5)
        )
    );
    world.add(Box::new(
        Sphere::new(
            Vec3::new(0., -1000., 0.),
            1000.,
            ground_mat
        )
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_unit();
            let c = Vec3::new(
                a as f64 + 0.9 * rand_unit(),
                0.2,
                b as f64 + 0.9 * rand_unit()
            );

            if (c - Vec3::new(4., 0.2, 0.))
                .mag() > 0.9 
            {
                if choose_mat < 0.8 {
                    let albedo = 
                        Vec3::rand_vec3() * Vec3::rand_vec3();
                    world.add(
                        Box::new(
                            Sphere::new(
                                c,
                                0.2,
                                Rc::new(Lambertian::new(
                                    albedo
                                ))
                            )
                        )
                    )
                } else if choose_mat < 0.95 {
                    let albedo = 
                        Vec3::rand_vec3_range(0.5, 1.);
                    let fuzz = rand_range(0., 0.5);
                    world.add(
                        Box::new(
                            Sphere::new(
                                c,
                                0.2,
                                Rc::new(Metal::new(
                                    albedo, fuzz
                                ))
                            )
                        )
                    )
                } else {
                    world.add(
                        Box::new(
                            Sphere::new(
                                c,
                                0.2,
                                Rc::new(
                                    Dielectric::new(1.5)
                                )
                            )
                        )
                    )
                }
            }
        }
    }

    world.add(
        Box::new(
            Sphere::new(
                Vec3::new(0., 1., 0.),
                1.,
                Rc::new(
                    Dielectric::new(1.5)
                )
            )
        )
    );

    world.add(
        Box::new(
            Sphere::new(
                Vec3::new(-4.0, 1., 0.),
                1.,
                Rc::new(
                    Lambertian::new(
                        Vec3::new(0.4, 0.2, 0.1)
                    )
                )
            )
        )
    );

    world.add(
        Box::new(
            Sphere::new(
                Vec3::new(4.0, 1., 0.),
                1.,
                Rc::new(
                    Metal::new(
                        Vec3::new(0.7, 0.6, 0.5),
                        0.0
                    )
                )
            )
        )
    );


    let camera: Camera = Camera::new(
        16. / 9.,
        1200,
        500,
        50,
        20.,
        Vec3::new(13., 2., 3.),
        Vec3::new(0., 0., 0.),
        Vec3::new(0., 1., 0.),
        10.,
        0.6
    );

    camera.render(&world);
}
