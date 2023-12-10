use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::{Point, Vec3},
};
use std::rc::Rc;

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let ac = ray.origin() - self.center;
        let a = ray.dir().dot(ray.dir());
        let b = 2.0 * ac.dot(ray.dir());
        let c = ac.dot(ac) - self.radius * self.radius;
        let discriminant: f64 = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let mut root = (-b - discriminant.sqrt()) / (2.0 * a);

        if root <= ray_tmin || root >= ray_tmax {
            root = (-b + discriminant.sqrt()) / (2.0 * a);
            if root <= ray_tmin || root >= ray_tmax {
                return None;
            }
        }
        let intersection = ray.at(root);
        let outside_normal = (intersection - self.center) / self.radius;
        let mut tmp = HitRecord {
            intersection,
            t: root,
            normal: Vec3::new(0.0, 0.0, 0.0),
            out_facing: false,
            material: self.material.clone(),
        };

        tmp.set_outside_normal(ray, outside_normal);

        Some(HitRecord { ..tmp })
    }
}
