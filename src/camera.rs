#![allow(dead_code)]

use crate::{
    hittable::{Hittable, HittableList},
    material::Rgb,
    ray::Ray,
    vec3::{Point, Vec3},
};
use image::EncodableLayout;
use indicatif::ProgressBar;
use rand::{rngs::ThreadRng, Rng};
use std::f64::INFINITY;

pub struct CameraConfig {
    pub aspect_ratio: f64,
    pub width: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    pub float_correction: f64,
    pub vfov: f64, // vertical field of view, angle
    pub look_from: Point,
    pub look_at: Point,
    pub camera_vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
}

impl Default for CameraConfig {
    fn default() -> CameraConfig {
        CameraConfig {
            aspect_ratio: 16.0 / 9.0,
            width: 800,
            samples_per_pixel: 10,
            max_depth: 20,
            float_correction: 0.0001,
            vfov: 90.0,
            look_from: Point::new(0.0, 0.0, 0.0),
            look_at: Point::new(0.0, 0.0, 1.0),
            camera_vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 1.0,
            focus_dist: 10.0,
        }
    }
}

pub struct Camera {
    aspect_ratio: f64,
    width: usize,
    height: usize,
    viewport_height: f64,
    viewport_width: f64,
    viewport_u: Vec3,
    viewport_v: Vec3,
    delta_u: Vec3,
    delta_v: Vec3,
    viewport_upperleft: Point,
    pixel00loc: Point,
    indicator_bar: ProgressBar,
    cache: Vec<u8>,
    samples_per_pixel: usize,
    max_depth: usize,
    float_correction: f64,
    vfov: f64,
    look_from: Point,
    look_at: Point,
    camera_vup: Vec3,
    camera_u: Vec3,
    camera_v: Vec3,
    camera_w: Vec3,
    defocus_angle: f64,
    focus_dist: f64,
    defocus_u: Vec3,
    defocus_v: Vec3,
}

impl Camera {
    pub fn create(config: CameraConfig) -> Camera {
        // image size
        let mut height = (config.width as f64 / config.aspect_ratio) as usize;
        height = if height < 1 { 1 } else { height };

        // View plane size
        let viewport_height = 2.0 * config.focus_dist * (config.vfov / 2.0).to_radians().tan();
        let viewport_width = viewport_height * (config.width as f64 / height as f64);

        // camera position
        let camera_center = config.look_from;

        // camear coordinate system
        let camera_w = (config.look_from - config.look_at).unit_vector(); // point at oppesite to looking direction
        let camera_u = config.camera_vup.cross(camera_w).unit_vector();
        let camera_v = camera_w.cross(camera_u).unit_vector();

        // defocus disk
        let defocus_radius = (config.defocus_angle / 2.0).to_radians().tan() * config.focus_dist;
        let defocus_u = defocus_radius * camera_u;
        let defocus_v = defocus_radius * camera_v;

        // view plan vectors
        let viewport_u = viewport_width * camera_u;
        let viewport_v = viewport_height * -camera_v;

        // delta viewport
        let delta_u = viewport_u / config.width as f64;
        let delta_v = viewport_v / height as f64;

        // upper corner
        let viewport_upperleft =
            camera_center - config.focus_dist * camera_w - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00loc = viewport_upperleft + 0.5 * (delta_u + delta_v);

        // misc
        let indicator_bar = ProgressBar::new(height as u64);
        let mut cache: Vec<u8> = vec![];
        cache.reserve(config.width * height * 3);

        Camera {
            aspect_ratio: config.aspect_ratio,
            width: config.width,
            height,
            viewport_height,
            viewport_width,
            viewport_u,
            viewport_v,
            delta_u,
            delta_v,
            viewport_upperleft,
            pixel00loc,
            indicator_bar,
            cache,
            samples_per_pixel: config.samples_per_pixel,
            max_depth: config.max_depth,
            float_correction: config.float_correction,
            vfov: config.vfov,
            look_from: config.look_from,
            look_at: config.look_at,
            camera_vup: config.camera_vup,
            camera_u,
            camera_v,
            camera_w,
            defocus_angle: config.defocus_angle,
            focus_dist: config.focus_dist,
            defocus_u,
            defocus_v,
        }
    }

    pub fn render(&mut self, world: &HittableList, random_generator: &mut ThreadRng) {
        for i in 0..self.height {
            for j in 0..self.width {
                let pixel_center =
                    self.pixel00loc + j as f64 * self.delta_u + i as f64 * self.delta_v;

                let mut color = Rgb::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    color = color
                        + self.ray_color(
                            &self.get_ray(pixel_center, random_generator),
                            &world,
                            random_generator,
                            self.max_depth,
                        );
                }
                self.write_color(color);
            }
            self.indicator_bar.inc(1);
        }

        self.indicator_bar.finish();

        println!("Saving file...");
        image::save_buffer(
            "/tmp/pic.png",
            self.cache.as_bytes(),
            self.width as u32,
            self.height as u32,
            image::ColorType::Rgb8,
        )
        .unwrap();
        println!("Done");
    }

    fn ray_color(
        &self,
        r: &Ray,
        world: &HittableList,
        random_generator: &mut ThreadRng,
        depth: usize,
    ) -> Rgb {
        if depth == 0 {
            return Rgb::new(0.0, 0.0, 0.0);
        }

        match world.hit(r, self.float_correction, INFINITY) {
            Some(hit_record) => {
                let (ray, color) = hit_record
                    .material
                    .scatter(r, &hit_record, random_generator);
                Rgb::from(self.ray_color(&ray, world, random_generator, depth - 1)) * color
            }
            _ => {
                let unit_dir = r.dir().unit_vector();
                let a = 0.5 * (unit_dir.j + 1.0);
                (1.0 - a) * Rgb::white() + a * Rgb::new(0.5, 0.7, 1.0)
            }
        }
    }

    fn write_color(&mut self, color: Rgb) {
        let color_desaturated = (color / self.samples_per_pixel).to_gamma() * 255.0;
        self.cache.push(color_desaturated.r as u8);
        self.cache.push(color_desaturated.g as u8);
        self.cache.push(color_desaturated.b as u8);
    }

    fn random_sample_square(&self, random_generator: &mut ThreadRng) -> Point {
        random_generator.gen_range(-0.5..0.5) * self.delta_u
            + random_generator.gen_range(-0.5..0.5) * self.delta_v
    }

    fn get_ray(&self, pixel_center: Point, random_generator: &mut ThreadRng) -> Ray {
        let random_point = self.random_sample_square(random_generator) + pixel_center;
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.look_from
        } else {
            self.defocus_disk_sample(random_generator)
        };
        Ray {
            origin: ray_origin,
            direction: random_point - ray_origin,
        }
    }

    fn defocus_disk_sample(&self, random_generator: &mut ThreadRng) -> Point {
        let p = Vec3::random_in_unit_circle(random_generator);
        self.look_from + p.i * self.defocus_u + p.j * self.defocus_v
    }
}
