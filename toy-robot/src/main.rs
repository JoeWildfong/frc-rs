use std::time::Duration;

use frc_lib::pneumatics::{ctre::{CtrePcm, CtrePneumatics}, DoubleSolenoid, Solenoid, DoubleSolenoidState};

#[tokio::main]
async fn main() {
    let mut pneumatics = CtrePcm::default();
    let (_compressor, CtrePneumatics { channel0, channel1, channel5, .. }) = pneumatics.as_parts();

    let mut double_solenoid = DoubleSolenoid::new(channel0, channel1);
    let mut single_solenoid = Solenoid::new(channel5);

    tokio::time::sleep(Duration::from_secs(2)).await;

    double_solenoid.set(DoubleSolenoidState::Backward);
    single_solenoid.set(false);

    tokio::time::sleep(Duration::from_secs(2)).await;

    double_solenoid.set(DoubleSolenoidState::Forward);
    single_solenoid.set(true);

    tokio::time::sleep(Duration::from_secs(2)).await;

    double_solenoid.set(DoubleSolenoidState::Off);
    single_solenoid.set(false);
}
