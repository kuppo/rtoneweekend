use image::EncodableLayout;

fn main() {
    let width = 800;
    let height = 600;

    let mut cache: Vec<u8> = vec![];
    cache.reserve(width * height * 3);

    for _ in 0..width {
        for _ in 0..height {
            cache.push((127 as f64).sqrt() as u8);
            cache.push((127 as f64).sqrt() as u8);
            cache.push((127 as f64).sqrt() as u8);
        }
    }

    image::save_buffer(
        "/tmp/test2.png",
        cache.as_bytes(),
        width as u32,
        height as u32,
        image::ColorType::Rgb8,
    )
    .unwrap();
}
