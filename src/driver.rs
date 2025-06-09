use std::{sync::atomic::{AtomicBool, Ordering}, time::{Duration, Instant}};

use buttplug::client::{ButtplugClient, ScalarValueCommand};
use tokio::time::{interval};

use crate::{shape::Constant, Pattern, PatternGenerator};

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
