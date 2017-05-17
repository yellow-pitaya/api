extern crate redpitaya;

use redpitaya::Channel;
use redpitaya::generator::{
    Mode,
    TrigSrc,
    Waveform,
};

fn main() {
    redpitaya::init()
        .expect("Red Pitaya API init failed!");

    redpitaya::generator::waveform(Channel::RP_CH_1, Waveform::RP_WAVEFORM_SINE).unwrap();
    redpitaya::generator::freq(Channel::RP_CH_1, 200.0).unwrap();
    redpitaya::generator::amp(Channel::RP_CH_1, 1.0).unwrap();

    redpitaya::generator::burst_count(Channel::RP_CH_1, 1).unwrap();

    redpitaya::generator::out_enable(Channel::RP_CH_1).unwrap();
    redpitaya::generator::mode(Channel::RP_CH_1, Mode::RP_GEN_MODE_BURST).unwrap();
    redpitaya::generator::trigger_source(Channel::RP_CH_1, TrigSrc::RP_GEN_TRIG_SRC_EXT_PE).unwrap();

    redpitaya::release()
        .unwrap();
}
