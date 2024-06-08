use std::sync::OnceLock;

use embedded_hal_async::digital::Wait;
use frc::{
    dio::DioPort,
    pneumatics::{
        ctre_pcm::{CtrePcm, CtrePneumatics},
        rev_ph::{RevPh, RevPneumatics},
        AnyDoubleSolenoid, DoubleSolenoid, DoubleSolenoidState, Solenoid, TypedDoubleSolenoid,
        TypedSolenoid,
    },
    reactor::driver_station::DriverStation,
};

static DS: OnceLock<DriverStation> = OnceLock::new();

#[tokio::main]
async fn main() {
    let ctre_pneumatics = CtrePcm::default();
    let (
        _ctre_compressor,
        CtrePneumatics {
            channel0: ctre0,
            channel1: ctre1,
            channel2: ctre2,
            channel3: ctre3,
            ..
        },
    ) = ctre_pneumatics.into_parts();

    let rev_pneumatics = RevPh::default();
    let (
        _rev_compressor,
        RevPneumatics {
            channel0: rev0,
            channel1: rev1,
            channel2: rev2,
            ..
        },
    ) = rev_pneumatics.into_parts();

    let ctre_double_solenoid = TypedDoubleSolenoid::new(ctre0, ctre1);
    let ctre_double_solenoid2 = TypedDoubleSolenoid::new(ctre2, ctre3);
    let rev_double_solenoid = TypedDoubleSolenoid::new(rev0, rev1);
    let double_solenoid_array = [
        ctre_double_solenoid.erase_all(),
        ctre_double_solenoid2.erase_all(),
        rev_double_solenoid.erase_all(),
    ];

    let ds = DS.get_or_init(DriverStation::new);
    let DioPort { dio1, .. } = DioPort::take().unwrap();

    tokio::spawn(tie_solenoid_to_limit_switch(
        TypedSolenoid::new(rev2),
        dio1.into_input(),
    ));
    tokio::spawn(fire_solenoids_on_button(ds, double_solenoid_array));
    std::future::pending::<()>().await;
}

async fn tie_solenoid_to_limit_switch(mut solenoid: impl Solenoid, mut limit_pin: impl Wait) {
    loop {
        limit_pin.wait_for_high().await.unwrap();
        solenoid.set(true);
        limit_pin.wait_for_low().await.unwrap();
        solenoid.set(false);
    }
}

async fn fire_solenoids_on_button<const N: usize>(
    ds: &'static DriverStation,
    mut solenoids: [AnyDoubleSolenoid; N],
) {
    loop {
        if let Some(controller_state) = ds.get_controller_state(0) {
            for (i, solenoid) in solenoids.iter_mut().enumerate() {
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
