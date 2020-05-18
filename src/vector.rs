#![warn(clippy::all)]
#![allow(dead_code)]

#[derive(Debug, PartialEq)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    pub fn normalize(&mut self) {
        let length = ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
        if length > 0.0 {
            self.x /= length;
            self.y /= length;
            self.z /= length;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_negate() {
        let mut v = Vector::new(1.0, -2.0, 3.0);
        v.negate();
        assert_abs_diff_eq!(v.x, -1.0);
        assert_abs_diff_eq!(v.y, 2.0);
        assert_abs_diff_eq!(v.z, -3.0);
    }

    #[test]
    fn should_normalize() {
        let mut v = Vector::new(1.0, 2.0, 3.0);
        let length = (14.0f64).sqrt();
        v.normalize();
        assert_abs_diff_eq!(v.x, 1.0 / length);
        assert_abs_diff_eq!(v.y, 2.0 / length);
        assert_abs_diff_eq!(v.z, 3.0 / length);
    }
}
