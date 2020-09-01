use std::cmp::Ordering;

use common::vec2::Vec2i;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Direction {
    q: i32,
    n: i32,
    d: i32
}

fn norm(v: i32) -> i32 {
    if v == 0 {
        0
    } else {
        v / v.abs()
    }
}

impl Direction {
    pub fn from(v: Vec2i) -> Self {
        // Quadrant
        let xq = norm(v.x) * -1;
        let yq = norm(v.y) * xq;

        let quad = std::cmp::max(0, xq)*2 + std::cmp::max(0, yq);

        return Self {
            q: quad,
            n: v.y,
            d: v.x
        }
    }

    pub fn to_vec(&self) -> Vec2i {
        Vec2i::new(self.d, self.n)
    }
}

impl Ord for Direction {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else if self.q < other.q {
            Ordering::Less
        } else if self.q > other.q {
            Ordering::Greater
        } else {
            (other.n * self.d).cmp(&(self.n * other.d))
        }
    }
}

impl PartialOrd for Direction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quad() {
        let v = Direction::from(Vec2i::new(1, 1));
        assert_eq!(v.q, 0);
        let v = Direction::from(Vec2i::new(1, -8));
        assert_eq!(v.q, 1);
        let v = Direction::from(Vec2i::new(-5, -2));
        assert_eq!(v.q, 2);
        let v = Direction::from(Vec2i::new(-1, 1));
        assert_eq!(v.q, 3);
    }

    #[test]
    fn ordering() {
        let a = Direction::from(Vec2i::new(1, 1));
        let b = Direction::from(Vec2i::new(2, 1));
        let c = Direction::from(Vec2i::new(2, -1));
        let d = Direction::from(Vec2i::new(1, -1));
        let e = Direction::from(Vec2i::new(-2, -5));
        let f = Direction::from(Vec2i::new(-3, 2));

        assert!(a < b);
        assert!(a < c);
        assert!(a < d);
        assert!(b < c);
        
        let mut l = vec![c.clone(), e.clone(), a.clone(), b.clone(), f.clone(), d.clone()];
        l.sort();

        assert_eq!(l, vec![a, b, c, d, e, f]);
    }
}