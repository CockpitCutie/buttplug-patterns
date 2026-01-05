use std::{f64::consts::PI, time::Duration};

use crate::PatternGenerator;

/// Generates a zero value for a given duration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pause {
    duration: Duration,
}

impl Pause {
    pub fn new(duration: Duration) -> Self {
        Pause { duration }
    }
}

impl PatternGenerator for Pause {
    fn sample(&mut self, _time: Duration) -> f64 {
        0.0
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}

/// Generates a constant value for a given duration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constant {
    level: f64,
    duration: Duration,
}

impl Constant {
    pub fn new(level: f64, duration: Duration) -> Self {
        Constant { level, duration }
    }
}

impl PatternGenerator for Constant {
    fn sample(&mut self, _time: Duration) -> f64 {
        self.level
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}

/// Generates a linear value between two points for a given duration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Linear {
    from: f64,
    to: f64,
    duration: Duration,
}

impl Linear {
    pub fn new(from: f64, to: f64, duration: Duration) -> Self {
        Linear { from, to, duration }
    }
}

impl PatternGenerator for Linear {
    fn sample(&mut self, time: Duration) -> f64 {
        self.from + (self.to - self.from) * time.as_secs_f64() / self.duration.as_secs_f64()
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}

/// Generates a Saw wave between 0 and an amplitude for a given duration.
///
/// Waves are generated as single pulses with a given wavelength.
/// To play several cycles of the wave in sequence, use the `repeat` method.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SawWave {
    amplitude: f64,
    wavelength: Duration,
}

impl SawWave {
    pub fn new(amplitude: f64, wavelength: Duration) -> Self {
        SawWave {
            amplitude,
            wavelength,
        }
    }
}

impl PatternGenerator for SawWave {
    fn sample(&mut self, time: Duration) -> f64 {
        self.amplitude * (1.0 / self.wavelength.as_secs_f64()) * time.as_secs_f64() % 1.0
    }

    fn duration(&self) -> Duration {
        self.wavelength
    }
}

/// Generates a Triangle wave between 0 and an amplitude for a given duration.
///
/// Waves are generated as single pulses with a given wavelength.
/// To play several cycles of the wave in sequence, use the `repeat` method.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TriangleWave {
    amplitude: f64,
    wavelength: Duration,
}

impl TriangleWave {
    pub fn new(amplitude: f64, wavelength: Duration) -> Self {
        TriangleWave {
            amplitude,
            wavelength,
        }
    }
}

impl PatternGenerator for TriangleWave {
    fn sample(&mut self, time: Duration) -> f64 {
        // Formula for a triangle wave between 0 and `amplitude` with period `wavelength`
        // https://en.wikipedia.org/wiki/Triangle_wave#Definition
        ((2.0 * self.amplitude / self.wavelength.as_secs_f64())
            * (((time.as_secs_f64() - self.wavelength.as_secs_f64() / 2.0)
                % self.wavelength.as_secs_f64())
                - self.wavelength.as_secs_f64() / 2.0)
                .abs())
        .min(self.amplitude) // first couple values are out of range for some reason so we clamp them down
    }

    fn duration(&self) -> Duration {
        self.wavelength
    }
}

/// Generates a Square wave between 0 and an amplitude for a given duration.
///
/// Waves are generated as single pulses with a given wavelength.
/// To play several cycles of the wave in sequence, use the `repeat` method.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SquareWave {
    amplitude: f64,
    wavelength: Duration,
}

impl SquareWave {
    pub fn new(amplitude: f64, wavelength: Duration) -> Self {
        SquareWave {
            amplitude,
            wavelength,
        }
    }
}

impl PatternGenerator for SquareWave {
    fn sample(&mut self, time: Duration) -> f64 {
        if time.as_secs_f64() % self.wavelength.as_secs_f64() < self.wavelength.as_secs_f64() / 2.0
        {
            self.amplitude
        } else {
            0.0
        }
    }

    fn duration(&self) -> Duration {
        self.wavelength
    }
}

/// Generates a Sine wave between 0 and an amplitude for a given duration.
///
/// Waves are generated as single pulses with a given wavelength.
/// To play several cycles of the wave in sequence, use the `repeat` method.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SineWave {
    amplitude: f64,
    wavelength: Duration,
}

impl SineWave {
    pub fn new(amplitude: f64, wavelength: Duration) -> Self {
        SineWave {
            amplitude,
            wavelength,
        }
    }
}

impl PatternGenerator for SineWave {
    fn sample(&mut self, time: Duration) -> f64 {
        // sine value between 0 and `amplitude` based on a wavelength of `wavelength` starting at 0
        (self.amplitude / 2.0)
            * f64::cos(
                2.0 * PI
                    * (1.0 / self.wavelength.as_secs_f64())
                    * (time.as_secs_f64() + self.wavelength.as_secs_f64() / 2.0),
            )
            + self.amplitude / 2.0
    }

    fn duration(&self) -> Duration {
        self.wavelength
    }
}
