use rtoneweekend::{
    camera::{Camera, CameraConfig},
    hittable::Hittable,
    material::{Dieletric, Lambertian, Metal, Rgb},
    shape::Sphere,
    vec3::{Point, Vec3},
};
use std::rc::Rc;

fn main() {
    let mut random_generator = rand::thread_rng();
    let mut camera = Camera::create(CameraConfig {
        width: 800,
        max_depth: 10,
        samples_per_pixel: 20,
        vfov: 20.0,
        look_from: Point::new(-2.0, 2.0, 1.0),
        look_at: Point::new(0.0, 0.0, -1.0),
        camera_vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 10.0,
        focus_dist: 3.4,
        ..Default::default()
    });

    // world
    let world: Vec<Rc<dyn Hittable>> = vec![
        Rc::new(Sphere {
            center: Point::new(0.0, -100.5, -1.0),
            radius: 100.0,
            material: Rc::new(Lambertian {
                albedo: Rgb::new(0.8, 0.8, 0.0),
            }),
        }),
        Rc::new(Sphere {
            center: Point::new(0.0, 0.0, -1.0),
            radius: 0.5,
            material: Rc::new(Lambertian {
                albedo: Rgb::new(0.1, 0.2, 0.5),
            }),
        }),
        Rc::new(Sphere {
            center: Point::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: Rc::new(Dieletric { ir: 1.5 }),
        }),
        Rc::new(Sphere {
            center: Point::new(-1.0, 0.0, -1.0),
            radius: -0.4,
            material: Rc::new(Dieletric { ir: 1.5 }),
        }),
        Rc::new(Sphere {
            center: Point::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: Rc::new(Metal {
                albedo: Rgb::new(0.8, 0.6, 0.2),
                fuzz: 0.0,
            }),
        }),
    ];

    camera.render(&world, &mut random_generator);
}
