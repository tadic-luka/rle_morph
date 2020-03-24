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

    /// Get underlying vec used to store pixels
    pub fn into_raw(self) -> Vec<u8> {
        self.data
    }

    /// Flip bits in image (nonzero to 0, 0 to 1)
    pub fn flip_bits_mut(&mut self) {
        for elem in &mut self.data {
            *elem = if *elem > 0 {
                0
            } else {
                1
            }
        }
    }

    /// Flip bits in image (nonzero to 0, 0 to 1)
    pub fn flip_bits(mut self) -> Self {
        self.flip_bits_mut();
        self
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

impl Index<usize> for Image {
    type Output = [u8];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.w..(index+1)*self.w]
    }
}

impl IndexMut<usize> for Image {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * self.w..(index+1)*self.w]
    }
}
