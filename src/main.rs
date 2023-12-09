use rtoneweekend::{
    camera::{Camera, CameraConfig},
    hittable::Hittable,
    shape::Sphere,
    vec3::Point,
};
use std::rc::Rc;

fn main() {
    let mut random_generator = rand::thread_rng();
    let mut camera = Camera::create(CameraConfig {
        width: 800,
        max_depth: 10,
        ..Default::default()
    });

    // world
    let world: Vec<Rc<dyn Hittable>> = vec![
        Rc::new(Sphere {
            origin: Point::new(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
        Rc::new(Sphere {
            origin: Point::new(0.0, -1000.5, -1.0),
            radius: 1000.0,
        }),
    ];

    camera.render(&world, &mut random_generator);
}
