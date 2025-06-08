pub mod driver;
mod shape;

pub use shape::{Constant, SineWave, SquareWave};

pub trait Pattern: Sized {
    fn sample(&self, time: f64) -> f64;

    fn scale_time(self, scalar: f64) -> ScaleTime<Self> {
        ScaleTime { pattern: self, scalar }
    }

    fn scale_intensity(self, scalar: f64) -> ScaleIntensity<Self> {
        ScaleIntensity { pattern: self, scalar }
    }

    fn sum<Q: Pattern>(self, other: Q) -> Sum<Self, Q> {
        Sum { a: self, b: other }
    }

    fn average<Q: Pattern>(self, other: Q) -> Average<Self, Q> {
        Average { a: self, b: other }
    }

    fn clamp(self, floor: f64, ceiling: f64) -> Clamp<Self> {
        Clamp { pattern: self, floor, ceiling }
    }

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
        self.pattern.sample(self.scalar * time)
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