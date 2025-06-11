# Buttplug Patterns
A composable interface for creating complex vibration patterns for buttplugio devices!

### Creating Patterns
Patterns are created using a builder pattern, there are several primitives
and methods to transform them to create complex patterns.
```rs
use buttplug_patterns::shapes::{SineWave, Constant};

let my_pattern = SineWave::new(1.0, Duration::from_secs_f64(1.0)) // Sine wave from 0.0 to 1.0 over 1 second
    .repeat(2.0) // repeat the sine wave for 2 cycles
    .chain(Constant::new(0.0, Duration::from_secs_f64(1.0))) // pause for 1 second
    .forever() // repeat forever
```

### Running Patterns
Patterns can be run using the `Driver` struct, which will actuate the pattern on connected devices.
```rs
use buttplug::{client::ButtplugClient, core::connector::new_json_ws_client_connector};
use buttplug_patterns::{Driver, SineWave};

let connector = new_json_ws_client_connector("ws://localhost:12345");
let bp = ButtplugClient::new("pattern test");
bp.connect(connector).await.unwrap();
Driver::new(bp, SineWave::new(1.0, Duration::from_secs_f64(1.0)).forever()).run().await;
```
