use crate::{
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct HitRecord {
    pub intersection: Point,
    pub t: f64,
    pub normal: Vec3, // unit vector
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}
