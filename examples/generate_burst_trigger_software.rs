use redpitaya::generator::{Mode, Waveform};
use redpitaya::Channel;

fn main() -> redpitaya::Result {
    redpitaya::init().expect("Red Pitaya API init failed!");

    redpitaya::generator::set_waveform(Channel::RP_CH_1, Waveform::RP_WAVEFORM_SINE)?;
    redpitaya::generator::set_freq(Channel::RP_CH_1, 200.0)?;
    redpitaya::generator::set_amp(Channel::RP_CH_1, 1.0)?;

    redpitaya::generator::set_mode(Channel::RP_CH_1, Mode::RP_GEN_MODE_BURST)?;
    redpitaya::generator::set_burst_count(Channel::RP_CH_1, 1)?;
    redpitaya::generator::set_burst_repetitions(Channel::RP_CH_1, 10_000)?;
    redpitaya::generator::set_burst_period(Channel::RP_CH_1, 5_000)?;
    redpitaya::generator::set_trigger(Channel::RP_CH_1)?;

    std::thread::sleep(std::time::Duration::new(1, 0));

    redpitaya::generator::out_enable(Channel::RP_CH_1)?;

    redpitaya::release()
}
