pub mod driver;
pub mod shape;

/// Represents a pattern to be used to actuate buttplug devices.
pub trait PatternGenerator {
    /// Gives an intensity value for a given time.
    /// 
    /// Behavior when sampling a pattern for a time past it's duration is not specified.
    /// Some patterns will return valid values for any time, but you should use the 
    /// `.repeat()`, `.forever()`, and `.chain()` methods of `Pattern` for extending Patterns
    fn sample(&self, time: f64) -> f64;

    /// how long a cycle of the pattern takes in seconds
    fn duration(&self) -> f64;
}

impl<T: PatternGenerator> Pattern for T {}

/// Extension trait for `PatternGenerator`, contains methods for building and transforming
/// `Pattern`s,
/// 
/// Patterns can be done with the `Driver` type 
pub trait Pattern: PatternGenerator + Sized {
    /// Scales the pattern in the time domain by a given `scalar`.
    ///
    /// For example, a scalar of 2.0 would double the length of cycles.
    /// This would turn a sine wave of wavelength 0.5 seconds into a square wave of wavelength 1.0 seconds.
    fn scale_time(self, scalar: f64) -> ScaleTime<Self> {
        ScaleTime {
            pattern: self,
            scalar,
        }
    }

    /// Scales the pattern in the intensity domain by a given `scalar`.
    ///
    /// For example, a scalar of 2.0 would double the intensity of the pattern.
    /// This would turn a sine wave of amplitude 0.5 into a square wave of amplitude 1.0.
    fn scale_intensity(self, scalar: f64) -> ScaleIntensity<Self> {
        ScaleIntensity {
            pattern: self,
            scalar,
        }
    }

    /// Takes the sum of two patterns.
    ///
    /// For example, a sine wave of amplitude 0.5 and a square wave of amplitude 0.5 would sum to a sine wave of amplitude 1.0.
    fn sum<Q: Pattern>(self, other: Q) -> Sum<Self, Q> {
        Sum { a: self, b: other }
    }

    /// Takes the difference of two patterns.
    ///
    /// For example, a sine wave of amplitude 0.75 and a square wave of amplitude 0.25 would subtract to a sine wave of amplitude 0.5.
    fn subtract<Q: Pattern>(self, other: Q) -> Subtract<Self, Q> {
        Subtract { a: self, b: other }
    }

    /// Takes the average of two patterns.
    ///
    /// For example, a sine wave of amplitude 0.75 and a square wave of amplitude 0.25 would average to a sine wave of amplitude 0.5.
    fn average<Q: Pattern>(self, other: Q) -> Average<Self, Q> {
        Average { a: self, b: other }
    }

    /// Clamps the pattern to a given range.
    ///
    /// This is useful for limiting the output of a pattern to a certain range.
    /// Or making waves that clip at the edges of the range.
    fn clamp(self, floor: f64, ceiling: f64) -> Clamp<Self> {
        Clamp {
            pattern: self,
            floor,
            ceiling,
        }
    }

    /// Clamps the pattern to a valid scalar value for a buttplug command.
    /// This is in the range of 0.0 to 1.0.
    fn clamp_valid(self) -> Clamp<Self> {
        self.clamp(0.0, 1.0)
    }

    fn shift(self, time_shift: f64) -> Shift<Self> {
        Shift {
            pattern: self,
            time_shift,
        }
    }

    fn repeat(self, count: f64) -> Repeat<Self> {
        Repeat {
            pattern: self,
            count,
        }
    }

    fn forever(self) -> Forever<Self> {
        Forever { pattern: self }
    }

    fn chain<Q: Pattern>(self, other: Q) -> Chain<Self, Q> {
        Chain {
            first: self,
            then: other,
        }
    }
}

pub struct ScaleTime<P: Pattern> {
    pattern: P,
    scalar: f64,
}

impl<P: Pattern> PatternGenerator for ScaleTime<P> {
    fn sample(&self, time: f64) -> f64 {
        self.pattern.sample(self.scalar * (1.0 / time))
    }

    fn duration(&self) -> f64 {
        self.pattern.duration()
    }
}

pub struct ScaleIntensity<P: Pattern> {
    pattern: P,
    scalar: f64,
}

impl<P: Pattern> PatternGenerator for ScaleIntensity<P> {
    fn sample(&self, time: f64) -> f64 {
        self.scalar * self.pattern.sample(time)
    }

    fn duration(&self) -> f64 {
        self.pattern.duration()
    }
}

pub struct Sum<P: Pattern, Q: Pattern> {
    a: P,
    b: Q,
}

impl<P: Pattern, Q: Pattern> PatternGenerator for Sum<P, Q> {
    fn sample(&self, time: f64) -> f64 {
        self.a.sample(time) + self.b.sample(time)
    }

    fn duration(&self) -> f64 {
        self.a.duration().max(self.b.duration())
    }
}

pub struct Subtract<P: Pattern, Q: Pattern> {
    a: P,
    b: Q,
}

impl<P: Pattern, Q: Pattern> PatternGenerator for Subtract<P, Q> {
    fn sample(&self, time: f64) -> f64 {
        self.a.sample(time) - self.b.sample(time)
    }

    fn duration(&self) -> f64 {
        self.a.duration().max(self.b.duration())
    }
}

pub struct Average<P: Pattern, Q: Pattern> {
    a: P,
    b: Q,
}

impl<P: Pattern, Q: Pattern> PatternGenerator for Average<P, Q> {
    fn sample(&self, time: f64) -> f64 {
        (self.a.sample(time) + self.b.sample(time)) / 2.0
    }

    fn duration(&self) -> f64 {
        self.a.duration().max(self.b.duration())
    }
}

pub struct Clamp<P: Pattern> {
    pattern: P,
    ceiling: f64,
    floor: f64,
}

impl<P: Pattern> PatternGenerator for Clamp<P> {
    fn sample(&self, time: f64) -> f64 {
        self.pattern.sample(time).max(self.floor).min(self.ceiling)
    }

    fn duration(&self) -> f64 {
        self.pattern.duration()
    }
}

pub struct Shift<P: Pattern> {
    pattern: P,
    time_shift: f64,
}

impl<P: Pattern> PatternGenerator for Shift<P> {
    fn sample(&self, time: f64) -> f64 {
        self.pattern.sample(time + self.time_shift)
    }

    fn duration(&self) -> f64 {
        self.pattern.duration()
    }
}

pub struct Repeat<P: Pattern> {
    pattern: P,
    count: f64,
}

impl<P: Pattern> PatternGenerator for Repeat<P> {
    fn sample(&self, time: f64) -> f64 {
        self.pattern.sample(time % self.duration())
    }

    fn duration(&self) -> f64 {
        self.count * self.pattern.duration()
    }
}

pub struct Forever<P: Pattern> {
    pattern: P,
}

impl<P: Pattern> PatternGenerator for Forever<P> {
    fn sample(&self, time: f64) -> f64 {
        self.pattern.sample(time % self.pattern.duration())
    }

    fn duration(&self) -> f64 {
        f64::MAX
    }
}

pub struct Chain<P: Pattern, Q: Pattern> {
    first: P,
    then: Q,
}

impl<P: Pattern, Q: Pattern> PatternGenerator for Chain<P, Q> {
    fn sample(&self, time: f64) -> f64 {
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
