use std::f64::consts;
use std::time::Duration;

use crate::shapes::Linear;
use crate::Pattern;
use crate::PatternGenerator;

/// Scales the pattern in the time domain by a given scalar.
#[derive(Clone, Debug, PartialEq)]
pub struct ScaleTime<P: Pattern> {
    pub pattern: P,
    pub scalar: f64,
}

impl<P: Pattern> PatternGenerator for ScaleTime<P> {
    fn sample(&mut self, time: Duration) -> f64 {
        self.pattern.sample(Duration::from_secs_f64(
            self.scalar * (1.0 / time.as_secs_f64()),
        ))
    }

    fn duration(&self) -> Duration {
        self.pattern.duration()
    }
}

/// Scales the pattern in the intensity domain by a given scalar.
#[derive(Clone, Debug, PartialEq)]
pub struct ScaleIntensity<P: Pattern> {
    pub pattern: P,
    pub scalar: f64,
}

impl<P: Pattern> PatternGenerator for ScaleIntensity<P> {
    fn sample(&mut self, time: Duration) -> f64 {
        self.scalar * self.pattern.sample(time)
    }

    fn duration(&self) -> Duration {
        self.pattern.duration()
    }
}

/// Adds two patterns together.
#[derive(Clone, Debug, PartialEq)]
pub struct Sum<P: Pattern, Q: Pattern> {
    pub a: P,
    pub b: Q,
}

impl<P: Pattern, Q: Pattern> PatternGenerator for Sum<P, Q> {
    fn sample(&mut self, time: Duration) -> f64 {
        self.a.sample(time) + self.b.sample(time)
    }

    fn duration(&self) -> Duration {
        self.a.duration().max(self.b.duration())
    }
}

/// Subtracts two patterns from each other.
#[derive(Clone, Debug, PartialEq)]
pub struct Subtract<P: Pattern, Q: Pattern> {
    pub a: P,
    pub b: Q,
}

impl<P: Pattern, Q: Pattern> PatternGenerator for Subtract<P, Q> {
    fn sample(&mut self, time: Duration) -> f64 {
        self.a.sample(time) - self.b.sample(time)
    }

    fn duration(&self) -> Duration {
        self.a.duration().max(self.b.duration())
    }
}

/// Averages two patterns together.
#[derive(Clone, Debug, PartialEq)]
pub struct Average<P: Pattern, Q: Pattern> {
    pub a: P,
    pub b: Q,
}

impl<P: Pattern, Q: Pattern> PatternGenerator for Average<P, Q> {
    fn sample(&mut self, time: Duration) -> f64 {
        (self.a.sample(time) + self.b.sample(time)) / 2.0
    }

    fn duration(&self) -> Duration {
        self.a.duration().max(self.b.duration())
    }
}

/// Clamps the pattern to a given range for a buttplug command.
#[derive(Clone, Debug, PartialEq)]
pub struct Clamp<P: Pattern> {
    pub pattern: P,
    pub ceiling: f64,
    pub floor: f64,
}

impl<P: Pattern> PatternGenerator for Clamp<P> {
    fn sample(&mut self, time: Duration) -> f64 {
        self.pattern.sample(time).max(self.floor).min(self.ceiling)
    }

    fn duration(&self) -> Duration {
        self.pattern.duration()
    }
}

/// Scales the pattern to a valid range for a buttplug command.
#[derive(Clone, Debug, PartialEq)]
pub struct ValidScale<P: Pattern> {
    pub pattern: P,
}

impl<P: Pattern> PatternGenerator for ValidScale<P> {
    fn sample(&mut self, time: Duration) -> f64 {
        1.0 / (1.0 + consts::E.powf(-self.pattern.sample(time)))
    }

    fn duration(&self) -> Duration {
        self.pattern.duration()
    }
}

/// Shifts the pattern by a given time.
#[derive(Clone, Debug, PartialEq)]
pub struct Shift<P: Pattern> {
    pub pattern: P,
    pub time_shift: Duration,
}

