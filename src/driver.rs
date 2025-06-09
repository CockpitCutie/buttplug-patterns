use std::time::{Duration, Instant};

use buttplug::client::{ButtplugClient, ScalarValueCommand};

use crate::Pattern;

pub async fn run(bp: ButtplugClient, pattern: impl Pattern) {
    let start = Instant::now();
    while start.elapsed().as_secs_f64() < pattern.duration() {
        for device in bp.devices() {
            let level = pattern.sample(start.elapsed().as_secs_f64());
            device
                .vibrate(&ScalarValueCommand::ScalarValue(level))
                .await
                .unwrap();
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}
