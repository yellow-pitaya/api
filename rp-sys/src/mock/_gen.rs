struct GenState {
    offsets: Vec<f32>,
    out_enable: Vec<bool>,
    phases: Vec<f32>,
    waveforms: Vec<rp_waveform_t>,
    modes: Vec<rp_gen_mode_t>,
    trigger_src: Vec<rp_trig_src_t>,
    amp: Vec<f32>,
    burst_count: Vec<i32>,
    burst_period: Vec<u32>,
    burst_repetitions: Vec<i32>,
    duty_cycle: Vec<f32>,
    freq: Vec<f32>,
    burst_last_value: Vec<f32>,
}

impl Default for GenState
{
    fn default() -> Self
    {
        Self {
            offsets: vec![0.; 2],
            out_enable: vec![false; 2],
            phases: vec![0.; 2],
            waveforms: vec![rp_waveform_t::RP_WAVEFORM_SINE; 2],
            modes: vec![rp_gen_mode_t::RP_GEN_MODE_BURST; 2],
            trigger_src: vec![rp_trig_src_t::RP_GEN_TRIG_SRC_EXT_NE; 2],
            amp: vec![0.; 2],
            burst_count: vec![0; 2],
            burst_period: vec![0; 2],
            burst_repetitions: vec![0; 2],
            duty_cycle: vec![0.; 2],
            freq: vec![0.; 2],
            burst_last_value: vec![0.; 2],
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rp_gen_mode_t {
    RP_GEN_MODE_CONTINUOUS,
    RP_GEN_MODE_BURST,
    RP_GEN_MODE_STREAM,
}

pub unsafe fn rp_GenAmp(channel: rp_channel_t, amplitude: f32) -> c_int
{
    state!().r#gen.amp[channel as usize] = amplitude;

    ok!()
}

pub unsafe fn rp_GenArbWaveform(channel: rp_channel_t, waveform: *mut f32, length: u32) -> c_int
{
    todo!()
}

pub unsafe fn rp_GenBurstCount(channel: rp_channel_t, num: c_int) -> c_int
{
    state!().r#gen.burst_count[channel as usize] = num;

    ok!()
}

pub unsafe fn rp_GenBurstPeriod(channel: rp_channel_t, period: u32) -> c_int
{
    state!().r#gen.burst_period[channel as usize] = period;

    ok!()
}

pub unsafe fn rp_GenBurstRepetitions(channel: rp_channel_t, repetitions: c_int) -> c_int
{
    state!().r#gen.burst_repetitions[channel as usize] = repetitions;

    ok!()
}

pub unsafe fn rp_GenDutyCycle(channel: rp_channel_t, ratio: f32) -> c_int
{
    state!().r#gen.duty_cycle[channel as usize] = ratio;

    ok!()
}

pub unsafe fn rp_GenFreq(channel: rp_channel_t, frequency: f32) -> c_int
{
    state!().r#gen.freq[channel as usize] = frequency;

    ok!()
}

pub unsafe fn rp_GenGetAmp(channel: rp_channel_t, amplitude: *mut f32) -> c_int
{
    unsafe {
        *amplitude = state!().r#gen.amp[channel as usize];
    }

    ok!()
}

pub unsafe fn rp_GenGetArbWaveform(channel: rp_channel_t, waveform: *mut f32, length: *mut u32) -> c_int
{
    todo!()
}

pub unsafe fn rp_GenGetBurstCount(channel: rp_channel_t, num: *mut c_int) -> c_int
{
    unsafe {
        *num = state!().r#gen.burst_count[channel as usize];
    }

    ok!()
}

pub unsafe fn rp_GenGetBurstPeriod(channel: rp_channel_t, period: *mut u32) -> c_int
{
    unsafe {
        *period = state!().r#gen.burst_period[channel as usize];
    }

    ok!()
}

pub unsafe fn rp_GenGetBurstRepetitions(channel: rp_channel_t, repetitions: *mut c_int) -> c_int
{
    unsafe {
        *repetitions = state!().r#gen.burst_repetitions[channel as usize];
    }

    ok!()
}

pub unsafe fn rp_GenGetDutyCycle(channel: rp_channel_t, ratio: *mut f32) -> c_int
{
    unsafe {
        *ratio = state!().r#gen.duty_cycle[channel as usize];
    }

    ok!()
}

pub unsafe fn rp_GenGetFreq(channel: rp_channel_t, frequency: *mut f32) -> c_int
{
    unsafe {
        *frequency = state!().r#gen.freq[channel as usize];
    }

    ok!()
}

pub unsafe fn rp_GenGetMode(channel: rp_channel_t, mode: *mut rp_gen_mode_t) -> c_int
{
    unsafe {
        *mode = state!().r#gen.modes[channel as usize];
    }

    ok!()
}

pub unsafe fn rp_GenGetOffset(channel: rp_channel_t, offset: *mut f32) -> c_int
{
    unsafe {
        *offset = state!().r#gen.offsets[channel as usize];
    }

    ok!()
}

pub unsafe fn rp_GenGetPhase(channel: rp_channel_t, phase: *mut f32) -> c_int
{
    unsafe {
        *phase = state!().r#gen.phases[channel as usize];
    }

    ok!()
}

pub unsafe fn rp_GenGetTriggerSource(channel: rp_channel_t, src: *mut rp_trig_src_t) -> c_int
{
    unsafe {
        *src = state!().r#gen.trigger_src[channel as usize];
    }

    ok!()
}

pub unsafe fn rp_GenGetWaveform(channel: rp_channel_t, type_: *mut rp_waveform_t) -> c_int
{
    unsafe {
        *type_ = state!().r#gen.waveforms[channel as usize];
    }

    ok!()
}

pub unsafe fn rp_GenMode(channel: rp_channel_t, mode: rp_gen_mode_t) -> c_int
{
    state!().r#gen.modes[channel as usize] = mode;

    ok!()
}

pub unsafe fn rp_GenOffset(channel: rp_channel_t, offset: f32) -> c_int
{
    state!().r#gen.offsets[channel as usize] = offset;

    ok!()
}

pub unsafe fn rp_GenOutDisable(channel: rp_channel_t) -> c_int
{
    state!().r#gen.out_enable[channel as usize] = false;

    ok!()
}

pub unsafe fn rp_GenOutEnable(channel: rp_channel_t) -> c_int
{
    state!().r#gen.out_enable[channel as usize] = true;

    ok!()
}

pub unsafe fn rp_GenOutIsEnabled(channel: rp_channel_t, value: *mut bool) -> c_int
{
    unsafe {
        *value = state!().r#gen.out_enable[channel as usize];
    }

    ok!()
}

pub unsafe fn rp_GenPhase(channel: rp_channel_t, phase: f32) -> c_int
{
    state!().r#gen.phases[channel as usize] = phase;

    ok!()
}

pub unsafe fn rp_GenReset() -> c_int
{
    state!().r#gen = Default::default();

    ok!()
}

pub unsafe fn rp_GenTrigger(channel: u32) -> c_int
{
    ok!()
}

pub unsafe fn rp_GenTriggerSource(channel: rp_channel_t, src: rp_trig_src_t) -> c_int
{
    state!().r#gen.trigger_src[channel as usize] = src;

    ok!()
}

pub unsafe fn rp_GenWaveform(channel: rp_channel_t, type_: rp_waveform_t) -> c_int
{
    state!().r#gen.waveforms[channel as usize] = type_;

    ok!()
}

pub unsafe fn rp_GenGetBurstLastValue(channel: rp_channel_t, amplitude: *mut f32) -> c_int
{
    unsafe {
        *amplitude = state!().r#gen.burst_last_value[channel as usize];
    }

    ok!()
}

pub unsafe fn rp_GenBurstLastValue(channel: rp_channel_t, amplitude: f32) -> c_int
{
    state!().r#gen.burst_last_value[channel as usize] = amplitude;

    ok!()
}

pub unsafe fn rp_GenOutEnableSync(enable: bool) -> c_int
{
    ok!()
}

pub unsafe fn rp_GenSynchronise() -> c_int
{
    ok!()
}
