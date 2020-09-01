use std::slice::Iter;

// Double ended vector (one which has positive and negative addressable space)

#[derive(Clone, PartialEq, Debug)]
pub struct Devec<T> {
    neg: Vec<T>,
    pos: Vec<T>
}

impl <T : Clone> Devec<T> {
    pub fn with_size(min: isize, max: isize, val: T) -> Self {
        let mut w = Self::new();
        for _ in 0..(min.abs()) {
            w.push_l(val.clone());
        }

        for _ in 0..=max {
            w.push_r(val.clone());
        }

        w
    }
}

impl <T> Devec<T> {
    pub fn new() -> Self {
        Self {
            neg: Vec::new(),
            pos: Vec::new()
        }
    }

    pub fn max_idx(&self) -> isize {
        self.pos.len() as isize - 1
    }

    pub fn min_idx(&self) -> isize {
        (self.neg.len() as isize) * -1
    }

    pub fn push_r(&mut self, val: T) {
        self.pos.push(val);
    }

    pub fn push_l(&mut self, val: T) {
        self.neg.push(val);
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> {
        self.neg.iter().rev().chain(self.pos.iter())
    }

    pub fn get<'a>(&'a self, i: isize) -> Option<&'a T> {
        if i < 0 {
            let i = (i.abs() - 1) as usize;
            self.neg.get(i)
        } else {
            let i = i as usize;
            self.pos.get(i)
        }
    }

    pub fn get_mut<'a>(&'a mut self, i: isize) -> Option<&'a mut T> {
        if i < 0 {
            let i = (i.abs() - 1) as usize;
            self.neg.get_mut(i)
        } else {
            let i = i as usize;
            self.pos.get_mut(i)
        }
    }
}
