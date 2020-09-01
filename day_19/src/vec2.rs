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
