extern crate redpitaya;

use redpitaya::Channel;
use redpitaya::acquire::{
    Decimation,
    TrigSrc,
    TrigState,
};
use redpitaya::generator::Waveform;

fn main() {
    redpitaya::init()
        .expect("Rp api init failed!");

    redpitaya::generator::reset().unwrap();
    redpitaya::generator::freq(Channel::RP_CH_1, 20_000.0).unwrap();
    redpitaya::generator::amp(Channel::RP_CH_1, 1.0).unwrap();
    redpitaya::generator::waveform(Channel::RP_CH_1, Waveform::RP_WAVEFORM_SINE).unwrap();
    redpitaya::generator::out_enable(Channel::RP_CH_1).unwrap();

    redpitaya::acquire::reset().unwrap();
    redpitaya::acquire::set_decimation(Decimation::RP_DEC_1).unwrap();
    redpitaya::acquire::set_trigger_level(Channel::RP_CH_1, 0.1).unwrap();
    redpitaya::acquire::set_trigger_delay(0).unwrap();

    redpitaya::acquire::start().unwrap();

    std::thread::sleep(std::time::Duration::new(1, 0));

    redpitaya::acquire::set_trigger_src(TrigSrc::RP_TRIG_SRC_CHA_PE).unwrap();

    loop {
        match redpitaya::acquire::get_trigger_state() {
            Ok(state) => if state == TrigState::RP_TRIG_STATE_TRIGGERED {
                break;
            },
            Err(err) => panic!("{}", err),
        };
    }

    println!("{:?}", redpitaya::acquire::get_oldest_data_v(Channel::RP_CH_1, 16_384));

    redpitaya::release()
        .unwrap();
}
