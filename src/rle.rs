use super::Image;
use std::cmp::Ordering;

/// Sequence of '1' (or '0') pixels horizontally (or vertically) in binary image.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Run {
    /// coordinate (column) of left-most (starting) pixel
    pub x_start: i32,
    /// coordinate (column) of right-most (ending) pixel
    pub x_end: i32,
    /// y coordinate of current Run
    pub y: i32,
}

impl Run {
    #[inline]
    fn overlaps(mut self, other: Self) -> bool {
        match self.cmp(&other) {
            Ordering::Equal => true,
            Ordering::Less => {
                self.y == other.y && 
                    self.x_start <= other.x_start &&
                    self.x_end >= other.x_start
            }
            Ordering::Greater => other.overlaps(self)
        }
    }

    #[inline]
    fn merge(self, other: Self) -> Self {
        if !self.overlaps(other) {
            return self;
        }
        return Self {
            x_start: std::cmp::min(self.x_start, other.x_start),
            x_end: std::cmp::max(self.x_end, other.x_end),
            y: self.y
        }
    }

    #[inline]
    pub fn with_x_start(self, x_start: i32) -> Self {
        Self {
            x_start, 
            ..self
        }
    }

    #[inline]
    pub fn with_x_end(self, x_end: i32) -> Self {
        Self {
            x_end, 
            ..self
        }
    }

    #[inline]
    pub fn with_y(self, y: i32) -> Self {
        Self {
            y, 
            ..self
        }
    }

    pub fn merge_overlapping_runs_mut(runs: &mut Vec<Self>) {
        runs.sort_unstable();
        if runs.len() < 2 {
            return;
        }
        let mut res = Vec::new();
        res.push(runs[0]);
        for &current in &runs[1..] {
            let top = res.pop().unwrap();
            if !current.overlaps(top) {
                res.push(top);
                res.push(current);
            } else {
                res.push(top.merge(current));
            }
        }
        *runs = res;
    }
}

impl PartialOrd for Run {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Run {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        // first order by height then by left column and then by right
        // so that all runs of same height are one after another
        self.y
            .cmp(&other.y)
            .then(self.x_start.cmp(&other.x_start))
            .then(self.x_end.cmp(&other.x_end))
    }
}

/// Representation of a binary image using a combinations of runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RLE {
    /// width of image
    width: usize,
    /// height of image
    height: usize,
    /// runs of image
    runs: Vec<Run>,
}

impl RLE {
    #[inline]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            runs: Vec::new(),
        }
    }

    #[inline]
    pub fn l1_structuring(k: usize) -> Self {
        let dim = 2 * k + 1;
        let center = (dim / 2) as i32;
        let mut runs = Vec::with_capacity(dim);
        runs.push(Run::default()
            .with_x_start(0)
            .with_x_end((dim - 1) as i32)
            .with_y(center)
        );
        for i in 0..dim as i32 {
            if i == center {
                continue;
            }
            runs.push(Run::default()
                .with_x_start(center)
                .with_x_end(center)
                .with_y(i)
            );
        }
        Self {
            width: dim,
            height: dim,
            runs,
        }
    }

    #[inline]
    pub fn linf_structuring(k: usize) -> Self {
        let dim = 2 * k + 1;
        let center = (dim / 2) as i32;
        let mut runs = Vec::with_capacity(dim);
        for i in 0..dim as i32 {
            runs.push(Run::default()
                .with_x_start(0)
                .with_x_end((dim - 1) as i32)
                .with_y(i)
            );
        }
        Self {
            width: dim,
            height: dim,
            runs,
        }
    }

    #[inline]
    pub fn add_run(&mut self, run: Run) {
        self.runs.push(run);
    }

    pub fn merge_overlapping_runs_mut(&mut self) {
        Run::merge_overlapping_runs_mut(&mut self.runs);
    }

    #[inline]
    pub fn merge_overlapping_runs(mut self) -> Self {
        self.merge_overlapping_runs_mut();
        self
    }

    #[inline]
    pub fn to_image(&self) -> Image {
        let mut output = vec![0; self.width * self.height];
        for &run in self.runs.iter().filter(|run| run.y >= 0 && run.y < self.height as _) {
            let y = run.y as usize;
            let start = std::cmp::max(0, run.x_start) as usize;
            let end = std::cmp::max(0, std::cmp::min(run.x_end as usize, self.width));
            let col = &mut output[y * self.width..(y + 1) * self.width];
            for i in start..end + 1 {
                col[i] = 1;
            }
        }
        Image::new(self.width, self.height, output)
    }

    pub fn dilate(&self, s: &Self) -> Self {
        // find primary runs
        let mut primary_runs = Vec::with_capacity(self.runs.len() * s.runs.len());
        let delta_x: i32 = (s.width as i32 / 2);
        let delta_y: i32 =  (s.height as i32 / 2);
        for &a in &self.runs {
            for &b in &s.runs {
                primary_runs.push(
                    Run {
                        x_start: a.x_start - delta_x + b.x_start,
                        x_end: a.x_end - delta_x + b.x_end,
                        y: a.y + delta_y - b.y
                    }
                );
            }
        }
        loop {
            let len_before = primary_runs.len();
            Run::merge_overlapping_runs_mut(&mut primary_runs);
            if primary_runs.len() == len_before {
                break;
            }
        }
        Self {
            runs: primary_runs,
            width: self.width,
            height: self.height,
        }
    }

    #[inline]
    pub fn runs(&self) -> &[Run] {
        &self.runs
    }
}

