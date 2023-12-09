use std::ops::{Add, Mul, Sub};

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

/// The trait represents the material of the shape. It will return the scattered ray and its color.
pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: HitRecord,
        random_generator: &mut ThreadRng,
    ) -> (Ray, Rgb);
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
