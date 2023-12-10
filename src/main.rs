use rand::Rng;
use rtoneweekend::{
    camera::{Camera, CameraConfig},
    hittable::HittableList,
    material::{Dieletric, Lambertian, Metal, Rgb},
    shape::Sphere,
    vec3::{Point, Vec3},
};
use std::rc::Rc;

fn main() {
    let mut camera: Camera = Camera::create(CameraConfig {
        width: 2560,
        vfov: 20.0,
        max_depth: 20,
        samples_per_pixel: 100,
        look_from: Point::new(13.0, 2.0, 3.0),
        look_at: Point::new(0.0, 0.0, 0.0),
        camera_vup: Vec3::new(0.0, 1.0, 0.0),
        float_correction: 0.00001,
        defocus_angle: 0.6,
        focus_dist: 10.0,
        save_path: "/tmp/pic.png",
        aspect_ratio: 16.0 / 9.0,
        ..Default::default()
    });
    // dbg!(&camera);

    let mut random_generator = rand::thread_rng();

    // world
    let mut world: HittableList = HittableList::new();
    let ground_material = Rc::new(Lambertian {
        albedo: Rgb::new(0.5, 0.5, 0.5),
    });
    let ground = Rc::new(Sphere {
        center: Point::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material.clone(),
    });
    world.push(ground.clone());

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_generator.gen_range(0.0..1.0);
            let center = Point::new(
                a as f64 + 0.9 * random_generator.gen_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * random_generator.gen_range(0.0..1.0),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                match choose_mat {
                    a if a < 0.8 => {
                        let albedo = Rgb::new(
                            random_generator.gen_range(0.0..1.0),
                            random_generator.gen_range(0.0..1.0),
                            random_generator.gen_range(0.0..1.0),
                        ) * Rgb::new(
                            random_generator.gen_range(0.0..1.0),
                            random_generator.gen_range(0.0..1.0),
                            random_generator.gen_range(0.0..1.0),
                        );
                        let sphere_material = Rc::new(Lambertian { albedo });
                        let rand_sphere = Rc::new(Sphere {
                            center,
                            radius: 0.2,
                            material: sphere_material.clone(),
                        });
                        world.push(rand_sphere.clone());
                    }
                    b if b < 0.95 => {
                        let albedo = Rgb::new(
                            random_generator.gen_range(0.5..1.0),
                            random_generator.gen_range(0.5..1.0),
                            random_generator.gen_range(0.5..1.0),
                        );
                        let fuzz = random_generator.gen_range(0.0..0.5);
                        let sphere_material = Rc::new(Metal { albedo, fuzz });
                        let rand_sphere = Rc::new(Sphere {
                            center,
                            radius: 0.2,
                            material: sphere_material.clone(),
                        });
                        world.push(rand_sphere.clone());
                    }
                    _ => {
                        let sphere_meterial = Rc::new(Dieletric { ir: 1.5 });
                        let rand_sphere = Rc::new(Sphere {
                            center,
                            radius: 0.2,
                            material: sphere_meterial.clone(),
                        });
                        world.push(rand_sphere.clone());
                    }
                }
            }
        }
    }

    let material1 = Rc::new(Dieletric { ir: 1.5 });
    world.push(Rc::new(Sphere {
        center: Point::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1.clone(),
    }));
    let material2 = Rc::new(Lambertian {
        albedo: Rgb::new(0.4, 0.2, 0.1),
    });
    world.push(Rc::new(Sphere {
        center: Point::new(-5.0, 1.0, 0.0),
        radius: 1.0,
        material: material2.clone(),
    }));
    let material3 = Rc::new(Metal {
        albedo: Rgb::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.push(Rc::new(Sphere {
        center: Point::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3.clone(),
    }));

    camera.render(&world, &mut random_generator);
}
