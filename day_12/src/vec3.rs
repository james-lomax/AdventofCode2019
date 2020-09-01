#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd)]
pub struct Vec3(pub i32, pub i32, pub i32);

pub fn norm(v: i32) -> i32 {
    if v == 0 {
        0
    } else {
        v / v.abs()
    }
}

impl Vec3 {
    pub fn from(v: Vec<i32>) -> Self {
        Self(v[0], v[1], v[2])
    }

    pub fn add(&self, other: &Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }

    pub fn mul(&self, v: i32) -> Self {
        Self(self.0 * v, self.1 * v, self.2 * v)
    }

    pub fn div(&self, v: i32) -> Self {
        Self(self.0 / v, self.1 / v, self.2 / v)
    }

    pub fn manhatten(&self) -> i32 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }

    pub fn axis_norm(&self) -> Self {
        Self(norm(self.0), norm(self.1), norm(self.2))
    }
}