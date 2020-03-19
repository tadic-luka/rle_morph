use std::ops::{Index, IndexMut};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    w: usize,
    h: usize,
    data: Vec<u8>,
}

impl Image {
    pub fn new(w: usize, h: usize, data: Vec<u8>) -> Self {
        Self { w, h, data }
    }

    pub fn empty(w: usize, h: usize) -> Self {
        Self {
            w,
            h,
            data: vec![0; w * h],
        }
    }

    pub fn w(&self) -> usize {
        self.w
    }

    pub fn h(&self) -> usize {
        self.h
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.h {
            write!(f, "{:?}\n", &self.data[i * self.w..(i + 1) * self.w])?;
        }
        Ok(())
    }
}

