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

impl Pattern for SquareWave {
    fn sample(&self, time: f64) -> f64 {
        if time % self.wavelength_secs < self.wavelength_secs / 2.0 {
            self.amplitude
        } else {
            0.0
        }
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

impl Pattern for SineWave {
    fn sample(&self, time: f64) -> f64 {
        // sine value between 0 and `amplitude` based on a wavelength of `wavelength_secs`
        return (self.amplitude / 2.0) * f64::cos(2.0 * 3.14 * (1.0 / self.wavelength_secs) * time)
            + self.amplitude / 2.0;
    }
}
