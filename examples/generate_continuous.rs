extern crate redpitaya;

use redpitaya::Channel;
use redpitaya::generator::Waveform;

fn main() {
    redpitaya::init()
        .expect("Rp api init failed!");

    redpitaya::generator::freq(Channel::RP_CH_1, 10_000.0)
        .unwrap();

    redpitaya::generator::amp(Channel::RP_CH_1, 1.0)
        .unwrap();

    redpitaya::generator::waveform(Channel::RP_CH_1, Waveform::RP_WAVEFORM_SINE)
        .unwrap();

    redpitaya::generator::out_enable(Channel::RP_CH_1)
        .unwrap();

    redpitaya::release()
        .unwrap();
}
