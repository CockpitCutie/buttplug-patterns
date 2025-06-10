use std::ops::Range;

use crate::PatternGenerator;

/// Generates a random value between the given range every tick.
pub struct Random {
    pub range: Range<f64>,
    pub duration: f64,
}

impl Random {
    pub fn new(range: Range<f64>, duration: f64) -> Self {
        Random { range, duration }
    }
}

impl PatternGenerator for Random {
    fn sample(&mut self, _time: f64) -> f64 {
        rand::random_range(self.range.clone())
    }

    fn duration(&self) -> f64 {
        self.duration
    }
}
