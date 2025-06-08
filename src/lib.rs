pub mod driver;
pub mod shape;

pub trait Pattern: Sized {
    /// Gives an intensity value for a given time.
    fn sample(&self, time: f64) -> f64;

    /// Scales the pattern in the time domain by a given `scalar`.
    /// 
    /// For example, a scalar of 2.0 would double the length of cycles.
    /// This would turn a sine wave of wavelength 0.5 seconds into a square wave of wavelength 1.0 seconds.
    fn scale_time(self, scalar: f64) -> ScaleTime<Self> {
        ScaleTime { pattern: self, scalar }
    }

    /// Scales the pattern in the intensity domain by a given `scalar`.
    /// 
    /// For example, a scalar of 2.0 would double the intensity of the pattern.
    /// This would turn a sine wave of amplitude 0.5 into a square wave of amplitude 1.0.
    fn scale_intensity(self, scalar: f64) -> ScaleIntensity<Self> {
        ScaleIntensity { pattern: self, scalar }
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
        self.sum(other.scale_intensity(-1.0))
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
        Clamp { pattern: self, floor, ceiling }
    }

    /// Clamps the pattern to a valid scalar value for a buttplug command.
    /// This is in the range of 0.0 to 1.0.
    fn clamp_valid(self) -> Clamp<Self> {
        self.clamp(0.0, 1.0)
    }
}

pub struct ScaleTime<P: Pattern> {
    pattern: P,
    scalar: f64,
}

impl<P: Pattern> Pattern for ScaleTime<P> {
    fn sample(&self, time: f64) -> f64 {
        self.pattern.sample(self.scalar * (1.0 / time))
    }
}

pub struct ScaleIntensity<P: Pattern> {
    pattern: P,
    scalar: f64,
}

impl<P: Pattern> Pattern for ScaleIntensity<P> {
    fn sample(&self, time: f64) -> f64 {
        self.scalar * self.pattern.sample(time)
    }
}

pub struct Sum<P: Pattern, Q: Pattern> {
    a: P,
    b: Q,
}

impl<P: Pattern, Q: Pattern> Pattern for Sum<P, Q> {
    fn sample(&self, time: f64) -> f64 {
        self.a.sample(time) + self.b.sample(time)
    }
}

type Subtract<P, Q> = Sum<P, ScaleIntensity<Q>>;

pub struct Average<P: Pattern, Q: Pattern> {
    a: P,
    b: Q
}

impl<P: Pattern, Q: Pattern> Pattern for Average<P, Q> {
    fn sample(&self, time: f64) -> f64 {
        (self.a.sample(time) + self.b.sample(time)) / 2.0
    }
}

pub struct Clamp<P: Pattern> {
    pattern: P,
    ceiling: f64,
    floor: f64,
}

impl<P: Pattern> Pattern for Clamp<P> {
    fn sample(&self, time: f64) -> f64 {
        self.pattern.sample(time).max(self.floor).min(self.ceiling)
    }
}