use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use crate::{Pattern, PatternGenerator};
use buttplug::client::{ButtplugClient, ButtplugClientError, ScalarValueCommand};
use tokio::time::interval;

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
