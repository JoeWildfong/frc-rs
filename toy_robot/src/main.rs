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
            channel2: rev2,
            ..
        },
    ) = rev_pneumatics.as_parts();

    let ctre_double_solenoid = TypedDoubleSolenoid::new(ctre0, ctre1);
    let ctre_double_solenoid2 = TypedDoubleSolenoid::new(ctre2, ctre3);
    let rev_double_solenoid = TypedDoubleSolenoid::new(rev0, rev1);
    let mut double_solenoid_array = [
        ctre_double_solenoid.erase_all(),
        ctre_double_solenoid2.erase_all(),
        rev_double_solenoid.erase_all(),
    ];

    let ds = DriverStation::new();
    let DioPort { dio1, .. } = DioPort::take().unwrap();

    tokio::join!(
        fire_solenoids_on_button(&ds, &mut double_solenoid_array),
        tie_solenoid_to_limit_switch(TypedSolenoid::new(rev2), dio1.into_input())
    );
}

async fn tie_solenoid_to_limit_switch(mut solenoid: impl Solenoid, mut limit_pin: impl Wait) {
    loop {
        limit_pin.wait_for_high().await.unwrap();
        solenoid.set(true);
        limit_pin.wait_for_low().await.unwrap();
        solenoid.set(false);
    }
}

async fn fire_solenoids_on_button(ds: &DriverStation, solenoids: &mut [AnyDoubleSolenoid<'_>]) {
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
