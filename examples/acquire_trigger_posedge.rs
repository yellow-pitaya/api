use redpitaya::Channel;
use redpitaya::acquire::{
    Decimation,
    trigger::{self, Source, State},
};
use redpitaya::generator::Waveform;

fn main() -> redpitaya::Result<()> {
    redpitaya::init()?;

    redpitaya::generator::reset()?;
    redpitaya::generator::freq(Channel::RP_CH_1, 20_000.0)?;
    redpitaya::generator::amp(Channel::RP_CH_1, 1.0)?;
    redpitaya::generator::waveform(Channel::RP_CH_1, Waveform::RP_WAVEFORM_SINE)?;
    redpitaya::generator::out_enable(Channel::RP_CH_1)?;

    redpitaya::acquire::reset()?;
    redpitaya::acquire::set_decimation(Decimation::RP_DEC_1)?;
    redpitaya::acquire::trigger::set_level(trigger::Channel::RP_T_CH_1, 0.1)?;
    redpitaya::acquire::trigger::set_delay(0)?;

    redpitaya::acquire::start()?;

    std::thread::sleep(std::time::Duration::new(1, 0));

    redpitaya::acquire::trigger::set_source(Source::RP_TRIG_SRC_CHA_PE)?;

    loop {
        match redpitaya::acquire::trigger::get_state() {
            Ok(state) => if state == State::RP_TRIG_STATE_TRIGGERED {
                break;
            },
            Err(err) => panic!("{}", err),
        };
    }

    println!("{:?}", redpitaya::acquire::get_oldest_data_v(Channel::RP_CH_1, 16_384));

    redpitaya::release()
}
