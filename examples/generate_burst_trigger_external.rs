use redpitaya::Channel;
use redpitaya::generator::{
    Mode,
    TrigSrc,
    Waveform,
};

fn main() -> Result<(), String> {
    redpitaya::init()?;

    redpitaya::generator::waveform(Channel::RP_CH_1, Waveform::RP_WAVEFORM_SINE)?;
    redpitaya::generator::freq(Channel::RP_CH_1, 200.0)?;
    redpitaya::generator::amp(Channel::RP_CH_1, 1.0)?;

    redpitaya::generator::burst_count(Channel::RP_CH_1, 1)?;

    redpitaya::generator::out_enable(Channel::RP_CH_1)?;
    redpitaya::generator::mode(Channel::RP_CH_1, Mode::RP_GEN_MODE_BURST)?;
    redpitaya::generator::trigger_source(Channel::RP_CH_1, TrigSrc::RP_GEN_TRIG_SRC_EXT_PE)?;

    redpitaya::release()
}
