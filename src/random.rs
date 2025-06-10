use std::{ops::Range, time::Instant};

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

/// Generates a random value between the given range every `every` seconds.
/// 
/// This can not generate random values faster than the driver tickrate, 
/// and may skip values if the driver is not fast enough.
pub struct RandomEvery {
    pub range: Range<f64>,
    pub duration: f64,
    pub every: f64,
    last_time: Instant,
    last_value: f64,
}

impl RandomEvery {
    pub fn new(range: Range<f64>, duration: f64, every: f64) -> Self {
        let initial = rand::random_range(range.clone());
        RandomEvery {
            range,
            duration,
            every,
            last_time: Instant::now(),
            last_value: initial,
        }
    }
}

impl PatternGenerator for RandomEvery {
    fn sample(&mut self, _time: f64) -> f64 {
        if self.last_time.elapsed().as_secs_f64() > self.every {
            self.last_time = Instant::now();
            self.last_value = rand::random_range(self.range.clone());
        }
        self.last_value
    }

    fn duration(&self) -> f64 {
        self.duration
    }
}
