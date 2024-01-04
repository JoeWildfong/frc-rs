use frc::{
    pneumatics::{
        ctre::{CtrePcm, CtrePneumatics},
        DoubleSolenoid, DoubleSolenoidState, Solenoid,
    },
    reactor::driver_station::DriverStation,
};

#[tokio::main]
async fn main() {
    let mut pneumatics = CtrePcm::default();
    let (
        _compressor,
        CtrePneumatics {
            channel0,
            channel1,
            channel5,
            ..
        },
    ) = pneumatics.as_parts();

    let mut double_solenoid = DoubleSolenoid::new(channel0, channel1);
    let mut single_solenoid = Solenoid::new(channel5);

    let ds = DriverStation::new();

    loop {
        if let Some(controller_state) = ds.get_controller_state(0) {
            let button0 = controller_state.button(0).expect("no button 0?");
            double_solenoid.set(if button0 {
                DoubleSolenoidState::Forward
            } else {
                DoubleSolenoidState::Backward
            });
            let button1 = controller_state.button(1).expect("no button 1?");
            single_solenoid.set(button1);
        }
        ds.wait_for_packet().await;
    }
}
