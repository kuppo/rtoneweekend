#![allow(dead_code)]
use std::ops::{Add, Div, Mul, Neg, Sub};

use rand::{rngs::ThreadRng, Rng};

use crate::material::Rgb;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub i: f64,
    pub j: f64,
    pub k: f64,
}

impl Vec3 {
    pub fn new(i: f64, j: f64, k: f64) -> Vec3 {
        Vec3 { i, j, k }
    }

    pub fn random_unit_vector(random_generator: &mut ThreadRng) -> Vec3 {
        loop {
            let p = Vec3 {
                i: random_generator.gen_range(-1.0..1.0),
                j: random_generator.gen_range(-1.0..1.0),
                k: random_generator.gen_range(-1.0..1.0),
            };
            if p.length() <= 1.0 {
                return p.unit_vector();
            }
        }
    }

    pub fn random_unit_on_hemisphere(random_generator: &mut ThreadRng, normal: Vec3) -> Vec3 {
        let v = Vec3::random_unit_vector(random_generator);
        if v.dot(normal) > 0.0 {
            return v;
        } else {
            return -v;
        }
    }

    pub fn dot(&self, rhs: Vec3) -> f64 {
        self.i * rhs.i + self.j * rhs.j + self.k * rhs.k
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3 {
            i: self.j * rhs.k - self.k * rhs.j,
            j: self.k * rhs.i - self.i * rhs.k,
            k: self.i * rhs.j - self.j * rhs.i,
        }
    }

    pub fn length_pow2(&self) -> f64 {
        self.i * self.i + self.j * self.j + self.k * self.k
    }

    pub fn length(&self) -> f64 {
        (self.i * self.i + self.j * self.j + self.k * self.k).sqrt()
    }

    pub fn unit_vector(self) -> Vec3 {
        let length = self.length();

        Vec3 {
            i: self.i / length,
            j: self.j / length,
            k: self.k / length,
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.i < s && self.j < s && self.k < s
    }

    pub fn reflect(self, normal: Vec3) -> Vec3 {
        self - 2.0 * self.dot(normal) * normal
    }

    pub fn refract(self, normal: Vec3, etai_over_etat: f64, cos_theta: f64) -> Vec3 {
        let v_out_perp = etai_over_etat * (self + cos_theta * normal);
        let v_out_parp = -(1.0 - v_out_perp.length_pow2()).sqrt() * normal;
        v_out_parp + v_out_perp
    }
}

pub type Point = Vec3;

impl Default for Point {
    fn default() -> Point {
        Point {
            i: 0.0,
            j: 0.0,
            k: 0.0,
        }
    }
}

impl Into<Rgb> for Vec3 {
    fn into(self) -> Rgb {
        Rgb {
            r: self.i,
            g: self.j,
            b: self.k,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scaler: f64) -> Self::Output {
        Vec3 {
            i: self.i * scaler,
            j: self.j * scaler,
            k: self.k * scaler,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            i: self * rhs.i,
            j: self * rhs.j,
            k: self * rhs.k,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
            k: self.k + rhs.k,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            i: self.i - rhs.i,
            j: self.j - rhs.j,
            k: self.k - rhs.k,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scaler: f64) -> Self::Output {
        Vec3 {
            i: self.i / scaler,
            j: self.j / scaler,
            k: self.k / scaler,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            i: -self.i,
            j: -self.j,
            k: -self.k,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 2.0, 1.0);
        let v3 = v1 + v2;

        assert!(v3.i == 4.0 && v3.j == 4.0 && v3.k == 4.0);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 2.0, 1.0);
        let v3 = v1 - v2;

        assert!(v3.i == -2.0 && v3.j == 0.0 && v3.k == 2.0);
    }

    #[test]
    fn test_mul_f64() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = v1 * 2.0;

        assert!(v2.i == 2.0 && v2.j == 4.0 && v2.k == 6.0);
    }

    #[test]
    fn test_f64_mul_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = 2.0 * v1;

        assert!(v2.i == 2.0 && v2.j == 4.0 && v2.k == 6.0);
    }

    #[test]
    fn test_div() {
        let v1 = Vec3::new(2.0, 4.0, 6.0);
        let v2 = v1 / 2.0;

        assert!(v2.i == 1.0 && v2.j == 2.0 && v2.k == 3.0);
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(2.0, 4.0, 6.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);

        assert!(v1.dot(v2) == 28.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 5.0, 7.0);
        let v3 = v1.cross(v2);

        assert!(v3.i == -1.0 && v3.j == -4.0 && v3.k == 3.0);
    }

    #[test]
    fn test_length_pow2() {
        let v1 = Vec3::new(1.0, 2.0, 2.0);

        assert!(v1.length_pow2() == 9.0);
    }

    #[test]
    fn test_length() {
        let v1 = Vec3::new(2.0, 3.0, 6.0);

        assert!(v1.length() == 7.0);
    }

    #[test]
    fn test_unit_vector() {
        let v1 = Vec3::new(4.0, 4.0, 7.0);
        let v2 = v1.unit_vector();

        assert!(v2.i == 4.0 / 9.0 && v2.j == 4.0 / 9.0 && v2.k == 7.0 / 9.0);
    }

    #[test]
    fn test_into() {
        let v1 = Vec3::new(4.0, 4.0, 7.0);
        let rgb: Rgb = v1.into();

        assert!(rgb.r == 4.0 && rgb.g == 4.0 && rgb.b == 7.0);
    }
}
