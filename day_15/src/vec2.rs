#[allow(unused)]
//use num::Integer;
#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    pub fn manhatten(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn length(&self) -> i32 {
        self.manhatten()
    }

    pub fn add(&self, other: &Vec2) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    pub fn sub(&self, other: &Vec2) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }

    pub fn dot(&self, other: &Self) -> i32 {
        self.x * other.x + self.y * other.y
    }

    pub fn elem_mul(&self, other: &Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }

    pub fn normalized(&self) -> Self {
        let l = self.length();
        Self {
            x: self.x / l,
            y: self.y / l,
        }
    }

    // pub fn gcd_normalized(&self) -> Self {
    //     let gcd = self.x.gcd(&self.y);
    //     Self {
    //         x: self.x / gcd,
    //         y: self.y / gcd
    //     }
    // }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /**
     * Returns anticlockwise angle between `from` -> `self`
     */
    pub fn angle(&self, from: &Vec2) -> f64 {
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

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn gcd_normalized() {
    //     assert_eq!(Vec2::new(2, 8).gcd_normalized(), Vec2::new(1, 4));
    //     assert_eq!(Vec2::new(0, 5).gcd_normalized(), Vec2::new(0, 1));
    // }

    #[test]
    fn angle() {
        let up = Vec2::new(0, -1);

        let a = Vec2::new(-1, -1).angle(&up);
        println!("a={}", a);
        assert!((a - std::f64::consts::FRAC_PI_4 * 7.0) < 1e-10);

        let up = Vec2::new(0, 1);

        println!("d={}", Vec2::new(1, 1).angle(&up));
        println!("d={}", Vec2::new(1, -1).angle(&up));
        println!("d={}", Vec2::new(-1, -1).angle(&up));
        println!("d={}", Vec2::new(-1, 1).angle(&up));

        let diff = Vec2::new(0, 1).angle(&up);
        assert!(diff.abs() < 1e-10);
        let diff = Vec2::new(1, 1).angle(&up) - std::f64::consts::FRAC_PI_4 * 7.0;
        assert!(diff.abs() < 1e-10);

        let diff = Vec2::new(1, 0).angle(&up) - std::f64::consts::FRAC_PI_2 * 3.0;
        assert!(diff.abs() < 1e-10);
        let diff = Vec2::new(1, -1).angle(&up) - (std::f64::consts::FRAC_PI_4 * 5.0);
        assert!(diff.abs() < 1e-10);

        let diff = Vec2::new(0, -1).angle(&up) - std::f64::consts::PI;
        assert!(diff.abs() < 1e-10);
        let diff = Vec2::new(-1, -1).angle(&up) - (std::f64::consts::FRAC_PI_4 * 3.0);
        assert!(diff.abs() < 1e-10);

        let diff = Vec2::new(-1, 0).angle(&up) - std::f64::consts::FRAC_PI_2;
        assert!(diff.abs() < 1e-10);
        let diff = Vec2::new(-1, 1).angle(&up) - (std::f64::consts::FRAC_PI_4);
        assert!(diff.abs() < 1e-10);
    }
}
