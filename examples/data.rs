// https://redpitaya.readthedocs.io/en/latest/doc/appsFeatures/examples/acqRF-exm1.html
//
extern crate redpitaya;

use redpitaya::{
    acquire,
    Channel,
    generator,
};

fn main() {
    redpitaya::init()
        .unwrap();

    generator::reset()
        .unwrap();
    generator::freq(Channel::RP_CH_1, 20_000.0)
        .unwrap();
    generator::amp(Channel::RP_CH_1, 1.0)
        .unwrap();
    generator::waveform(Channel::RP_CH_1, generator::Waveform::RP_WAVEFORM_SINE)
        .unwrap();
    generator::out_enable(Channel::RP_CH_1)
        .unwrap();

    acquire::reset()
        .unwrap();
    acquire::set_decimation(acquire::Decimation::RP_DEC_1)
        .unwrap();
    acquire::set_trigger_level(Channel::RP_CH_1, 0.1)
        .unwrap();
    acquire::set_trigger_delay(0)
        .unwrap();

    acquire::start()
        .unwrap();

    ::std::thread::sleep(::std::time::Duration::from_millis(1_000));

    acquire::set_trigger_src(acquire::TrigSrc::RP_TRIG_SRC_CHA_PE)
        .unwrap();

    loop {
        match acquire::get_trigger_state() {
            Ok(state) => if state == acquire::TrigState::RP_TRIG_STATE_TRIGGERED {
                break;
            },
            Err(err) => panic!("{}", err),
        }
    }

    println!("{:?}", acquire::get_oldest_data_v(Channel::RP_CH_1, 16_384));

    redpitaya::release()
        .unwrap();
}
