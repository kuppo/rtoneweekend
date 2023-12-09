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
    pub viewport_height: f64,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    pub float_correction: f64,
}

impl Default for CameraConfig {
    fn default() -> CameraConfig {
        CameraConfig {
            aspect_ratio: 16.0 / 9.0,
            width: 800,
            viewport_height: 2.0,
            samples_per_pixel: 10,
            max_depth: 20,
            float_correction: 0.0001,
        }
    }
}

pub struct Camera {
    aspect_ratio: f64,
    width: usize,
    height: usize,
    camera_center: Point,
    focal_length: f64,
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
}

impl Camera {
    pub fn create(config: CameraConfig) -> Camera {
        // camera position
        let camera_center = Point::new(0.0, 0.0, 0.0);
        let focal_length = 1.0;

        // image size
        let mut height = (config.width as f64 / config.aspect_ratio) as usize;
        height = if height < 1 { 1 } else { height };

        // View plane size
        let viewport_width = config.viewport_height * (config.width as f64 / height as f64);

        // view plan vectors
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -config.viewport_height, 0.0);

        // delta viewport
        let delta_u = viewport_u / config.width as f64;
        let delta_v = viewport_v / height as f64;

        // upper corner
        let viewport_upperleft =
            camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00loc = viewport_upperleft + 0.5 * (delta_u + delta_v);

        // misc
        let indicator_bar = ProgressBar::new(height as u64);
        let mut cache: Vec<u8> = vec![];
        cache.reserve(config.width * height * 3);

        Camera {
            aspect_ratio: config.aspect_ratio,
            width: config.width,
            height,
            camera_center,
            focal_length,
            viewport_height: config.viewport_height,
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
                let v = Vec3::random_unit_vector(random_generator) + hit_record.normal;
                0.5 * Rgb::from(self.ray_color(
                    &Ray {
                        origin: hit_record.intersection,
                        direction: v,
                    },
                    world,
                    random_generator,
                    depth - 1,
                ))
            }
            _ => {
                let unit_dir = r.dir().unit_vector();
                let a = 0.5 * (unit_dir.j + 1.0);
                (1.0 - a) * Rgb::white() + a * Rgb::new(0.5, 0.7, 1.0)
            }
        }
    }

    fn write_color(&mut self, color: Rgb) {
        let color_desaturated = ((1.0 / self.samples_per_pixel as f64) * color).to_gamma();
        self.cache.push((color_desaturated.r * 255.0) as u8);
        self.cache.push((color_desaturated.g * 255.0) as u8);
        self.cache.push((color_desaturated.b * 255.0) as u8);
    }

    fn random_sample_square(&self, random_generator: &mut ThreadRng) -> Point {
        random_generator.gen_range(-0.5..0.5) * self.delta_u
            + random_generator.gen_range(-0.5..0.5) * self.delta_v
    }

    fn get_ray(&self, pixel_center: Point, random_generator: &mut ThreadRng) -> Ray {
        let random_point = self.random_sample_square(random_generator) + pixel_center;
        Ray {
            origin: self.camera_center,
            direction: random_point - self.camera_center,
        }
    }
}