#[derive(Debug, Clone, Copy)]
enum DecodeState {
    Start,
    RunStarted,
    RunEnded,
}
impl From<&Image> for RLE {
    fn from(img: &Image) -> RLE {
        let w = img.w();
        let h = img.h();
        let data = img.data();
        let mut runs = Vec::new();
        for y in 0..h {
            let mut state = DecodeState::Start;
            let mut run = Run::default();
            run.y = y as _;
            for current in y * w..(y+1) * w {
                if data[current] > 0 {
                    state = match state {
                        DecodeState::Start | DecodeState::RunEnded => {
                            run.x_start = (current - y * w) as _;
                            run.x_end = run.x_start;
                            DecodeState::RunStarted
                        }
                        DecodeState::RunStarted => {
                            run.x_end += 1;
                            DecodeState::RunStarted
                        },
                    };
                } else {
                    state = match state {
                        DecodeState::RunStarted => {
                            runs.push(run);
                            DecodeState::RunEnded
                        },
                        _ => state,
                    };
                }
            }
            if let DecodeState::RunStarted = state {
                runs.push(run);
            }

        }
        RLE {
            width: w,
            height: h,
            runs
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_y_same_r_same_l_test() {
        assert_eq!(
            Run {
                x_start: 0,
                x_end: 22,
                y: 0
            }
            .cmp(&Run {
                x_start: 0,
                x_end: 22,
                y: 0
            }),
            Ordering::Equal
        );
        assert_eq!(
            Run {
                x_start: 0,
                x_end: 0,
                y: 1
            }
            .cmp(&Run {
                x_start: 0,
                x_end: 0,
                y: 1
            }),
            Ordering::Equal
        );
    }

    #[test]
    fn same_y_same_r_diff_l_test() {
        assert_eq!(
            Run {
                x_start: 0,
                x_end: 22,
                y: 0
            }
            .cmp(&Run {
                x_start: 1,
                x_end: 22,
                y: 0
            }),
            Ordering::Less
        );

        assert_eq!(
            Run {
                x_start: 0,
                x_end: 1,
                y: 0
            }
            .cmp(&Run {
                x_start: 1,
                x_end: 22,
                y: 0
            }),
            Ordering::Less
        );
    }

    #[test]
    fn same_y_diff_r_same_l_test() {
        assert_eq!(
            Run {
                x_start: 0,
                x_end: 1,
                y: 0
            }
            .cmp(&Run {
                x_start: 0,
                x_end: 2,
                y: 0
            }),
            Ordering::Less
        );

        assert_eq!(
            Run {
                x_start: 0,
                x_end: 3,
                y: 0
            }
            .cmp(&Run {
                x_start: 0,
                x_end: 1,
                y: 0
            }),
            Ordering::Greater
        );
    }
    #[test]
    fn same_y_diff_r_diff_l_test() {
        assert_eq!(
            Run {
                x_start: 1,
                x_end: 22,
                y: 0
            }
            .cmp(&Run {
                x_start: 0,
                x_end: 13,
                y: 0
            }),
            Ordering::Greater
        );
        assert_eq!(
            Run {
                x_start: 0,
                x_end: 45,
                y: 0
            }
            .cmp(&Run {
                x_start: 2,
                x_end: 22,
                y: 0
            }),
            Ordering::Less
        );
    }
    #[test]
    fn diff_y_same_r_same_l_test() {
        assert_eq!(
            Run {
                x_start: 0,
                x_end: 0,
                y: 0
            }
            .cmp(&Run {
                x_start: 0,
                x_end: 0,
                y: 1
            }),
            Ordering::Less
        );
    }

    #[test]
    fn diff_y_same_r_diff_l_test() {
        assert_eq!(
            Run {
                x_start: 15,
                x_end: 22,
                y: 0
            }
            .cmp(&Run {
                x_start: 0,
                x_end: 22,
                y: 1
            }),
            Ordering::Less
        );
        assert_eq!(
            Run {
                x_start: 0,
                x_end: 22,
                y: 0
            }
            .cmp(&Run {
                x_start: 1,
                x_end: 22,
                y: 1
            }),
            Ordering::Less
        );
    }
    #[test]
    fn diff_y_diff_r_same_l_test() {
        assert_eq!(
            Run {
                x_start: 0,
                x_end: 15,
                y: 0
            }
            .cmp(&Run {
                x_start: 0,
                x_end: 22,
                y: 1
            }),
            Ordering::Less
        );
        assert_eq!(
            Run {
                x_start: 0,
                x_end: 22,
                y: 0
            }
            .cmp(&Run {
                x_start: 0,
                x_end: 15,
                y: 1
            }),
            Ordering::Less
        );
    }
    #[test]
    fn diff_y_diff_r_diff_l_test() {
        assert_eq!(
            Run {
                x_start: 15,
                x_end: 22,
                y: 0
            }
            .cmp(&Run {
                x_start: 0,
                x_end: 22,
                y: 1
            }),
            Ordering::Less
        );
        assert_eq!(
            Run {
                x_start: 0,
                x_end: 22,
                y: 0
            }
            .cmp(&Run {
                x_start: 15,
                x_end: 22,
                y: 1
            }),
            Ordering::Less
        );
    }

    #[test]
    fn run_overlap_test() {
        assert!(
            Run {
                x_start: 0,
                x_end: 10,
                y: 0,
            }.overlaps(
                Run {
                    x_start: 1,
                    x_end: 11,
                    y: 0,
                }
            )
        );
        assert!(
            Run {
                x_start: 0,
                x_end: 10,
                y: 0,
            }.overlaps(
                Run {
                    x_start: 11,
                    x_end: 12,
                    y: 0,
                }
            ) == false
        );

        assert!(
            Run {
                x_start: 0,
                x_end: 10,
                y: 1,
            }.overlaps(
                Run {
                    x_start: 11,
                    x_end: 12,
                    y: 0,
                }
            ) == false
        );

        assert!(
            Run {
                x_start: 0,
                x_end: 10,
                y: 1,
            }.overlaps(
                Run {
                    x_start: 0,
                    x_end: 10,
                    y: 0,
                }
            ) == false
        );
    }

    #[test]
    fn run_merge_test() {
        assert_eq!(
            Run {
                x_start: 0,
                x_end: 10,
                y: 0,
            }.merge(
                Run {
                    x_start: 5,
                    x_end: 15,
                    y: 0
                }
            ),
            Run {
                    x_start: 0,
                    x_end: 15,
                    y: 0
            }
        );

        assert_eq!(
            Run {
                x_start: 1,
                x_end: 3,
                y: 0,
            }.merge(
                Run {
                    x_start: 2,
                    x_end: 4,
                    y: 0
                }
            ),
            Run {
                    x_start: 1,
                    x_end: 4,
                    y: 0
            }
        );
    }

    #[test]
    fn merge_overlapping_test() {
        let mut rle = RLE::new(16, 16);
        rle.add_run(Run {
            x_start: 0,
            x_end: 10,
            y: 0,
        });
        rle.add_run(Run {
            x_start: 5,
            x_end: 11,
            y: 0,
        });
        rle.add_run(Run {
            x_start: 5,
            x_end: 11,
            y: 1,
        });
        rle.merge_overlapping_runs_mut();
        assert_eq!(rle.runs.len(), 2);
        assert_eq!(rle.runs, vec![
            Run {
                x_start: 0,
                x_end: 11,
                y: 0,
            },
            Run {
                x_start: 5,
                x_end: 11,
                y: 1
            }
        ]);
    }

    #[test]
    fn encode_to_image_test() {
        let mut a = RLE::new(3, 3);
        a.add_run(Run {
            x_start: 0,
            x_end: 0,
            y: 0,
        });
        let img = a.to_image();
        assert_eq!(img.w(), a.width);
        assert_eq!(img.h(), a.height);
        assert_eq!(img.data(), &[
            1, 0, 0,
            0, 0, 0,
            0, 0, 0
        ]);

        a.add_run(Run {
            x_start: 0,
            x_end: 2,
            y: 2
        });
        let img = a.to_image();
        assert_eq!(img.data(), &[
            1, 0, 0,
            0, 0, 0,
            1, 1, 1
        ]);
            
    }

    #[test]
    fn decode_from_image_test() {
        let img = Image::new(3, 3, vec![
            0, 1, 0, 
            0, 1, 1, 
            0, 0, 0
        ]);
        let rle = RLE::from(&img);
        assert_eq!(rle, RLE {
            width: 3,
            height: 3,
            runs: vec![
                Run {
                    x_start: 1,
                    x_end: 1,
                    y: 0,
                },
                Run {
                    x_start: 1,
                    x_end: 2,
                    y: 1
                }
            ]
        });

        let img = Image::new(3, 3, vec![
            1, 1, 1, 
            1, 1, 1, 
            1, 1, 1
        ]);
        let rle = RLE::from(&img);
        assert_eq!(rle, RLE {
            width: 3,
            height:3,
            runs: vec![
                Run {
                    x_start: 0,
                    x_end: 2,
                    y: 0,
                },
                Run {
                    x_start: 0,
                    x_end: 2,
                    y: 1
                },
                Run {
                    x_start: 0,
                    x_end: 2,
                    y: 2
                }
                ]
        });
        
    }

    #[test]
    fn encode_decode_test() {
        let img = Image::new(3, 3, vec![
            0, 1, 0, 
            0, 1, 1, 
            0, 0, 0
        ]);
        let rle = RLE::from(&img);
        assert_eq!(img, rle.to_image());
    }

    #[test]
    fn dilate_test() {
        let orig = Image::new(6, 6, vec![
            0, 1, 0, 0, 0, 0,
            0, 0, 0, 1, 1, 0, 
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0,
        ]);
        let dilate = Image::new(3, 3, vec![
            1, 1, 1,
            1, 1, 1,
            1, 1, 1
        ]);
        let rle = RLE::from(&orig);
        let result = rle.dilate(&RLE::from(&dilate)).to_image();

        assert_eq!(result,
            Image::new(
                6, 6, 
                vec![
                    1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1,
                    0, 0, 1, 1, 1, 1,
                    0, 0, 0, 0, 0, 0,
                    0, 1, 1, 1, 0, 0,
                    0, 1, 1, 1, 0, 0
                ])
        );
        let dilate = Image::new(3, 3, vec![
            0, 1, 0,
            1, 1, 1,
            0, 1, 0
        ]);
        let rle = RLE::from(&orig);
        let result = rle.dilate(&RLE::from(&dilate)).to_image();
        assert_eq!(result,
            Image::new(
                6, 6,
                vec![
                    1, 1, 1, 1, 1, 0,
                    0, 1, 1, 1, 1, 1,
                    0, 0, 0, 1, 1, 0,
                    0, 0, 0, 0, 0, 0,
                    0, 0, 1, 0, 0, 0,
                    0, 1, 1, 1, 0, 0
                ])
        );
    }
}

