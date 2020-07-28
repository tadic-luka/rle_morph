use super::{Run, RLE};

pub struct FlipBitsIter<'rle> {
    runs: std::slice::Iter<'rle, Run>,
    width: usize,
    height: usize,
    state: IterState,
}

#[derive(Debug, Copy, Clone)]
enum IterState {
    Empty { pos: usize },
    BlankLines { pos: i32, end: i32, run: Run },
    BlankLinesAfterLastRun { pos: i32, end: i32 },
    BetweenRuns(Run),
}

impl<'rle> FlipBitsIter<'rle> {
    pub fn new(rle: &'rle RLE) -> Self {
        if rle.runs().is_empty() {
            return Self {
                runs: rle.runs().iter(),
                width: rle.width(),
                height: rle.height(),
                state: IterState::Empty{ pos: 0 }
            };
        }
        let mut runs = rle.runs().iter();
        let first_run = runs.next().unwrap();
        Self {
            runs,
            width: rle.width(),
            height: rle.height(),
            state: IterState::BlankLines { pos: 0, end: first_run.y, run: *first_run },
        }
    }
    fn advance_between_next_run(&mut self, run: Run) -> Option<Run> {
        if let Some(&next_run) = self.runs.next() {
            if run.y == next_run.y {
                self.state = IterState::BetweenRuns(next_run);
                return Some(Run { x_start: run.x_end + 1, x_end: next_run.x_start - 1, y: run.y});
            }
            if run.x_end < self.width as i32 - 1 {
                self.state = IterState::BlankLines { pos: run.y + 1, end: next_run.y, run: next_run };
                return Some(Run { x_start: run.x_end + 1, x_end: self.width as i32 - 1, y: run.y });
            }
            if next_run.y > run.y + 1 {
                self.state = IterState::BlankLines { pos: run.y + 2, end: next_run.y, run: next_run };
                return Some(Run { x_start: 0, x_end: self.width as i32 - 1, y: run.y + 1 });
            }
            return self.advance_between_next_run(next_run);
        }
        if run.x_end < self.width as i32 - 1 {
            self.state = IterState::BlankLinesAfterLastRun { pos: run.y + 1, end: self.height as i32 };
            return Some(Run { x_start: run.x_end + 1, x_end: self.width as i32 - 1, y: run.y });
        }
        if run.y == self.height as i32 - 1 {
            return None;
        }
        self.state = IterState::BlankLinesAfterLastRun { pos: run.y + 2, end: self.height as i32 };
        return Some(Run { x_start: 0, x_end: self.width as i32 - 1, y: run.y + 1 });
    }
}
impl<'rle> Iterator for FlipBitsIter<'rle> {
    type Item = Run;
    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            IterState::Empty { pos } => {
                if pos < self.height {
                    self.state = IterState::Empty { pos: pos + 1 };
                    Some(Run { x_start: 0, x_end: self.width as i32 - 1, y: pos as i32})
                } else {
                    None
                }
            }
            IterState::BlankLines { pos, end, run } => {
                if pos < end {
                    self.state = IterState::BlankLines { pos: pos + 1, end, run };
                    Some(Run { x_start: 0, x_end: self.width as i32 - 1, y: pos })
                } else if run.x_start > 0 {
                    self.state = IterState::BetweenRuns(run);
                    Some(Run { x_start: 0, x_end: run.x_start - 1, y: run.y })
                } else {
                    self.advance_between_next_run(run)
                }

            }
            IterState::BlankLinesAfterLastRun { pos, end } => {
                if pos < end {
                    self.state = IterState::BlankLinesAfterLastRun { pos: pos + 1, end };
                    Some(Run { x_start: 0, x_end: self.width as i32 - 1, y: pos })
                } else {
                    None
                }
            }
            IterState::BetweenRuns(run) => {
                self.advance_between_next_run(run)
            }
        }
    }

}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Image;

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
        let runs: Vec<Run> = RLE::from(&img).flip_bits_iter().collect();
        assert_eq!(
            RLE::from_runs(6, 6, runs.clone()).to_image(1),
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
            RLE::from_runs(6, 6, RLE::from_runs(6, 6, runs.clone()).flip_bits_iter().collect()).to_image(1),
            img
        );
        assert_eq!(
            runs,
            vec![
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
        );
    }
}
