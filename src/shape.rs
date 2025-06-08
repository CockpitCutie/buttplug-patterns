use crate::Pattern;

pub struct Constant {
    level: f64,
}

impl Constant {
    pub fn new(level: f64) -> Self {
        return Constant { level };
    }
}

impl Pattern for Constant {
    fn sample(&self, _time: f64) -> f64 {
        self.level
    }
}

pub struct SquareWave {
    up: f64,
    down: f64,
    frequency_hz: f64,
}

impl SquareWave {
    pub fn new(up: f64, down: f64, frequency_hz: f64) -> Self {
        SquareWave {
            up,
            down,
            frequency_hz,
        }
    }
}

impl Pattern for SquareWave {
    fn sample(&self, time: f64) -> f64 {
        if time % self.frequency_hz < self.frequency_hz / 2.0 {
            self.up
        } else {
            self.down
        }
    }
}

pub struct SineWave {
    amplitude: f64,
    frequency_hz: f64,
}

impl SineWave {
    pub fn new(amplitude: f64, frequency_hz: f64) -> Self {
        SineWave {
            amplitude,
            frequency_hz,
        }
    }
}

impl Pattern for SineWave {
    fn sample(&self, time: f64) -> f64 {
        return (self.amplitude / 2.0) * f64::sin(2.0 * 3.14 * self.frequency_hz * time)
            + self.amplitude / 2.0;
    }
}
