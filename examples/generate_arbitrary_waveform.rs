use redpitaya::generator::Waveform;
use redpitaya::Channel;

fn main() -> redpitaya::Result {
    let mut t = [0.0; 16_384];
    let mut x = [0.0; 16_384];
    let mut y = [0.0; 16_384];

    redpitaya::init().expect("Red Pitaya API init failed!");

    for i in 1..t.len() {
        t[i] = 2.0 * std::f32::consts::PI / t.len() as f32 * i as f32;
    }

    for i in 0..x.len() {
        x[i] = t[i].sin() + (1.0 / 3.0 * (t[i] * 3.0).sin());
        y[i] = 1.0 / 2.0 * t[i].sin() + (1.0 / 4.0) * (t[i] * 4.0).sin();
    }

    redpitaya::generator::set_waveform(Channel::RP_CH_1, Waveform::RP_WAVEFORM_ARBITRARY)?;
    redpitaya::generator::set_waveform(Channel::RP_CH_2, Waveform::RP_WAVEFORM_ARBITRARY)?;

    redpitaya::generator::set_arb_waveform(Channel::RP_CH_1, &mut x)?;
    redpitaya::generator::set_arb_waveform(Channel::RP_CH_2, &mut y)?;

    redpitaya::generator::set_amp(Channel::RP_CH_1, 0.7)?;
    redpitaya::generator::set_amp(Channel::RP_CH_2, 1.0)?;

    redpitaya::generator::set_freq(Channel::RP_CH_1, 4_000.0)?;
    redpitaya::generator::set_freq(Channel::RP_CH_2, 4_000.0)?;

    redpitaya::generator::out_enable(Channel::RP_CH_1)?;
    redpitaya::generator::out_enable(Channel::RP_CH_2)?;

    redpitaya::release()
}
