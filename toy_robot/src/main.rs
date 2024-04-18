use frc::{
    pneumatics::{
        ctre::{CtreDoubleSolenoid, CtrePcm, CtrePneumatics},
        rev::{RevDoubleSolenoid, RevPh, RevPneumatics},
        DoubleSolenoid, DoubleSolenoidState
    },
    reactor::driver_station::DriverStation,
};

#[tokio::main]
async fn main() {
    let mut ctre_pneumatics = CtrePcm::default();
    let (
        _ctre_compressor,
        CtrePneumatics {
            channel0: ctre0,
            channel1: ctre1,
            channel2: ctre2,
            channel3: ctre3,
            ..
        },
    ) = ctre_pneumatics.as_parts();

    let mut rev_pneumatics = RevPh::default();
    let (
        _rev_compressor,
        RevPneumatics {
            channel0: rev0,
            channel1: rev1,
            ..
        }
    ) = rev_pneumatics.as_parts();

    let mut ctre_double_solenoid = CtreDoubleSolenoid::new(ctre0, ctre1);
    let mut ctre_double_solenoid2 = CtreDoubleSolenoid::new(ctre2, ctre3);
    let mut rev_double_solenoid = RevDoubleSolenoid::new(rev0, rev1);
    let mut double_solenoid_array = [ctre_double_solenoid.erase_all(), ctre_double_solenoid2.erase_all(), rev_double_solenoid.erase_all()];

    let ds = DriverStation::new();

    loop {
        if let Some(controller_state) = ds.get_controller_state(0) {
            for (i, solenoid) in double_solenoid_array.iter_mut().enumerate() {
                if let Some(button) = controller_state.button(i.try_into().unwrap()) {
                    solenoid.set(if button {
                        DoubleSolenoidState::Forward
                    } else {
                        DoubleSolenoidState::Backward
                    });
                }
            }
        }
        ds.wait_for_packet().await;
    }
}
