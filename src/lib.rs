/// Driver to run a pattern on a buttplug device
pub mod driver;
/// Patterns that generate random values.
pub mod random;
/// Patterns that generate basic shapes and waves.
pub mod shapes;
/// Patterns that transform other patterns.
///
/// Note: most transformers should not be used directly, but through methods on the `Pattern` trait.
pub mod transformers;

pub use driver::Driver;

use std::time::Duration;

use transformers::*;

/// Represents a pattern to be used to actuate buttplug devices.
pub trait PatternGenerator {
    /// Gives an intensity value for a given time.
    ///
    /// Behavior when sampling a pattern for a time past it's duration is not specified.
    /// Some patterns will return valid values for any time, but you should use the
    /// `.repeat()`, `.forever()`, and `.chain()` methods of `Pattern` for extending Patterns
    fn sample(&mut self, time: Duration) -> f64;

    /// how long a cycle of the pattern takes in seconds
    fn duration(&self) -> Duration;

    /// Resets the pattern to its initial state if it is stateful.
    /// if the pattern is stateless, this method does nothing.
    fn reset(&mut self) {}
}

impl<T: PatternGenerator> Pattern for T {}

/// Extension trait for `PatternGenerator`, contains methods for building and transforming
/// `Pattern`s,
///
/// Patterns can be run on a device using a `Driver`
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

    /// Scales a pattern to a valid range for a buttplug command.
    ///
    /// Scaling is performed by a sigmoid function 1/(1+e^(-x)).
    fn scale_valid(self) -> ValidScale<Self> {
        ValidScale { pattern: self }
    }

    // Time shifts a pattern by `time_shift` seconds, can be used to skip a portion of a pattern
    fn shift(self, time_shift: Duration) -> Shift<Self> {
        Shift {
            pattern: self,
            time_shift,
        }
    }

    /// Repeats a pattern `count` times, fractional repeats are supported, so `pattern.repeat(1.5)` is valid
    fn repeat(self, count: f64) -> Repeat<Self> {
        Repeat {
            pattern: self,
            count,
        }
    }

    /// Loops a pattern forever
    fn forever(self) -> Forever<Self> {
        Forever { pattern: self }
    }

    /// Chains two patterns together, `other` is run after `self`'s duration.
    fn chain<Q: Pattern>(self, other: Q) -> Chain<Self, Q> {
        Chain {
            first: self,
            then: other,
        }
    }

    /// Chains two patterns together with a linear crossfade between them.
    fn crossfade<Q: Pattern>(self, other: Q, overlap: Duration) -> Crossfade<Self, Q> {
        Crossfade {
            first: self,
            then: other,
            overlap_duration: overlap,
        }
    }

    /// Modulates the amplitude of the pattern by another pattern.
    fn multiply<M: Pattern>(self, modulator: M) -> AmplitudeModulator<Self, M> {
        AmplitudeModulator {
            pattern: self,
            modulator,
        }
    }
}

/// Can be used to make simple custom patterns.
///
/// This is useful for when you want to create a pattern that is not supported by the library.
/// To implement more complex patterns, consider making a type that implements the `PatternGenerator` trait.
pub struct CustomPattern {
    pub sample: fn(Duration) -> f64,
    pub duration: fn() -> Duration,
}

impl PatternGenerator for CustomPattern {
    fn sample(&mut self, time: Duration) -> f64 {
        (self.sample)(time)
    }

    fn duration(&self) -> Duration {
        (self.duration)()
    }
}
