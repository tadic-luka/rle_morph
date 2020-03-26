use std::cmp::Ordering;
/// Sequence of '1' (or '0') pixels horizontally in binary image.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Run {
    /// coordinate (column) of left-most (starting) pixel
    pub x_start: i32,
    /// coordinate (column) of right-most (ending) pixel
    pub x_end: i32,
    /// y coordinate (row) of current Run
    pub y: i32,
}

impl Run {
    /// Check if self overlaps with other.
    /// Two runs overlap if their y value is same and if their (x_start, x_end)
    /// intervals overlap.
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


    /// Merge two runs.
    /// If self does not overlap with other then self is returned.
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

    /// Merges overlapping runs given vec
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
}
