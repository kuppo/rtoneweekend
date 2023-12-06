use image::EncodableLayout;
use indicatif::ProgressBar;
use rtoneweekend::{
    material::Rgb,
    ray::Ray,
    vec3::{Point, Vec3},
};

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    // camera position
    let camera_center = Point::new(0.0, 0.0, 0.0);
    let focal_length = 1.0;

    // image size
    let width = 800;
    let mut height = (width as f64 / ASPECT_RATIO) as usize;
    height = if height < 1 { 1 } else { height };

    // View plane size
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (width as f64 / height as f64);

    // view plan vectors
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // delta viewport
    let delta_u = viewport_u / width as f64;
    let delta_v = viewport_v / height as f64;

    // upper corner
    let viewport_upperleft =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00loc = viewport_upperleft + 0.5 * (delta_u + delta_v);

    // misc
    let bar = ProgressBar::new(height as u64);
    let mut cache: Vec<u8> = vec![];
    cache.reserve(width * height * 3);

    for i in 0..width {
        for j in 0..height {
            let pixel_center = pixel00loc + i as f64 * delta_u + j as f64 * delta_v;
            let ray_direction = pixel_center - camera_center;
            let color = ray_color(&Ray::new(camera_center, ray_direction));
            write_color(&color, &mut cache);
        }
        bar.inc(1);
    }

    bar.finish();

    println!("Saving file...");
    image::save_buffer(
        "/tmp/pic.png",
        cache.as_bytes(),
        width as u32,
        height as u32,
        image::ColorType::Rgb8,
    )
    .unwrap();
    println!("Done");
}

fn ray_color(r: &Ray) -> Rgb {
    Rgb::white()
}

fn write_color(color: &Rgb, cache: &mut Vec<u8>) {
    cache.push((color.r * 255.0) as u8);
    cache.push((color.g * 255.0) as u8);
    cache.push((color.b * 255.0) as u8);
}
