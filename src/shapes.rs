use crate::PatternGenerator;

pub struct Constant {
    level: f64,
    duration: f64,
}

impl Constant {
    pub fn new(level: f64, duration: f64) -> Self {
        return Constant { level, duration };
    }
}

impl PatternGenerator for Constant {
    fn sample(&mut self, _time: f64) -> f64 {
        self.level
    }

    fn duration(&self) -> f64 {
        self.duration
    }
}

pub struct Linear {
    from: f64,
    to: f64,
    duration: f64,
}

impl Linear {
    pub fn new(from: f64, to: f64, duration: f64) -> Self {
        Linear { from, to, duration }
    }
}

impl PatternGenerator for Linear {
    fn sample(&mut self, time: f64) -> f64 {
        self.from + (self.to - self.from) * time / self.duration
    }

    fn duration(&self) -> f64 {
        self.duration
    }
}

pub struct SawWave {
    amplitude: f64,
    wavelength_secs: f64,
}

impl SawWave {
    pub fn new(amplitude: f64, wavelength_secs: f64) -> Self {
        SawWave {
            amplitude,
            wavelength_secs,
        }
    }
}

impl PatternGenerator for SawWave {
    fn sample(&mut self, time: f64) -> f64 {
        self.amplitude * (1.0 / self.wavelength_secs) * time % 1.0
    }

    fn duration(&self) -> f64 {
        self.wavelength_secs
    }
}

pub struct TriangleWave {
    amplitude: f64,
    wavelength_secs: f64,
}

impl TriangleWave {
    pub fn new(amplitude: f64, wavelength_secs: f64) -> Self {
        TriangleWave {
            amplitude,
            wavelength_secs,
        }
    }
}

impl PatternGenerator for TriangleWave {
    fn sample(&mut self, time: f64) -> f64 {
        // Formula for a triangle wave between 0 and `amplitude` with period `wavelength_secs`
        // https://en.wikipedia.org/wiki/Triangle_wave#Definition
        ((2.0 * self.amplitude / self.wavelength_secs)
            * (((time - self.wavelength_secs / 2.0) % self.wavelength_secs)
                - self.wavelength_secs / 2.0)
                .abs())
        .min(self.amplitude) // first couple values are out of range for some reason so we clamp them down
    }

    fn duration(&self) -> f64 {
        self.wavelength_secs
    }
}

pub struct SquareWave {
    amplitude: f64,
    wavelength_secs: f64,
}

impl SquareWave {
    pub fn new(amplitude: f64, wavelength_secs: f64) -> Self {
        SquareWave {
            amplitude,
            wavelength_secs,
        }
    }
}

impl PatternGenerator for SquareWave {
    fn sample(&mut self, time: f64) -> f64 {
        if time % self.wavelength_secs < self.wavelength_secs / 2.0 {
            self.amplitude
        } else {
            0.0
        }
    }

    fn duration(&self) -> f64 {
        self.wavelength_secs
    }
}

pub struct SineWave {
    amplitude: f64,
    wavelength_secs: f64,
}

impl SineWave {
    pub fn new(amplitude: f64, wavelength_secs: f64) -> Self {
        SineWave {
            amplitude,
            wavelength_secs,
        }
    }
}

impl PatternGenerator for SineWave {
    fn sample(&mut self, time: f64) -> f64 {
        // sine value between 0 and `amplitude` based on a wavelength of `wavelength_secs` starting at 0
        (self.amplitude / 2.0)
            * f64::cos(
                2.0 * 3.14 * (1.0 / self.wavelength_secs) * (time + self.wavelength_secs / 2.0),
            )
            + self.amplitude / 2.0
    }

    fn duration(&self) -> f64 {
        self.wavelength_secs
    }
}
