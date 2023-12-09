use crate::{
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct HitRecord {
    pub intersection: Point,
    pub t: f64,
    pub normal: Vec3, // unit vector
    pub out_facing: bool,
}

impl HitRecord {
    // outside_normal should be a unit vector
    pub fn set_outside_normal(&mut self, ray: &Ray, outside_normal: Vec3) {
        if ray.dir().dot(outside_normal) > 0.0 {
            self.out_facing = false;
            self.normal = -outside_normal;
        } else {
            self.out_facing = true;
            self.normal = outside_normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}
