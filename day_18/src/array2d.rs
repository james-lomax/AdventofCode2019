use crate::vec2::{Vec2u, Vec2i};

use std::convert::TryFrom;

#[derive(Clone)]
pub struct Array2D<T> {
    pub len_x: usize,
    pub len_y: usize,
    pub data: Vec<Vec<T>>
}

impl <T : std::fmt::Display> std::fmt::Display for Array2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for v in &self.data {
            for c in v {
                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl <T : Clone> Array2D<T> {
    pub fn with_shape(len_x: usize, len_y: usize, init_val: &T) -> Self {
        let mut v = Vec::new();
        for y in 0..len_y {
            let mut row = Vec::new();
            for x in 0..len_x {
                row.push(init_val.clone());
            }
            v.push(row);
        }

        return Self {
            len_x: len_x,
            len_y: len_y,
            data: v
        };
    }
}

impl <T> Array2D<T> {
    pub fn from(data: Vec<Vec<T>>) -> Self {
        if data.len() == 0 {
            return Self {
                len_x: 0,
                len_y: 0,
                data: data
            };
        }

        let len_x = data[0].len();
        for d in &data {
            assert_eq!(len_x, d.len());
        }
        Self {
            len_x: len_x,
            len_y: data.len(),
            data: data
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.len_x && y < self.len_y {
            Some(&self.data[y][x])
        } else {
            None
        }
    }

    pub fn getv(&self, p: &Vec2u) -> Option<&T> {
        return self.get(p.x, p.y);
    }

    pub fn getvi(&self, p: &Vec2i) -> Option<&T> {
        if let Ok(p) = Vec2u::try_from(p.clone()) {
            self.getv(&p)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.len_x && y < self.len_y {
            Some(&mut self.data[y][x])
        } else {
            None
        }
    }

    pub fn getv_mut(&mut self, p: &Vec2u) -> Option<&mut T> {

        return self.get_mut(p.x, p.y);
    }

    pub fn getvi_mut(&mut self, p: &Vec2i) -> Option<&mut T> {
        if let Ok(p) = Vec2u::try_from(p.clone()) {
            self.getv_mut(&p)
        } else {
            None
        }
    }

    pub fn position<F : Fn(&T) -> bool>(&self, f: F) -> Option<(usize, usize)> {
        for y in 0..self.len_y {
            for x in 0..self.len_x {
                if f(&self.data[y][x]) {
                    return Some((x, y));
                }
            }
        }

        return None;
    }
}
