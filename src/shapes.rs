use std::time::Duration;

use crate::PatternGenerator;

/// Generates a constant value for a given duration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constant {
    level: f64,
    duration: Duration,
}

impl Constant {
    pub fn new(level: f64, duration: Duration) -> Self {
        return Constant { level, duration };
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
    wavelength_secs: Duration,
}

impl SawWave {
    pub fn new(amplitude: f64, wavelength_secs: Duration) -> Self {
        SawWave {
            amplitude,
            wavelength_secs,
        }
    }
}

impl PatternGenerator for SawWave {
    fn sample(&mut self, time: Duration) -> f64 {
        self.amplitude * (1.0 / self.wavelength_secs.as_secs_f64()) * time.as_secs_f64() % 1.0
    }

    fn duration(&self) -> Duration {
        self.wavelength_secs
    }
}

/// Generates a Triangle wave between 0 and an amplitude for a given duration.
///
/// Waves are generated as single pulses with a given wavelength.
/// To play several cycles of the wave in sequence, use the `repeat` method.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TriangleWave {
    amplitude: f64,
    wavelength_secs: Duration,
}

impl TriangleWave {
    pub fn new(amplitude: f64, wavelength_secs: Duration) -> Self {
        TriangleWave {
            amplitude,
            wavelength_secs,
        }
    }
}

impl PatternGenerator for TriangleWave {
    fn sample(&mut self, time: Duration) -> f64 {
        // Formula for a triangle wave between 0 and `amplitude` with period `wavelength_secs`
        // https://en.wikipedia.org/wiki/Triangle_wave#Definition
        ((2.0 * self.amplitude / self.wavelength_secs.as_secs_f64())
            * (((time.as_secs_f64() - self.wavelength_secs.as_secs_f64() / 2.0)
                % self.wavelength_secs.as_secs_f64())
                - self.wavelength_secs.as_secs_f64() / 2.0)
                .abs())
        .min(self.amplitude) // first couple values are out of range for some reason so we clamp them down
    }

    fn duration(&self) -> Duration {
        self.wavelength_secs
    }
}

/// Generates a Square wave between 0 and an amplitude for a given duration.
///
/// Waves are generated as single pulses with a given wavelength.
/// To play several cycles of the wave in sequence, use the `repeat` method.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SquareWave {
    amplitude: f64,
    wavelength_secs: Duration,
}

impl SquareWave {
    pub fn new(amplitude: f64, wavelength_secs: Duration) -> Self {
        SquareWave {
            amplitude,
            wavelength_secs,
        }
    }
}

impl PatternGenerator for SquareWave {
    fn sample(&mut self, time: Duration) -> f64 {
        if time.as_secs_f64() % self.wavelength_secs.as_secs_f64()
            < self.wavelength_secs.as_secs_f64() / 2.0
        {
            self.amplitude
        } else {
            0.0
        }
    }

    fn duration(&self) -> Duration {
        self.wavelength_secs
    }
}

/// Generates a Sine wave between 0 and an amplitude for a given duration.
///
/// Waves are generated as single pulses with a given wavelength.
/// To play several cycles of the wave in sequence, use the `repeat` method.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SineWave {
    amplitude: f64,
    wavelength_secs: Duration,
}

impl SineWave {
    pub fn new(amplitude: f64, wavelength_secs: Duration) -> Self {
        SineWave {
            amplitude,
            wavelength_secs,
        }
    }
}

impl PatternGenerator for SineWave {
    fn sample(&mut self, time: Duration) -> f64 {
        // sine value between 0 and `amplitude` based on a wavelength of `wavelength_secs` starting at 0
        (self.amplitude / 2.0)
            * f64::cos(
                2.0 * 3.14
                    * (1.0 / self.wavelength_secs.as_secs_f64())
                    * (time.as_secs_f64() + self.wavelength_secs.as_secs_f64() / 2.0),
            )
            + self.amplitude / 2.0
    }

    fn duration(&self) -> Duration {
        self.wavelength_secs
    }
}
