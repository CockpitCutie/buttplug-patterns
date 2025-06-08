use buttplug::client::{ButtplugClient, LinearCommand, RotateCommand, ScalarCommand};

pub mod driver;
mod shape;

pub use shape::{Constant, SquareWave, SineWave};

pub enum Command {
    Linear(LinearCommand),
    Rotate(RotateCommand),
    Scalar(ScalarCommand),
}

pub trait Pattern: Sized {
    fn sample(&self, time: f64) -> f64;

    fn scale_time(self, scalar: f64) -> ScaleTime {
        todo!()
    }

    fn scale_intensity(self, scalar: f64) -> ScaleIntensity {
        todo!()
    }

    fn sum(self, other: impl Pattern) -> Sum {
        todo!()
    }
}

struct ScaleTime {}

struct ScaleIntensity{}

struct Sum {}
