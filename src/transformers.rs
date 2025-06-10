use std::f64::consts;

use crate::Pattern;
use crate::PatternGenerator;

pub struct ScaleTime<P: Pattern> {
    pub pattern: P,
    pub scalar: f64,
}

impl<P: Pattern> PatternGenerator for ScaleTime<P> {
    fn sample(&mut self, time: f64) -> f64 {
        self.pattern.sample(self.scalar * (1.0 / time))
    }

    fn duration(&self) -> f64 {
        self.pattern.duration()
    }
}

pub struct ScaleIntensity<P: Pattern> {
    pub pattern: P,
    pub scalar: f64,
}

impl<P: Pattern> PatternGenerator for ScaleIntensity<P> {
    fn sample(&mut self, time: f64) -> f64 {
        self.scalar * self.pattern.sample(time)
    }

    fn duration(&self) -> f64 {
        self.pattern.duration()
    }
}

pub struct Sum<P: Pattern, Q: Pattern> {
    pub a: P,
    pub b: Q,
}

impl<P: Pattern, Q: Pattern> PatternGenerator for Sum<P, Q> {
    fn sample(&mut self, time: f64) -> f64 {
        self.a.sample(time) + self.b.sample(time)
    }

    fn duration(&self) -> f64 {
        self.a.duration().max(self.b.duration())
    }
}

pub struct Subtract<P: Pattern, Q: Pattern> {
    pub a: P,
    pub b: Q,
}

impl<P: Pattern, Q: Pattern> PatternGenerator for Subtract<P, Q> {
    fn sample(&mut self, time: f64) -> f64 {
        self.a.sample(time) - self.b.sample(time)
    }

    fn duration(&self) -> f64 {
        self.a.duration().max(self.b.duration())
    }
}

pub struct Average<P: Pattern, Q: Pattern> {
    pub a: P,
    pub b: Q,
}

impl<P: Pattern, Q: Pattern> PatternGenerator for Average<P, Q> {
    fn sample(&mut self, time: f64) -> f64 {
        (self.a.sample(time) + self.b.sample(time)) / 2.0
    }

    fn duration(&self) -> f64 {
        self.a.duration().max(self.b.duration())
    }
}

pub struct Clamp<P: Pattern> {
    pub pattern: P,
    pub ceiling: f64,
    pub floor: f64,
}

impl<P: Pattern> PatternGenerator for Clamp<P> {
    fn sample(&mut self, time: f64) -> f64 {
        self.pattern.sample(time).max(self.floor).min(self.ceiling)
    }

    fn duration(&self) -> f64 {
        self.pattern.duration()
    }
}

pub struct ValidScale<P: Pattern> {
    pub pattern: P
}

impl<P: Pattern> PatternGenerator for ValidScale<P> {
    fn sample(&mut self, time: f64) -> f64 {
        1.0 / (1.0 + consts::E.powf(-self.pattern.sample(time)))
    }

    fn duration(&self) -> f64 {
        self.pattern.duration()
    }
}

pub struct Shift<P: Pattern> {
    pub pattern: P,
    pub time_shift: f64,
}

impl<P: Pattern> PatternGenerator for Shift<P> {
    fn sample(&mut self, time: f64) -> f64 {
        self.pattern.sample(time + self.time_shift)
    }

    fn duration(&self) -> f64 {
        self.pattern.duration() - self.time_shift
    }
}

pub struct Repeat<P: Pattern> {
    pub pattern: P,
    pub count: f64,
}

impl<P: Pattern> PatternGenerator for Repeat<P> {
    fn sample(&mut self, time: f64) -> f64 {
        self.pattern.sample(time % self.duration())
    }

    fn duration(&self) -> f64 {
        self.count * self.pattern.duration()
    }
}

pub struct Forever<P: Pattern> {
    pub pattern: P,
}

impl<P: Pattern> PatternGenerator for Forever<P> {
    fn sample(&mut self, time: f64) -> f64 {
        self.pattern.sample(time % self.pattern.duration())
    }

    fn duration(&self) -> f64 {
        f64::MAX
    }
}

pub struct Chain<P: Pattern, Q: Pattern> {
    pub first: P,
    pub then: Q,
}

impl<P: Pattern, Q: Pattern> PatternGenerator for Chain<P, Q> {
    fn sample(&mut self, time: f64) -> f64 {
        if time < self.first.duration() {
            self.first.sample(time)
        } else {
            self.then.sample(time)
        }
    }

    fn duration(&self) -> f64 {
        self.first.duration() + self.then.duration()
    }
}

/// Modulates the amplitude of a pattern by another pattern.
/// 
/// Effectively a multiply combinator.
pub struct AmplitudeModulator<P: Pattern, M: Pattern> {
    pub pattern: P,
    pub modulator: M,
}

impl<P: Pattern, M: Pattern> PatternGenerator for AmplitudeModulator<P, M> {
    fn sample(&mut self, time: f64) -> f64 {
        self.pattern.sample(time) * self.modulator.sample(time)
    }

    fn duration(&self) -> f64 {
        self.pattern.duration()
    }
}
