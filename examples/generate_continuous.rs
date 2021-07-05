use redpitaya::generator::Waveform;
use redpitaya::Channel;

fn main() -> redpitaya::Result<()> {
    redpitaya::init()?;

    redpitaya::generator::set_freq(Channel::RP_CH_1, 10_000.0)?;

    redpitaya::generator::set_amp(Channel::RP_CH_1, 1.0)?;

    redpitaya::generator::set_waveform(Channel::RP_CH_1, Waveform::RP_WAVEFORM_SINE)?;

    redpitaya::generator::out_enable(Channel::RP_CH_1)?;

    redpitaya::release()
}
