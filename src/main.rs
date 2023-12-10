use rtoneweekend::{
    camera::{Camera, CameraConfig},
    hittable::Hittable,
    material::{Dieletric, Lambertian, Metal, Rgb},
    shape::Sphere,
    vec3::Point,
};
use std::{
    f64::consts::{FRAC_PI_4, PI},
    rc::Rc,
};

fn main() {
    let mut random_generator = rand::thread_rng();
    let mut camera = Camera::create(CameraConfig {
        width: 800,
        max_depth: 10,
        samples_per_pixel: 10,
        ..Default::default()
    });

    // world
    let world: Vec<Rc<dyn Hittable>> = vec![
        // Rc::new(Sphere {
        //     center: Point::new(0.0, -1000.5, -1.0),
        //     radius: 1000.0,
        //     material: Rc::new(Lambertian {
        //         albedo: Rgb::new(0.8, 0.8, 0.0),
        //     }),
        // }),
        Rc::new(Sphere {
            center: Point::new(-FRAC_PI_4.cos(), 0.0, -1.0),
            radius: FRAC_PI_4.cos(),
            material: Rc::new(Lambertian {
                albedo: Rgb::new(0.0, 0.0, 1.0),
            }),
        }),
        Rc::new(Sphere {
            center: Point::new(FRAC_PI_4.cos(), 0.0, -1.0),
            radius: FRAC_PI_4.cos(),
            material: Rc::new(Lambertian {
                albedo: Rgb::new(1.0, 0.0, 0.0),
            }),
        }),
    ];

    camera.render(&world, &mut random_generator);
}
