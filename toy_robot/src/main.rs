use frc::{
    pneumatics::{
        ctre::{CtrePcm, CtrePneumatics},
        DoubleSolenoid, DoubleSolenoidState,
    },
    reactor::driver_station::DriverStation,
};

#[tokio::main]
async fn main() {
    let mut pneumatics = CtrePcm::default();
    let (
        _compressor,
        CtrePneumatics {
            channel0, channel1, ..
        },
    ) = pneumatics.as_parts();

    let mut double_solenoid = DoubleSolenoid::new(channel0, channel1);

    let ds = DriverStation::new();

    loop {
        if let Some(controller_state) = ds.get_controller_state(0) {
            if let Some(button0) = controller_state.button(0) {
                double_solenoid.set(if button0 {
                    DoubleSolenoidState::Forward
                } else {
                    DoubleSolenoidState::Backward
                });
            }
        }
        ds.wait_for_packet().await;
    }
}
