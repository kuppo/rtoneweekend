use std::ops::{Add, Div, Mul, Sub};

use rand::rngs::ThreadRng;

use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Rgb {
    pub fn new(r: f64, g: f64, b: f64) -> Rgb {
        Rgb { r, g, b }
    }

    pub fn white() -> Rgb {
        Rgb {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    pub fn to_gamma(self) -> Rgb {
        Rgb {
            r: self.r.sqrt(),
            g: self.g.sqrt(),
            b: self.b.sqrt(),
        }
    }
}

impl Mul<f64> for Rgb {
    type Output = Self;

    fn mul(self, scaler: f64) -> Self::Output {
        Rgb {
            r: self.r * scaler,
            g: self.g * scaler,
            b: self.b * scaler,
        }
    }
}

impl Mul<Rgb> for f64 {
    type Output = Rgb;

    fn mul(self, rhs: Rgb) -> Self::Output {
        Rgb {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

impl Mul for Rgb {
    type Output = Rgb;

    fn mul(self, rhs: Rgb) -> Self::Output {
        Rgb {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Add for Rgb {
    type Output = Rgb;

    fn add(self, rhs: Rgb) -> Self::Output {
        Rgb {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sub for Rgb {
    type Output = Rgb;

    fn sub(self, rhs: Rgb) -> Self::Output {
        Rgb {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl Div<usize> for Rgb {
    type Output = Rgb;

    fn div(self, rhs: usize) -> Self::Output {
        Rgb {
            r: self.r / (rhs as f64),
            g: self.g / (rhs as f64),
            b: self.b / (rhs as f64),
        }
    }
}

/// The trait represents the material of the shape. It will return the scattered ray and its color.
pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        random_generator: &mut ThreadRng,
    ) -> (Ray, Rgb);
}

pub struct Lambertian {
    pub albedo: Rgb,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _: &Ray,
        hit_record: &HitRecord,
        random_generator: &mut ThreadRng,
    ) -> (Ray, Rgb) {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector(random_generator);
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        (
            Ray {
                origin: hit_record.intersection,
                direction: scatter_direction,
            },
            self.albedo,
        )
    }
}

pub struct Metal {
    pub albedo: Rgb,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        random_generator: &mut ThreadRng,
    ) -> (Ray, Rgb) {
        let reflect = ray_in.dir().unit_vector().reflect(hit_record.normal);
        let fuzz_factor = self.fuzz.clamp(0.0, 1.0);

        (
            Ray {
                origin: hit_record.intersection,
                direction: reflect + fuzz_factor * Vec3::random_unit_vector(random_generator),
            },
            self.albedo,
        )
    }
}

pub struct Dieletric {
    pub ir: f64, // index of refraction
}

impl Material for Dieletric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, _: &mut ThreadRng) -> (Ray, Rgb) {
        let ir = if hit_record.out_facing {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_dir = ray_in.dir().unit_vector();
        let cos_theta = -unit_dir.dot(hit_record.normal);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let bouncing_vec = match (ir * sin_theta) > 1.0 {
            true => unit_dir.reflect(hit_record.normal),
            false => unit_dir.refract(hit_record.normal, ir, cos_theta),
        };

        (
            Ray {
                origin: hit_record.intersection,
                direction: bouncing_vec,
            },
            Rgb::white(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white() {
        let rgb = Rgb::white();

        assert!(rgb.r == 1.0 && rgb.g == 1.0 && rgb.b == 1.0);
    }

    #[test]
    fn test_mul_rgb() {
        let c1 = Rgb::new(1.0, 2.0, 5.0);
        let c2 = Rgb::new(4.0, 4.0, 4.0) * c1;

        assert!(c2.r == 4.0 && c2.g == 8.0 && c2.b == 20.0);
    }

    #[test]
    fn test_right_mul() {
        let c1 = Rgb::white() * 0.2;

        assert!(c1.r == 0.2 && c1.g == 0.2 && c1.b == 0.2);
    }

    #[test]
    fn test_left_mul() {
        let c2 = 0.2 * Rgb::white();

        assert!(c2.r == 0.2 && c2.g == 0.2 && c2.b == 0.2);
    }

    #[test]
    fn test_div_usize() {
        let c2 = Rgb::new(4.0, 4.0, 4.0) / 2;

        assert!(c2.r == 2.0 && c2.g == 2.0 && c2.b == 2.0);
    }

    #[test]
    fn test_add() {
        let c1 = Rgb::new(1.0, 2.0, 5.0);
        let c2 = Rgb::white();
        let c3 = c1 + c2;

        assert!(c3.r == 2.0 && c3.g == 3.0 && c3.b == 6.0);
    }

    #[test]
    fn test_sub() {
        let c1 = Rgb::new(1.0, 2.0, 5.0);
        let c2 = Rgb::white();
        let c3 = c1 - c2;

        assert!(c3.r == 0.0 && c3.g == 1.0 && c3.b == 4.0);
    }

    #[test]
    fn test_to_gamma() {
        let c1 = Rgb::new(4.0, 4.0, 4.0).to_gamma();

        assert!(c1.r == 2.0 && c1.g == 2.0 && c1.b == 2.0);
    }
}
