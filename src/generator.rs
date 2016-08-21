extern crate rp_sys;

pub use rp_sys::rp_gen_mode_t as Mode;
pub use rp_sys::rp_trig_src_t as TrigSrc;
pub use rp_sys::rp_waveform_t as Waveform;

use std::ptr;

pub fn reset() -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_GenReset()
    )
}

pub fn out_enable(channel: super::Channel) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_GenOutEnable(channel)
    )
}

pub fn out_disable(channel: super::Channel) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_GenOutDisable(channel)
    )
}

pub fn out_is_enable(channel: super::Channel) -> Result<bool, String>
{
    let mut value = 0;

    match handle_unsafe!(
        rp_sys::rp_GenOutIsEnabled(channel, &mut value)
    ) {
        Ok(_) => Ok(value != 0),
        Err(err) => Err(err),
    }
}

pub fn amp(channel: super::Channel, amplitude: f32) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_GenAmp(channel, amplitude)
    )
}

pub fn get_amp(channel: super::Channel) -> Result<f32, String>
{
    let mut amplitude = 0.0;

    match handle_unsafe!(
        rp_sys::rp_GenGetAmp(channel, &mut amplitude)
    ) {
        Ok(_) => Ok(amplitude),
        Err(err) => Err(err),
    }
}

pub fn offset(channel: super::Channel, offset: f32) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_GenOffset(channel, offset)
    )
}

pub fn get_offset(channel: super::Channel) -> Result<f32, String>
{
    let mut offset = 0.0;

    match handle_unsafe!(
        rp_sys::rp_GenGetOffset(channel, &mut offset)
    ) {
        Ok(_) => Ok(offset),
        Err(err) => Err(err),
    }
}

pub fn freq(channel: super::Channel, frequency: f32) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_GenFreq(channel, frequency)
    )
}

pub fn get_freq(channel: super::Channel) -> Result<f32, String>
{
    let mut frequency = 0.0;

    match handle_unsafe!(
        rp_sys::rp_GenGetFreq(channel, &mut frequency)
    ) {
        Ok(_) => Ok(frequency),
        Err(err) => Err(err),
    }
}

pub fn phase(channel: super::Channel, phase: f32) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_GenPhase(channel, phase)
    )
}

pub fn get_phase(channel: super::Channel) -> Result<f32, String>
{
    let mut phase = 0.0;

    match handle_unsafe!(
        rp_sys::rp_GenGetPhase(channel, &mut phase)
    ) {
        Ok(_) => Ok(phase),
        Err(err) => Err(err),
    }
}

pub fn waveform(channel: super::Channel, waveform: Waveform) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_GenWaveform(channel, waveform)
    )
}

pub fn get_waveform(channel: super::Channel) -> Result<Waveform, String>
{
    let mut waveform = Waveform::RP_WAVEFORM_SINE;

    match handle_unsafe!(
        rp_sys::rp_GenGetWaveform(channel, &mut waveform)
    ) {
        Ok(_) => Ok(waveform),
        Err(err) => Err(err),
    }
}

pub fn arb_waveform(channel: super::Channel, waveform: &mut [f32]) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_GenArbWaveform(channel, waveform.as_mut_ptr(), waveform.len() as u32)
    )
}

pub fn get_arb_waveform(channel: super::Channel) -> Result<Vec<f32>, String>
{
    let mut length = 0;
    let buffer = ptr::null_mut();

    match handle_unsafe!(
        rp_sys::rp_GenGetArbWaveform(channel, buffer, &mut length)
    ) {
        Ok(_) => Ok(unsafe { Vec::from_raw_parts(buffer, length as usize, length as usize) }),
        Err(err) => Err(err),
    }
}

pub fn duty_cycle(channel: super::Channel, ratio: f32) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_GenDutyCycle(channel, ratio)
    )
}

pub fn get_duty_cycle(channel: super::Channel) -> Result<f32, String>
{
    let mut ratio = 0.0;

    match handle_unsafe!(
        rp_sys::rp_GenGetDutyCycle(channel, &mut ratio)
    ) {
        Ok(_) => Ok(ratio),
        Err(err) => Err(err),
    }
}

pub fn mode(channel: super::Channel, mode: Mode) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_GenMode(channel, mode)
    )
}

pub fn get_mode(channel: super::Channel) -> Result<Mode, String>
{
    let mut mode = Mode::RP_GEN_MODE_CONTINUOUS;

    match handle_unsafe!(
        rp_sys::rp_GenGetMode(channel, &mut mode)
    ) {
        Ok(_) => Ok(mode),
        Err(err) => Err(err),
    }
}

pub fn burst_count(channel: super::Channel, num: i32) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_GenBurstCount(channel, num)
    )
}

pub fn get_burst_count(channel: super::Channel) -> Result<i32, String>
{
    let mut num = 0;

    match handle_unsafe!(
        rp_sys::rp_GenGetBurstCount(channel, &mut num)
    ) {
        Ok(_) => Ok(num),
        Err(err) => Err(err),
    }
}

pub fn burst_repetitions(channel: super::Channel, repetitions: i32) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_GenBurstRepetitions(channel, repetitions)
    )
}

pub fn get_burst_repetitions(channel: super::Channel) -> Result<i32, String>
{
    let mut repetitions = 0;

    match handle_unsafe!(
        rp_sys::rp_GenGetBurstRepetitions(channel, &mut repetitions)
    ) {
        Ok(_) => Ok(repetitions),
        Err(err) => Err(err),
    }
}

pub fn burst_period(channel: super::Channel, period: u32) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_GenBurstPeriod(channel, period)
    )
}

pub fn get_burst_period(channel: super::Channel) -> Result<u32, String>
{
    let mut period = 0;

    match handle_unsafe!(
        rp_sys::rp_GenGetBurstPeriod(channel, &mut period)
    ) {
        Ok(_) => Ok(period),
        Err(err) => Err(err),
    }
}

pub fn trigger_source(channel: super::Channel, src: TrigSrc) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_GenTriggerSource(channel, src)
    )
}

pub fn get_trigger_source(channel: super::Channel) -> Result<TrigSrc, String>
{
    let mut src = TrigSrc::RP_GEN_TRIG_SRC_INTERNAL;

    match handle_unsafe!(
        rp_sys::rp_GenGetTriggerSource(channel, &mut src)
    ) {
        Ok(_) => Ok(src),
        Err(err) => Err(err),
    }
}

pub fn trigger(channel: u32) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_GenTrigger(channel)
    )
}
