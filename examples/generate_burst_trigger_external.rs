use redpitaya::Channel;
use redpitaya::generator::{
    Mode,
    TrigSrc,
    Waveform,
};

fn main() -> redpitaya::Result<()> {
    redpitaya::init()?;

    redpitaya::generator::set_waveform(Channel::RP_CH_1, Waveform::RP_WAVEFORM_SINE)?;
    redpitaya::generator::set_freq(Channel::RP_CH_1, 200.0)?;
    redpitaya::generator::set_amp(Channel::RP_CH_1, 1.0)?;

    redpitaya::generator::set_burst_count(Channel::RP_CH_1, 1)?;

    redpitaya::generator::out_enable(Channel::RP_CH_1)?;
    redpitaya::generator::set_mode(Channel::RP_CH_1, Mode::RP_GEN_MODE_BURST)?;
    redpitaya::generator::set_trigger_source(Channel::RP_CH_1, TrigSrc::RP_GEN_TRIG_SRC_EXT_PE)?;

    redpitaya::release()
}
