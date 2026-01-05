use std::{
    ops::Range,
    time::{Duration, Instant},
};

use crate::PatternGenerator;

/// Generates a random value between the given range every tick.
#[derive(Clone, Debug, PartialEq)]
pub struct Random {
    pub range: Range<f64>,
    pub duration: Duration,
}

impl Random {
    pub fn new(range: Range<f64>, duration: Duration) -> Self {
        Random { range, duration }
    }
}

impl PatternGenerator for Random {
    fn sample(&mut self, _time: Duration) -> f64 {
        rand::random_range(self.range.clone())
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}

/// Generates a random value between the given range every `interval` seconds.
///
/// This can not generate random values faster than the driver tickrate,
/// and may skip values if the driver is not fast enough.
#[derive(Clone, Debug, PartialEq)]
pub struct RandomEvery {
    pub range: Range<f64>,
    pub duration: Duration,
    pub interval: f64,
    last_time: Instant,
    last_value: f64,
}

impl RandomEvery {
    pub fn new(range: Range<f64>, duration: Duration, interval: f64) -> Self {
        let initial = rand::random_range(range.clone());
        RandomEvery {
            range,
            duration,
            interval,
            last_time: Instant::now(),
            last_value: initial,
        }
    }
}

impl PatternGenerator for RandomEvery {
    fn sample(&mut self, _time: Duration) -> f64 {
        if self.last_time.elapsed().as_secs_f64() > self.interval {
            self.last_time = Instant::now();
            self.last_value = rand::random_range(self.range.clone());
        }
        self.last_value
    }

    fn duration(&self) -> Duration {
        self.duration
    }

    fn reset(&mut self) {
        self.last_value = rand::random_range(self.range.clone());
    }
}

/// Randomly increases and decreases a value between the given range every tick.
#[derive(Clone, Debug, PartialEq)]
pub struct RandomWalk {
    pub range: Range<f64>,
    pub duration: Duration,
    pub increase: f64,
    pub decrease: f64,
    state: f64,
}

impl RandomWalk {
    pub fn new(range: Range<f64>, duration: Duration, increase: f64, decrease: f64) -> Self {
        RandomWalk {
            range,
            duration,
            increase,
            decrease,
            state: 0.0,
        }
    }
}

impl PatternGenerator for RandomWalk {
    fn sample(&mut self, _time: Duration) -> f64 {
        let value = rand::random_range(self.range.clone());
        self.state += if value > self.state {
            self.increase
        } else {
            -self.decrease
        }
        .max(self.range.start)
        .min(self.range.end);
        self.state
    }

    fn duration(&self) -> Duration {
        self.duration
    }

    fn reset(&mut self) {
        self.state = 0.0;
    }
}
