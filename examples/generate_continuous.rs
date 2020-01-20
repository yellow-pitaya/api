use redpitaya::Channel;
use redpitaya::generator::Waveform;

fn main() -> Result<(), String> {
    redpitaya::init()?;

    redpitaya::generator::freq(Channel::RP_CH_1, 10_000.0)?;

    redpitaya::generator::amp(Channel::RP_CH_1, 1.0)?;

    redpitaya::generator::waveform(Channel::RP_CH_1, Waveform::RP_WAVEFORM_SINE)?;

    redpitaya::generator::out_enable(Channel::RP_CH_1)?;

    redpitaya::release()
}
