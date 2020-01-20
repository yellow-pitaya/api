use redpitaya::Channel;
use redpitaya::generator::{
    Mode,
    Waveform,
};

fn main() -> Result<(), String> {
    redpitaya::init()
        .expect("Red Pitaya API init failed!");

    redpitaya::generator::waveform(Channel::RP_CH_1, Waveform::RP_WAVEFORM_SINE)?;
    redpitaya::generator::freq(Channel::RP_CH_1, 200.0)?;
    redpitaya::generator::amp(Channel::RP_CH_1, 1.0)?;

    redpitaya::generator::mode(Channel::RP_CH_1, Mode::RP_GEN_MODE_BURST)?;
    redpitaya::generator::burst_count(Channel::RP_CH_1, 1)?;
    redpitaya::generator::burst_repetitions(Channel::RP_CH_1, 10_000)?;
    redpitaya::generator::burst_period(Channel::RP_CH_1, 5_000)?;
    redpitaya::generator::trigger(Channel::RP_CH_1)?;

    std::thread::sleep(std::time::Duration::new(1, 0));

    redpitaya::generator::out_enable(Channel::RP_CH_1)?;

    redpitaya::release()
}
