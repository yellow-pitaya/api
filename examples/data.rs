// https://redpitaya.readthedocs.io/en/latest/doc/appsFeatures/examples/acqRF-exm1.html

use redpitaya::{
    acquire,
    Channel,
    generator,
};

fn main() -> Result<(), String> {
    redpitaya::init()?;

    generator::reset()?;
    generator::freq(Channel::RP_CH_1, 20_000.0)?;
    generator::amp(Channel::RP_CH_1, 1.0)?;
    generator::waveform(Channel::RP_CH_1, generator::Waveform::RP_WAVEFORM_SINE)?;
    generator::out_enable(Channel::RP_CH_1)?;

    acquire::reset()?;
    acquire::set_decimation(acquire::Decimation::RP_DEC_1)?;
    acquire::set_trigger_level(Channel::RP_CH_1, 0.1)?;
    acquire::set_trigger_delay(0)?;

    acquire::start()?;

    std::thread::sleep(::std::time::Duration::from_millis(1_000));

    acquire::set_trigger_src(acquire::TrigSrc::RP_TRIG_SRC_CHA_PE)?;

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
}
