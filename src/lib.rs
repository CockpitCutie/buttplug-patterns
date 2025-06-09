pub mod shapes;
pub mod transformers;

use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

use buttplug::client::{ButtplugClient, ScalarValueCommand};
use tokio::time::interval;

use transformers::*;

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

    /// Scales a pattern to a valid range for a buttplug command.
    /// 
    /// Scaling is performed by a sigmoid function 1/(1+e^(-x)).
    fn scale_valid(self) -> ValidScale<Self> {
        ValidScale { pattern: self }
    }

    // Time shifts a pattern by `time_shift` seconds, can be used to skip a portion of a pattern
    fn shift(self, time_shift: f64) -> Shift<Self> {
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
}

pub struct Driver {
    pub buttplug: ButtplugClient,
    tickrate_hz: u64,
    pattern: Box<dyn PatternGenerator>,
}

impl Driver {
    pub fn new<P: 'static + Pattern>(bp: ButtplugClient, pattern: P) -> Self {
        Driver {
            buttplug: bp,
            tickrate_hz: 10,
            pattern: Box::new(pattern),
        }
    }

    pub fn set_tickrate(&mut self, hz: u64) -> &mut Self {
        self.tickrate_hz = hz;
        self
    }

    pub async fn run(&mut self) {
        let start = Instant::now();
        let mut interval = interval(Duration::from_millis(1000 / self.tickrate_hz));
        loop {
            let elapsed = start.elapsed().as_secs_f64();
            if elapsed > self.pattern.duration() {
                break;
            }

            for device in self.buttplug.devices() {
                let level = self.pattern.sample(elapsed);
                device
                    .vibrate(&ScalarValueCommand::ScalarValue(level))
                    .await
                    .unwrap();
            }

            interval.tick().await;
        }
    }

    pub async fn run_while(&mut self, running: AtomicBool) {
        let mut interval = interval(Duration::from_millis(1000 / self.tickrate_hz));
        let start = Instant::now();
        while running.load(Ordering::Acquire) {
            let elapsed = start.elapsed().as_secs_f64();
            if elapsed > self.pattern.duration() {
                break;
            }

            for device in self.buttplug.devices() {
                let level = self.pattern.sample(elapsed);
                device
                    .vibrate(&ScalarValueCommand::ScalarValue(level))
                    .await
                    .unwrap();
            }

            interval.tick().await;
        }
    }
}
