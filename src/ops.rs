use super::RLE;
use std::ops::{BitAnd, Sub, BitOr, BitOrAssign};

/// Binary or on image pixels, mutable version.
impl<'a, 'b> BitOr<&'a RLE> for &'b RLE {
    type Output = RLE;
    /// If dimensions of self and rhs are not same this method will panic.
    fn bitor(self, rhs: &'a RLE) -> Self::Output {
        assert!(self.width == rhs.width && self.height == rhs.height);
        let mut runs = Vec::with_capacity(self.runs.len() + rhs.runs.len());
        runs.extend(&self.runs);
        runs.extend(&rhs.runs);
        RLE {
            runs,
            width: self.width(),
            height: self.height(),
        }.merge_overlapping_runs()
    }
}

/// Binary or on image pixels, mutable version.
impl BitOr for RLE {
    type Output = RLE;
    /// If dimensions of self and rhs are not same this method will panic.
    fn bitor(self, rhs: RLE) -> Self::Output {
        assert!(self.width == rhs.width && self.height == rhs.height);
        let mut runs = Vec::with_capacity(self.runs.len() + rhs.runs.len());
        runs.extend(&self.runs);
        runs.extend(&rhs.runs);
        RLE {
            runs,
            width: self.width(),
            height: self.height(),
        }.merge_overlapping_runs()
    }
}

/// Binary or on image pixels, mutable version.
impl<'a> BitOrAssign<&'a RLE> for RLE {
    /// If dimensions of self and rhs are not same this method will panic.
    fn bitor_assign(&mut self, rhs: &'a RLE) {
        assert!(self.width == rhs.width && self.height == rhs.height);
        self.runs.extend(&rhs.runs);

        self.merge_overlapping_runs_mut();
    }
}

/// Binary or on image pixels, mutable version.
impl BitOrAssign for RLE {
    /// If dimensions of self and rhs are not same this method will panic.
    fn bitor_assign(&mut self, rhs: RLE) {
        assert!(self.width == rhs.width && self.height == rhs.height);
        self.runs.extend(&rhs.runs);

        self.merge_overlapping_runs_mut();
    }
}

/// Get all 1s in self which are not in other, same as set difference.
impl<'a, 'b> Sub<&'a RLE> for &'b RLE {
    type Output = RLE;
    /// If dimensions of self and rhs are not same this method will panic.
    fn sub(self, rhs: &'a RLE) -> Self::Output {
        assert!(self.width == rhs.width && self.height == rhs.height);
        self.bitand(&rhs.flip_bits())
    }
}

impl BitAnd for RLE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        &self & &rhs
    }
}

/// Binary and on image pixels.
impl<'a, 'b> BitAnd<&'a RLE> for &'b RLE {
    type Output = RLE;
    /// If dimensions of self and rhs are not same this method will panic.
    fn bitand(self, rhs: &'a RLE) -> Self::Output {
        assert!(self.width == rhs.width && self.height == rhs.height);
        if self.runs.is_empty() || rhs.runs.is_empty() {
            return RLE {
                runs: Vec::new(),
                width: self.width,
                height: self.height,
            };
        }
        let mut runs = Vec::with_capacity(self.runs.len());
        let mut i = 0;
        let mut j = 0;
        while i < self.runs.len() && j < rhs.runs.len() {
            if self.runs[i].y < rhs.runs[j].y {
                i += 1;
                continue;
            }
            else if self.runs[i].y > rhs.runs[j].y {
                j += 1;
                continue;
            }
            else if self.runs[i].intersects(rhs.runs[j]) {
                runs.push(self.runs[i].intersect(rhs.runs[j]).unwrap());
            }
            if self.runs[i].x_end < rhs.runs[j].x_end  {
                i += 1;
            } else {
                j += 1;
            }
        }
        RLE {
            runs,
            width: self.width,
            height: self.height,
        }.merge_overlapping_runs()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Image, Run};

    #[test]
    fn and_test() {
        let a = Image::new(6, 6, vec![
            1, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 1, 1, 1, 0,
            0, 0, 1, 0, 0, 0,
            0, 0, 1, 1, 1, 1,
            0, 0, 0, 0, 0, 0,
        ]);
        let b = Image::new(6, 6, vec![
            0, 1, 1, 1, 0, 0,
            0, 0, 0, 1, 0, 0,
            0, 0, 1, 1, 1, 0,
            0, 0, 1, 0, 0, 0,
            0, 0, 1, 1, 0, 1,
            0, 0, 0, 0, 0, 0,
        ]);
        let rle = &RLE::from(&a) & &RLE::from(&b);
        assert_eq!(
            rle.to_image(1),
            Image::new(6, 6,vec![
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 1, 1, 1, 0,
            0, 0, 1, 0, 0, 0,
            0, 0, 1, 1, 0, 1,
            0, 0, 0, 0, 0, 0,
            ])
        );
        assert_eq!(
            rle,
            RLE {
                width: 6,
                height: 6,
                runs: vec![
                    Run { x_start: 2, x_end: 4, y: 2 },
                    Run { x_start: 2, x_end: 2, y: 3 },
                    Run { x_start: 2, x_end: 3, y: 4 },
                    Run { x_start: 5, x_end: 5, y: 4 },
                ]
            }
        );
    }

    #[test]
    fn sub_test() {
        let a = Image::new(6, 6, vec![
            1, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0,
            0, 0, 1, 1, 1, 0,
            0, 0, 1, 0, 0, 0,
            0, 0, 1, 1, 1, 1,
            0, 0, 0, 0, 0, 0,
        ]);
        let b = Image::new(6, 6, vec![
            0, 1, 1, 1, 0, 0,
            0, 0, 0, 1, 0, 0,
            0, 0, 1, 1, 1, 0,
            0, 0, 1, 0, 0, 0,
            0, 0, 1, 1, 1, 1,
            0, 0, 0, 0, 0, 0,
        ]);
        let rle = &RLE::from(&a) - &RLE::from(&b);
        assert_eq!(
            rle.to_image(1),
            Image::new(6, 6,vec![
            1, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            ])
        );
        assert_eq!(
            rle,
            RLE {
                width: 6,
                height: 6,
                runs: vec![
                    Run { x_start: 0, x_end: 0, y: 0 },
                    Run { x_start: 5, x_end: 5, y: 0 },
                ]
            }
        );
    }
}
