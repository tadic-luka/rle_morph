use super::{Run, RLE};
use std::ops::{BitAnd, Sub, BitOr, BitOrAssign, BitAndAssign, Not};

/// Flip bits (1s -> 0s, 0s -> 1s)
impl<'a> Not for &'a RLE {
    type Output = RLE;
    fn not(self) -> Self::Output {
        // if all bits are 0, then set all 'height' runs with 'width' length 
        if self.runs.is_empty() {
            return RLE {
                width: self.width,
                height: self.height,
                runs: (0..self.height).map(|i| Run {
                    x_start: 0,
                    x_end: self.width as i32 - 1,
                    y: i as i32
                }).collect(),
            };
        }
        // runs has at least 1 element, set first as last
        let mut last_run = self.runs[0];
        let mut runs = Vec::new();
        // first create all 0 rows with y less than last_run.y to ones 
        runs.extend(
            (0..last_run.y).map(|y| Run { x_start: 0, x_end: self.width as i32 - 1, y: y as _ })
        );
        // if first run does not start from 0 then add run from 0 to x_start - 1
        if 0 < last_run.x_start {
            runs.push(Run {
                x_start: 0,
                x_end: last_run.x_start - 1,
                y: last_run.y,
            });
        }
        // go trough all other runs
        for &run in self.runs.iter().skip(1) {
            // if we are in same row 
            if run.y == last_run.y {
                // add run between last and current run (between them are zeroes)
                runs.push(Run {
                    x_start: last_run.x_end + 1,
                    x_end: run.x_start - 1,
                    y: run.y,
                });
            } else { // we are not in the same row
                // if last_run (run from previous row) did not go to end (width)
                // then add ones starting from last_run to end of width
                if self.width as i32 - 1 > last_run.x_end {
                    runs.push(Run {
                        x_start: last_run.x_end + 1,
                        x_end: self.width as i32 - 1,
                        y: last_run.y,
                    });
                }
                // add run for each row between last and current run
                // if current run row is right after last run row then this will not loop
                for i in last_run.y+1..run.y {
                    runs.push(Run {
                        x_start: 0,
                        x_end: self.width as i32 - 1,
                        y: i,
                    });
                }
                // again if first run in this row does not start from 0 then add run with 1 
                // from start to run.x_start
                if 0 < run.x_start {
                    runs.push(Run {
                        x_start: 0,
                        x_end: run.x_start - 1,
                        y: run.y,
                    });
                }
            }
            last_run = run;
        }
        if self.width as i32 - 1 > last_run.x_end {
            runs.push(Run {
                x_start: last_run.x_end + 1,
                x_end: self.width as i32 - 1,
                y: last_run.y,
            });
        }
        // in the end if last_run is not really last row
        // all rows after that are zeroes which are flipped to ones
        runs.extend(
            (last_run.y + 1..self.height as i32).map(|y| Run { x_start: 0, x_end: self.width as i32 - 1, y: y as _ })
        );
        RLE {
            width: self.width,
            height: self.height,
            runs
        }
    }
}

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
        self.bitand(&!rhs)
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

    #[test]
    fn not_test() {
        let img = Image::new(6, 6, vec![
            0, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 1, 1, 1, 0,
            0, 0, 1, 0, 0, 0,
            0, 0, 1, 1, 1, 0,
            0, 0, 0, 0, 0, 0,
        ]);
        let rle = !&RLE::from(&img);
        assert_eq!(
            rle.to_image(1),
            Image::new(6, 6,vec![
            1, 0, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1,
            1, 1, 0, 0, 0, 1,
            1, 1, 0, 1, 1, 1,
            1, 1, 0, 0, 0, 1,
            1, 1, 1, 1, 1, 1,
            ])
        );
        assert_eq!(
            (!&rle).to_image(1),
            img
        );
        assert_eq!(
            rle,
            RLE {
                width: 6,
                height: 6,
                runs: vec![
                    Run { x_start: 0, x_end: 0, y: 0 },
                    Run { x_start: 2, x_end: 5, y: 0 },
                    Run { x_start: 0, x_end: 5, y: 1 },
                    Run { x_start: 0, x_end: 1, y: 2 },
                    Run { x_start: 5, x_end: 5, y: 2 },
                    Run { x_start: 0, x_end: 1, y: 3 },
                    Run { x_start: 3, x_end: 5, y: 3 },
                    Run { x_start: 0, x_end: 1, y: 4 },
                    Run { x_start: 5, x_end: 5, y: 4 },
                    Run { x_start: 0, x_end: 5, y: 5 },
                ]
            }
        );
    }
}
