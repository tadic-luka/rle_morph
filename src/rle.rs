use super::{Image, Run};


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
    /// Create RLE binary image with all pixels 0
    #[inline]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            runs: Vec::new(),
        }
    }

    /// Create RLE binary image with all pixels 1
    pub fn ones(width: usize, height: usize) -> Self {
        Self {
            runs: (0..height).map(|y| Run {x_start: 0, x_end: width as _, y: y as _}).collect(),
            width,
            height,
        }
    }

    /// Create RLE binary image from raw pixels.
    /// If w * h != data.len() then this will panic.
    /// All pixel values greater than 0 will be treated as binary value 1 else 0.
    pub fn from_raw_data(w: usize, h: usize, data: &[u8]) -> Self {
        assert_eq!(w * h, data.len());
        let mut runs = Vec::new();
        for y in 0..h {
            let mut state = EncodeState::NotRunning;
            let mut run = Run::default();
            run.y = y as _;
            for current in y * w..(y+1) * w {
                if data[current] > 0 {
                    state = match state {
                        // if we were not in run create run
                        EncodeState::NotRunning => {
                            run.x_start = (current - y * w) as _;
                            run.x_end = run.x_start;
                            EncodeState::Running
                        }
                        // if we were in run then just increment interval
                        EncodeState::Running => {
                            run.x_end += 1;
                            EncodeState::Running
                        },
                    };
                } else {
                    state = match state {
                        // if we were in run now stop and add in collection
                        EncodeState::Running => {
                            runs.push(run);
                            EncodeState::NotRunning
                        },
                        // otherwise continue
                        EncodeState::NotRunning => state,
                    };
                }
            }
            // in the end if we were in run add that run into collection
            if let EncodeState::Running = state {
                runs.push(run);
            }

        }
        Self {
            width: w,
            height: h,
            runs
        }

    }

    /// Structuring element for dilation/erosion using l1 norm (manhattan distance)
    #[inline]
    pub fn l1_structuring(k: usize) -> Self {
        let dim = 2 * k + 1;
        let center = (dim / 2) as i32;
        let mut runs = Vec::with_capacity(dim);
        // Center row has all 1, every other row has number of rows
        // equal to center row - distance to current row.
        // This means that pixels are mirrored.
        for i in 0..dim as i32 {
            if i <= center {
                runs.push(Run::default()
                    .with_x_start(center - i)
                    .with_x_end(center + i)
                    .with_y(i)
                );
            } else {
                // every row after center is mirrored
                let run = runs[dim - i as usize - 1];
                runs.push(run.with_y(i));
            }
        }
        Self {
            width: dim,
            height: dim,
            runs,
        }
    }

    /// Structuring element for dilation/erosion using linf norm (maximum norm)
    #[inline]
    pub fn linf_structuring(k: usize) -> Self {
        let dim = 2 * k + 1;
        let center = (dim / 2) as i32;
        // all pixels are 1
        let mut runs = (0..dim as i32).map(|i| Run {
            x_start: 0,
            x_end: (dim - 1) as i32,
            y: i
        }).collect();
        Self {
            width: dim,
            height: dim,
            runs,
        }
    }

    /// Set Run of pixels to 1.
    /// This method will also try to merge runs
    #[inline]
    fn add_run(&mut self, run: Run) {
        self.runs.push(run);
        self.merge_overlapping_runs_mut();
    }

    /// Merge overlapping runs in this RLE.
    /// Mutable version
    #[inline]
    pub fn merge_overlapping_runs_mut(&mut self) {
        loop {
            let last_cnt = self.runs.len();
            Run::merge_overlapping_runs_mut(&mut self.runs);
            if last_cnt == self.runs.len() {
                break;
            }
        }
    }

    /// Merge overlapping runs in this RLE.
    /// Move version.
    #[inline]
    pub fn merge_overlapping_runs(mut self) -> Self {
        self.merge_overlapping_runs_mut();
        self
    }

    /// Decode RLE to binary image (0s and 1s).
    #[inline]
    pub fn to_image(&self, pixel_val: u8) -> Image {
        let mut output = vec![0; self.width * self.height];
        for &run in self.runs.iter().filter(|run| run.y >= 0 && run.y < self.height as _) {
            let y = run.y as usize;
            let start = std::cmp::max(0, run.x_start) as usize;
            let end = std::cmp::max(0, std::cmp::min(run.x_end as usize, self.width - 1));
            let col = &mut output[y * self.width..(y + 1) * self.width];
            for i in start..end + 1 {
                col[i] = pixel_val;
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
        Self {
            runs: primary_runs,
            width: self.width,
            height: self.height,
        }.merge_overlapping_runs()
    }

    pub fn erode(&self, s: &Self) -> Self {
        self.flip_bits().dilate(s).flip_bits()
    }

    #[inline]
    pub fn runs(&self) -> &[Run] {
        &self.runs
    }

    /// Flip bits (1s -> 0s, 0s -> 1s)
    pub fn flip_bits(&self) -> Self {
        // if all bits are 0, then set all 'height' runs with 'width' length 
        if self.runs.is_empty() {
            return Self {
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
        Self {
            width: self.width,
            height: self.height,
            runs
        }
    }


    /// Get all 1s in self which are not in other, same as set difference.
    /// If dimensions of self and other are not same this method will panic.
    pub fn diff(&self, other: &Self) -> Self {
        self.and(&other.flip_bits())
    }

    /// Binary or on image pixels, mutable version.
    /// If dimensions of self and other are not same this method will panic.
    pub fn or_mut(&mut self, other: &Self) {
        assert!(self.width == other.width && self.height == other.height);
        self.runs.reserve(other.runs.len());
        self.runs.extend(&other.runs);

        self.merge_overlapping_runs_mut();
    }

    /// Binary or on image pixels.
    /// If dimensions of self and other are not same this method will panic.
    pub fn or(&self, other: &Self) -> Self {
        assert!(self.width == other.width && self.height == other.height);
        let mut runs = Vec::with_capacity(self.runs.len() + other.runs.len());
        runs.extend(&self.runs);
        runs.extend(&other.runs);
        Self {
            runs,
            width: self.width,
            height: self.height,
        }.merge_overlapping_runs()
    }

    /// Binary and on image pixels.
    /// If dimensions of self and other are not same this method will panic.
    pub fn and(&self, other: &Self) -> Self {
        assert!(self.width == other.width && self.height == other.height);
        if self.runs.is_empty() || other.runs.is_empty() {
            return Self {
                runs: Vec::new(),
                width: self.width,
                height: self.height,
            };
        }
        let mut runs = Vec::with_capacity(self.runs.len());
        let mut i = 0;
        let mut j = 0;
        while i < self.runs.len() && j < other.runs.len() {
            if self.runs[i].y < other.runs[j].y {
                i += 1;
                continue;
            }
            else if self.runs[i].y > other.runs[j].y {
                j += 1;
                continue;
            }
            else if self.runs[i].intersects(other.runs[j]) {
                runs.push(self.runs[i].intersect(other.runs[j]).unwrap());
            }
            if self.runs[i].x_end < other.runs[j].x_end  {
                i += 1;
            } else {
                j += 1;
            }
        }
        Self {
            runs,
            width: self.width,
            height: self.height,
        }.merge_overlapping_runs()
    }

    /// Get image width.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get image height.
    pub fn height(&self) -> usize {
        self.height
    }

}

#[derive(Debug, Clone, Copy)]
enum EncodeState {
    NotRunning,
    Running,
}
impl From<&Image> for RLE {
    fn from(img: &Image) -> RLE {
        let w = img.w();
        let h = img.h();
        let data = img.data();
        RLE::from_raw_data(w, h, &data)
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
        let img = a.to_image(1);
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
        let img = a.to_image(1);
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

        let img = Image::new(3, 5, vec![
            0, 0, 0, 
            0, 1, 1, 
            0, 1, 0,
            1, 0, 0,
            0, 0, 0
        ]);
        let rle = RLE::from(&img);
        assert_eq!(rle, RLE {
            width: 3,
            height: 5,
            runs: vec![
                Run {
                    x_start: 1,
                    x_end: 2,
                    y: 1,
                },
                Run {
                    x_start: 1,
                    x_end: 1,
                    y: 2
                },
                Run {
                    x_start: 0,
                    x_end: 0,
                    y: 3
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
        assert_eq!(img, rle.to_image(1));
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
        let result = rle.dilate(&RLE::from(&dilate)).to_image(1);

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
        let result = rle.dilate(&RLE::from(&dilate)).to_image(1);
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

    #[test]
    fn erode_test() {
        let orig = Image::new(6, 6, vec![
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0,
            0, 1, 1, 1, 0, 0,
            0, 0, 1, 0, 0, 0,
            0, 0, 1, 0, 0, 0,
        ]);
        let erode = Image::new(3, 3, vec![
            0, 1, 0,
            1, 1, 1,
            0, 1, 0
        ]);
        let rle = RLE::from(&orig);
        let result = rle.erode(&RLE::from(&erode)).to_image(1);
        assert_eq!(result,
            Image::new(
                6, 6,
                vec![
                    0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                    0, 0, 1, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0
                ])
        );

        let orig = Image::new(5, 5, vec![
            0, 0, 0, 0, 0,
            0, 1, 1, 1, 0,
            0, 1, 1, 1, 0,
            0, 1, 1, 1, 0,
            0, 1, 1, 1, 0,
        ]);
        let erode = Image::new(3, 3, vec![
            1, 1, 1,
            1, 1, 1,
            1, 1, 1
        ]);
        let rle = RLE::from(&orig);
        let result = rle.erode(&RLE::from(&erode)).to_image(1);
        assert_eq!(result,
            Image::new(
                5, 5,
                vec![
                    0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0,
                    0, 0, 1, 0, 0,
                    0, 0, 1, 0, 0,
                    0, 0, 1, 0, 0,
                ])
        );
    }

    #[test]
    fn l1_structuring_test() {
        // manhattan distance of 0, no dilation
        let r = RLE::l1_structuring(0);
        let expected = Image::new(
            1,1,
            vec![1]
        );
        assert_eq!(r, RLE::from(&expected));

        // manhattan distance of 1
        let r = RLE::l1_structuring(1);
        let expected = Image::new(
            3,3,
            vec![
                0, 1, 0,
                1, 1, 1,
                0, 1, 0,
            ]
        );
        assert_eq!(r, RLE::from(&expected));
        // manhattan distance of 1
        let r = RLE::l1_structuring(1);
        let expected = Image::new(
            3,3,
            vec![
                0, 1, 0,
                1, 1, 1,
                0, 1, 0,
            ]
        );
        assert_eq!(r, RLE::from(&expected));
        // manhattan distance of 1
        let r = RLE::l1_structuring(1);
        let expected = Image::new(
            3,3,
            vec![
                0, 1, 0,
                1, 1, 1,
                0, 1, 0,
            ]
        );
        assert_eq!(r, RLE::from(&expected));
        // manhattan distance of 1
        let r = RLE::l1_structuring(1);
        let expected = Image::new(
            3,3,
            vec![
                0, 1, 0,
                1, 1, 1,
                0, 1, 0,
            ]
        );
        assert_eq!(r, RLE::from(&expected));

        // manhattan distance of 2
        let r = RLE::l1_structuring(2);
        let expected = Image::new(
            5, 5,
            vec![
                0, 0, 1, 0, 0,
                0, 1, 1, 1, 0,
                1, 1, 1, 1, 1,
                0, 1, 1, 1, 0,
                0, 0, 1, 0, 0,
            ]
        );
        assert_eq!(r, RLE::from(&expected));

        // manhattan distance of 3
        let r = RLE::l1_structuring(3);
        let expected = Image::new(
            7, 7,
            vec![
                0, 0, 0, 1, 0, 0, 0,
                0, 0, 1, 1, 1, 0, 0,
                0, 1, 1, 1, 1, 1, 0,
                1, 1, 1, 1, 1, 1, 1,
                0, 1, 1, 1, 1, 1, 0,
                0, 0, 1, 1, 1, 0, 0,
                0, 0, 0, 1, 0, 0, 0,
            ]
        );
        assert_eq!(r, RLE::from(&expected));
    }

    #[test]
    fn linf_structuring_test() {
        let r = RLE::linf_structuring(1);
        let expected = Image::new(
            3,3,
            vec![
                1, 1, 1,
                1, 1, 1,
                1, 1, 1,
            ]
        );
        assert_eq!(r, RLE::from(&expected));

        let r = RLE::linf_structuring(2);
        let expected = Image::new(
            5, 5,
            vec![
                1, 1, 1, 1, 1,
                1, 1, 1, 1, 1,
                1, 1, 1, 1, 1,
                1, 1, 1, 1, 1,
                1, 1, 1, 1, 1,
            ]
        );
        assert_eq!(r, RLE::from(&expected));

        let r = RLE::linf_structuring(3);
        let expected = Image::new(
            7, 7,
            vec![
                1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1,
            ]
        );
        assert_eq!(r, RLE::from(&expected));
    }

    #[test]
    fn flip_bits_test() {
        let img = Image::new(6, 6, vec![
            0, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 1, 1, 1, 0,
            0, 0, 1, 0, 0, 0,
            0, 0, 1, 1, 1, 0,
            0, 0, 0, 0, 0, 0,
        ]);
        let rle = RLE::from(&img).flip_bits();
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
            rle.flip_bits().to_image(1),
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

    #[test]
    fn diff_test() {
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
        let rle = RLE::from(&a).diff(&RLE::from(&b));
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
        let rle = RLE::from(&a).and(&RLE::from(&b));
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
}
