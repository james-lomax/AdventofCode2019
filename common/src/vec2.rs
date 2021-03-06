use num::Integer;
use std::convert::{TryFrom, TryInto};

#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd)]
pub struct Vec2<T : Integer> {
    pub x: T,
    pub y: T,
}

pub type Vec2i = Vec2<i32>;
pub type Vec2u = Vec2<usize>;

impl <T> Vec2<T> 
    where T : Integer + Copy
{
    pub fn new(x: T, y: T) -> Self {
        Self { x: x, y: y }
    }

    pub fn sqLength(&self) -> T {
        self.x*self.x + self.y*self.y
    }

    pub fn add(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }

    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn mul(&self, scalar: T) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar
        }
    }

    pub fn elem_mul(&self, other: &Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }

    pub fn gcd_normalized(&self) -> Self {
        let gcd = self.x.gcd(&self.y);
        Self {
            x: self.x / gcd,
            y: self.y / gcd
        }
    }
}

impl Vec2<i32> {
    pub fn manhatten(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /**
     * Returns anticlockwise angle between `from` -> `self`
     */
    pub fn angle(&self, from: &Vec2<i32>) -> f64 {
        let x1 = from.x as f64;
        let y1 = from.y as f64;
        let x2 = self.x as f64;
        let y2 = self.y as f64;
        let dot = x1 * x2 + y1 * y2;
        let det = x1 * y2 - y1 * x2;
        let angle = det.atan2(dot);
        if angle < 0.0 {
            return (std::f64::consts::PI * 2.0) + angle;
        } else {
            return angle;
        }
    }
}

impl TryFrom<Vec2i> for Vec2u {
    type Error = <usize as TryFrom<i32>>::Error;

    fn try_from(value: Vec2i) -> Result<Self, Self::Error> {
        Ok(Self::new(value.x.try_into()?, value.y.try_into()?))
    }
}

impl TryFrom<Vec2u> for Vec2i {
    type Error = <i32 as TryFrom<usize>>::Error;

    fn try_from(value: Vec2u) -> Result<Self, Self::Error> {
        Ok(Self::new(value.x.try_into()?, value.y.try_into()?))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gcd_normalized() {
        assert_eq!(Vec2i::new(2, 8).gcd_normalized(), Vec2i::new(1, 4));
        assert_eq!(Vec2i::new(0, 5).gcd_normalized(), Vec2i::new(0, 1));
    }

    #[test]
    fn angle() {
        let up = Vec2i::new(0, -1);

        let a = Vec2i::new(-1, -1).angle(&up);
        println!("a={}", a);
        assert!((a - std::f64::consts::FRAC_PI_4 * 7.0) < 1e-10);

        let up = Vec2i::new(0, 1);

        println!("d={}", Vec2i::new(1, 1).angle(&up));
        println!("d={}", Vec2i::new(1, -1).angle(&up));
        println!("d={}", Vec2i::new(-1, -1).angle(&up));
        println!("d={}", Vec2i::new(-1, 1).angle(&up));

        let diff = Vec2i::new(0, 1).angle(&up);
        assert!(diff.abs() < 1e-10);
        let diff = Vec2i::new(1, 1).angle(&up) - std::f64::consts::FRAC_PI_4 * 7.0;
        assert!(diff.abs() < 1e-10);

        let diff = Vec2i::new(1, 0).angle(&up) - std::f64::consts::FRAC_PI_2 * 3.0;
        assert!(diff.abs() < 1e-10);
        let diff = Vec2i::new(1, -1).angle(&up) - (std::f64::consts::FRAC_PI_4 * 5.0);
        assert!(diff.abs() < 1e-10);

        let diff = Vec2i::new(0, -1).angle(&up) - std::f64::consts::PI;
        assert!(diff.abs() < 1e-10);
        let diff = Vec2i::new(-1, -1).angle(&up) - (std::f64::consts::FRAC_PI_4 * 3.0);
        assert!(diff.abs() < 1e-10);

        let diff = Vec2i::new(-1, 0).angle(&up) - std::f64::consts::FRAC_PI_2;
        assert!(diff.abs() < 1e-10);
        let diff = Vec2i::new(-1, 1).angle(&up) - (std::f64::consts::FRAC_PI_4);
        assert!(diff.abs() < 1e-10);
    }
}
