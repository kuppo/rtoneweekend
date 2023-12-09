#![allow(dead_code)]

use crate::{
    hittable::{Hittable, HittableList},
    material::Rgb,
    ray::Ray,
    vec3::{Point, Vec3},
};
use image::EncodableLayout;
use indicatif::ProgressBar;
use std::f64::INFINITY;

pub struct CameraConfig {
    pub aspect_ratio: f64,
    pub width: usize,
    pub viewport_height: f64,
}

impl Default for CameraConfig {
    fn default() -> CameraConfig {
        CameraConfig {
            aspect_ratio: 16.0 / 9.0,
            width: 800,
            viewport_height: 2.0,
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
        }
    }

    pub fn render(&mut self, world: &HittableList) {
        for i in 0..self.height {
            for j in 0..self.width {
                let pixel_center =
                    self.pixel00loc + j as f64 * self.delta_u + i as f64 * self.delta_v;
                let ray_direction = pixel_center - self.camera_center;
                let color = self.ray_color(&Ray::new(self.camera_center, ray_direction), &world);
                self.write_color(&color);
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

    fn ray_color(&self, r: &Ray, world: &HittableList) -> Rgb {
        match world.hit(r, 0.0, INFINITY) {
            Some(hit_record) => (0.5 * (hit_record.normal + Vec3::new(1.0, 1.0, 1.0))).into(),
            _ => {
                let unit_dir = r.dir().unit_vector();
                let a = 0.5 * (unit_dir.j + 1.0);
                (1.0 - a) * Rgb::white() + a * Rgb::new(0.5, 0.7, 1.0)
            }
        }
    }

    fn write_color(&mut self, color: &Rgb) {
        self.cache.push((color.r * 255.0) as u8);
        self.cache.push((color.g * 255.0) as u8);
        self.cache.push((color.b * 255.0) as u8);
    }
}
