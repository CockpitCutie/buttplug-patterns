/// Patterns that generate random values.
pub mod random;
/// Patterns that generate basic shapes and waves.
pub mod shapes;
/// Patterns that transform other patterns.
/// 
/// Note: most transformers should not be used directly, but through methods on the `Pattern` trait.
pub mod transformers;

use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use buttplug::client::{ButtplugClient, ButtplugClientError, ScalarValueCommand};
use tokio::time::interval;

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

/// Driver that can send patterns to buttplug devices.
pub struct Driver {
    pub buttplug: Arc<ButtplugClient>,
    tickrate_hz: u64,
    pattern: Box<dyn PatternGenerator>,
    device_patterns: HashMap<u32, Box<dyn PatternGenerator>>,
    actuator_patterns: HashMap<(u32, u32), Box<dyn PatternGenerator>>,
}

impl Driver {
    /// Creates a new driver with a given ButtplugClient and Pattern.
    ///
    /// The ButtplugClient is passed via an Arc to allow for applications to maintain access to the client
    /// after the driver has been created.
    pub fn new<P: 'static + Pattern>(bp: Arc<ButtplugClient>, pattern: P) -> Self {
        Driver {
            buttplug: bp,
            tickrate_hz: 10, // 10 hz is fast enough to feel smooth without overwhelming the device or server in my testing
            pattern: Box::new(pattern),
            device_patterns: HashMap::new(),
            actuator_patterns: HashMap::new(),
        }
    }

    /// Sets the tickrate of the driver, in Hz. The tickrate is the number of times per second
    /// that the driver samples the pattern and sends the new intensity to the device.
    ///
    /// The default tickrate is 10 Hz.
    pub fn set_tickrate(&mut self, hz: u64) -> &mut Self {
        self.tickrate_hz = hz;
        self
    }

    /// Sets the global pattern of the driver.
    /// This pattern is applied to all actuators on all devices that do not have a more specific pattern.
    pub fn set_pattern<P: 'static + PatternGenerator>(&mut self, pattern: P) -> &mut Self {
        self.pattern = Box::new(pattern);
        self
    }

    /// Sets the pattern of a specific device based on its index.
    ///
    /// Device indexes can be found using the `index()` method of the `ButtplugClientDevice`.
    pub fn set_device_pattern<P: 'static + PatternGenerator>(
        &mut self,
        device_id: u32,
        pattern: P,
    ) -> &mut Self {
        self.device_patterns.insert(device_id, Box::new(pattern));
        self
    }

    /// Removes the pattern of a specific device based on its ID.
    pub fn remove_device_pattern(&mut self, device_id: u32) -> &mut Self {
        self.device_patterns.remove(&device_id);
        self
    }

    /// Sets the pattern of a specific actuator based on its device ID and actuator ID.
    pub fn set_actuator_pattern<P: 'static + PatternGenerator>(
        &mut self,
        device_id: u32,
        actuator_id: u32,
        pattern: P,
    ) -> &mut Self {
        self.actuator_patterns
            .insert((device_id, actuator_id), Box::new(pattern));
        self
    }

    /// Removes the pattern of a specific actuator based on its device and actuator ID.
    pub fn remove_actuator_pattern(&mut self, device_id: u32, actuator_id: u32) -> &mut Self {
        self.actuator_patterns.remove(&(device_id, actuator_id));
        self
    }

    /// Runs the driver, actuating all connected devices with the current pattern. All devices will stop when `run` exits.
    pub async fn run(&mut self) -> Result<(), ButtplugClientError> {
        self.run_while(AtomicBool::new(true)).await
    }

    /// Runs the driver, actuating all connected devices with the current pattern, while the `running` is true.
    ///
    /// This is useful for when you want to cancel the driver early. All devices will stop when `run_while` exits.
    pub async fn run_while(&mut self, running: AtomicBool) -> Result<(), ButtplugClientError> {
        self.pattern.reset();
        self.device_patterns.values_mut().for_each(|p| p.reset());
        self.actuator_patterns.values_mut().for_each(|p| p.reset());
        let start = Instant::now();
        let mut interval = interval(Duration::from_millis(1000 / self.tickrate_hz));
        while running.load(Ordering::Acquire) {
            let elapsed = start.elapsed();
            if elapsed > self.pattern.duration() {
                break;
            }

            let global_intensity = self.pattern.sample(elapsed);
            for device in self.buttplug.devices() {
                let mut actuator_map: HashMap<u32, f64> = HashMap::new();
                for actuator in device.vibrate_attributes() {
                    // vibrate attributes returns a vec of actuator info
                    let level = self
                        .actuator_patterns
                        .get_mut(&(device.index(), *actuator.index()))
                        .map(|p| p.sample(elapsed))
                        .unwrap_or(
                            self.device_patterns
                                .get_mut(&device.index())
                                .map(|p| p.sample(elapsed))
                                .unwrap_or(global_intensity),
                        );
                    actuator_map.insert(*actuator.index(), level);
                }
                device
                    .vibrate(&ScalarValueCommand::ScalarValueMap(actuator_map))
                    .await?;
            }
            interval.tick().await;
        }
        self.buttplug.stop_all_devices().await
    }
}