impl<P: Pattern> PatternGenerator for Shift<P> {
    fn sample(&mut self, time: Duration) -> f64 {
        self.pattern.sample(time + self.time_shift)
    }

    fn duration(&self) -> Duration {
        self.pattern.duration() - self.time_shift
    }
}

/// Repeats a pattern a given number of times.
#[derive(Clone, Debug, PartialEq)]
pub struct Repeat<P: Pattern> {
    pub pattern: P,
    pub count: f64,
}

impl<P: Pattern> PatternGenerator for Repeat<P> {
    fn sample(&mut self, time: Duration) -> f64 {
        self.pattern.sample(Duration::from_secs_f64(
            time.as_secs_f64() % self.duration().as_secs_f64(),
        ))
    }

    fn duration(&self) -> Duration {
        Duration::from_secs_f64(self.count * self.pattern.duration().as_secs_f64())
    }
}

/// Repeats a pattern forever.
#[derive(Clone, Debug, PartialEq)]
pub struct Forever<P: Pattern> {
    pub pattern: P,
}

impl<P: Pattern> PatternGenerator for Forever<P> {
    fn sample(&mut self, time: Duration) -> f64 {
        let time_slice = time.as_secs_f64() % self.pattern.duration().as_secs_f64();
        self.pattern.sample(Duration::from_secs_f64(time_slice))
    }

    fn duration(&self) -> Duration {
        Duration::MAX
    }
}

/// Chains two patterns together.
#[derive(Clone, Debug, PartialEq)]
pub struct Chain<P: Pattern, Q: Pattern> {
    pub first: P,
    pub then: Q,
}

impl<P: Pattern, Q: Pattern> PatternGenerator for Chain<P, Q> {
    fn sample(&mut self, time: Duration) -> f64 {
        if time < self.first.duration() {
            self.first.sample(time)
        } else {
            self.then.sample(time)
        }
    }

    fn duration(&self) -> Duration {
        self.first.duration() + self.then.duration()
    }
}

/// Linear crossfade between two patterns over a given duration.
pub struct Crossfade<P: Pattern, Q: Pattern> {
    pub first: P,
    pub then: Q,
    pub overlap_duration: Duration,
}

impl<P: Pattern, Q: Pattern> Crossfade<P, Q> {
    pub fn new(first: P, then: Q, overlap_duration: Duration) -> Self {
        Self {
            first,
            then,
            overlap_duration,
        }
    }

    fn sample_overlap(&mut self, time: Duration) -> f64 {
        let progress = (time - (self.first.duration() - self.overlap_duration)).as_secs_f64()
            / self.overlap_duration.as_secs_f64();
        self.first.sample(time) * (1.0 - progress) + self.then.sample(time) * progress
    }
}

impl<P: Pattern, Q: Pattern> PatternGenerator for Crossfade<P, Q> {
    fn sample(&mut self, time: Duration) -> f64 {
        if time < self.first.duration() - self.overlap_duration {
            self.first.sample(time)
        } else if time < self.first.duration() {
            self.sample_overlap(time)
        } else {
            self.then.sample(time - self.overlap_duration)
        }
    }
    fn duration(&self) -> Duration {
        self.first.duration() + self.then.duration() - self.overlap_duration
    }
}

/// Modulates the amplitude of a pattern by another pattern.
///
/// Effectively a multiply combinator.
#[derive(Clone, Debug, PartialEq)]
pub struct AmplitudeModulator<P: Pattern, M: Pattern> {
    pub pattern: P,
    pub modulator: M,
}

impl<P: Pattern, M: Pattern> PatternGenerator for AmplitudeModulator<P, M> {
    fn sample(&mut self, time: Duration) -> f64 {
        self.pattern.sample(time) * self.modulator.sample(time)
    }

    fn duration(&self) -> Duration {
        self.pattern.duration()
    }
}
