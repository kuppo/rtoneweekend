use std::ops::Mul;

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
}
