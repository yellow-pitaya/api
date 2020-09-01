pub use crate::rp::rp_gen_mode_t as Mode;
pub use crate::rp::rp_trig_src_t as TrigSrc;
pub use crate::rp::rp_waveform_t as Waveform;

/**
 * Sets generate to default values.
 */
pub fn reset() -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_GenReset()
    )
}

/**
 * Enables output.
 */
pub fn out_enable(channel: super::Channel) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_GenOutEnable(channel)
    )
}

/**
 * Disables output
 */
pub fn out_disable(channel: super::Channel) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_GenOutDisable(channel)
    )
}

/**
 * Gets value true if channel is enabled otherwise return false.
 */
pub fn out_is_enable(channel: super::Channel) -> crate::Result<bool>
{
    let mut value = false;

    match handle_unsafe!(
        crate::rp::rp_GenOutIsEnabled(channel, &mut value)
    ) {
        Ok(_) => Ok(value),
        Err(err) => Err(err),
    }
}

/**
 * Sets channel signal peak to peak amplitude.
 */
pub fn set_amp(channel: super::Channel, amplitude: f32) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_GenAmp(channel, amplitude)
    )
}

/**
 * Gets channel signal peak to peak amplitude.
 */
pub fn amp(channel: super::Channel) -> crate::Result<f32>
{
    let mut amplitude = 0.0;

    match handle_unsafe!(
        crate::rp::rp_GenGetAmp(channel, &mut amplitude)
    ) {
        Ok(_) => Ok(amplitude * 20.0),
        Err(err) => Err(err),
    }
}

/**
 * Sets DC offset of the signal.
 *
 * signal = signal + DC_offset.
 */
pub fn set_offset(channel: super::Channel, offset: f32) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_GenOffset(channel, offset)
    )
}

/**
 * Gets DC offset of the signal.
 */
pub fn offset(channel: super::Channel) -> crate::Result<f32>
{
    let mut offset = 0.0;

    match handle_unsafe!(
        crate::rp::rp_GenGetOffset(channel, &mut offset)
    ) {
        Ok(_) => Ok(offset * 20.0),
        Err(err) => Err(err),
    }
}

/**
 * Sets channel signal frequency.
 */
pub fn set_freq(channel: super::Channel, frequency: f32) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_GenFreq(channel, frequency)
    )
}

/**
 * Gets channel signal frequency.
 */
pub fn freq(channel: super::Channel) -> crate::Result<f32>
{
    let mut frequency = 0.0;

    match handle_unsafe!(
        crate::rp::rp_GenGetFreq(channel, &mut frequency)
    ) {
        Ok(_) => Ok(frequency),
        Err(err) => Err(err),
    }
}

/**
 * Sets channel signal phase.
 *
 * This shifts the signal in time.
 */
pub fn set_phase(channel: super::Channel, phase: f32) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_GenPhase(channel, phase)
    )
}

/**
 * Gets channel signal phase.
 */
pub fn phase(channel: super::Channel) -> crate::Result<f32>
{
    let mut phase = 0.0;

    match handle_unsafe!(
        crate::rp::rp_GenGetPhase(channel, &mut phase)
    ) {
        Ok(_) => Ok(phase),
        Err(err) => Err(err),
    }
}

/**
 * Sets channel signal waveform.
 *
 * This determines how the signal looks.
 */
pub fn set_waveform(channel: super::Channel, waveform: Waveform) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_GenWaveform(channel, waveform)
    )
}

/**
 * Gets channel signal waveform.
 */
pub fn waveform(channel: super::Channel) -> crate::Result<Waveform>
{
    let mut waveform = Waveform::RP_WAVEFORM_SINE;

    match handle_unsafe!(
        crate::rp::rp_GenGetWaveform(channel, &mut waveform)
    ) {
        Ok(_) => Ok(waveform),
        Err(err) => Err(err),
    }
}

/**
 * Sets user defined waveform.
 */
pub fn set_arb_waveform(channel: super::Channel, waveform: &mut [f32]) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_GenArbWaveform(channel, waveform.as_mut_ptr(), waveform.len() as u32)
    )
}

/**
 * Gets user defined waveform.
 */
pub fn arb_waveform(channel: super::Channel) -> crate::Result<Vec<f32>>
{
    let mut slice = [0.0; 16_384];
    let mut length = slice.len() as u32;

    match handle_unsafe!(
        crate::rp::rp_GenGetArbWaveform(channel, slice.as_mut_ptr(), &mut length)
    ) {
        Ok(_) => {
            let mut vec = slice.to_vec();
            vec.truncate(length as usize);

            Ok(vec)
        },
        Err(err) => Err(err),
    }
}

/**
 * Sets duty cycle of PWM signal.
 */
pub fn set_duty_cycle(channel: super::Channel, ratio: f32) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_GenDutyCycle(channel, ratio)
    )
}

/**
 * Gets duty cycle of PWM signal.
 */
pub fn duty_cycle(channel: super::Channel) -> crate::Result<f32>
{
    let mut ratio = 0.0;

    match handle_unsafe!(
        crate::rp::rp_GenGetDutyCycle(channel, &mut ratio)
    ) {
        Ok(_) => Ok(ratio),
        Err(err) => Err(err),
    }
}

/**
 * Sets generation mode.
 */
pub fn set_mode(channel: super::Channel, mode: Mode) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_GenMode(channel, mode)
    )
}

/**
 * Gets generation mode.
 */
pub fn mode(channel: super::Channel) -> crate::Result<Mode>
{
    let mut mode = Mode::RP_GEN_MODE_CONTINUOUS;

    match handle_unsafe!(
        crate::rp::rp_GenGetMode(channel, &mut mode)
    ) {
        Ok(_) => Ok(mode),
        Err(err) => Err(err),
    }
}

/**
 * Sets number of generated waveforms in a burst.
 */
pub fn set_burst_count(channel: super::Channel, num: i32) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_GenBurstCount(channel, num)
    )
}

/**
 * Gets number of generated waveforms in a burst.
 */
pub fn burst_count(channel: super::Channel) -> crate::Result<i32>
{
    let mut num = 0;

    match handle_unsafe!(
        crate::rp::rp_GenGetBurstCount(channel, &mut num)
    ) {
        Ok(_) => Ok(num),
        Err(err) => Err(err),
    }
}

/**
 * Sets number of burst repetitions.
 *
 * This determines how many bursts will be generated.
 */
pub fn set_burst_repetitions(channel: super::Channel, repetitions: i32) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_GenBurstRepetitions(channel, repetitions)
    )
}

/**
 * Gets number of burst repetitions.
 */
pub fn burst_repetitions(channel: super::Channel) -> crate::Result<i32>
{
    let mut repetitions = 0;

    match handle_unsafe!(
        crate::rp::rp_GenGetBurstRepetitions(channel, &mut repetitions)
    ) {
        Ok(_) => Ok(repetitions),
        Err(err) => Err(err),
    }
}

/**
 * Sets the time/period of one burst in micro seconds.
 *
 * Period must be equal or greater then the time of one burst.
 */
pub fn set_burst_period(channel: super::Channel, period: u32) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_GenBurstPeriod(channel, period)
    )
}

/**
 * Gets the period of one burst in micro seconds.
 */
pub fn burst_period(channel: super::Channel) -> crate::Result<u32>
{
    let mut period = 0;

    match handle_unsafe!(
        crate::rp::rp_GenGetBurstPeriod(channel, &mut period)
    ) {
        Ok(_) => Ok(period),
        Err(err) => Err(err),
    }
}

/**
 * Sets trigger source.
 */
pub fn set_trigger_source(channel: super::Channel, src: TrigSrc) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_GenTriggerSource(channel, src)
    )
}

/**
 * Gets trigger source.
 */
pub fn trigger_source(channel: super::Channel) -> crate::Result<TrigSrc>
{
    let mut src = TrigSrc::RP_GEN_TRIG_SRC_INTERNAL;

    match handle_unsafe!(
        crate::rp::rp_GenGetTriggerSource(channel, &mut src)
    ) {
        Ok(_) => Ok(src),
        Err(err) => Err(err),
    }
}

/**
 * Sets Trigger for specified channel/channels.
 */
pub fn set_trigger(channel: super::Channel) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_GenTrigger(channel as u32)
    )
}

/**
 * Sets the DAC protection mode from overheating.
 */
pub fn set_enable_temp_protection(channel: super::Channel, enable: bool) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_SetEnableTempProtection(channel, enable)
    )
}

/**
 * Get status of DAC protection mode from overheating.
 */
pub fn enable_temp_protection(channel: super::Channel) -> crate::Result<bool>
{
    let mut enable = false;

    match handle_unsafe!(
        crate::rp::rp_GetEnableTempProtection(channel, &mut enable)
    ) {
        Ok(_) => Ok(enable),
        Err(err) => Err(err),
    }
}

/**
 * Resets the flag indicating that the DAC is overheated.
 */
pub fn set_latch_temp_alarm(channel: super::Channel, status: bool) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_SetLatchTempAlarm(channel, status)
    )
}

/**
 * Returns the status that there was an overheat.
 */
pub fn latch_temp_alarm(channel: super::Channel) -> crate::Result<bool>
{
    let mut status = false;

    match handle_unsafe!(
        crate::rp::rp_GetLatchTempAlarm(channel, &mut status)
    ) {
        Ok(_) => Ok(status),
        Err(err) => Err(err),
    }
}

/**
 * Returns the current DAC overheat status in real time.
 */
pub fn runtime_temp_alarm(channel: super::Channel) -> crate::Result<bool>
{
    let mut status = false;

    match handle_unsafe!(
        crate::rp::rp_GetRuntimeTempAlarm(channel, &mut status)
    ) {
        Ok(_) => Ok(status),
        Err(err) => Err(err),
    }
}

pub fn pll_control_enable() -> crate::Result<bool>
{
    let mut enable = false;

    match handle_unsafe!(
        crate::rp::rp_GetPllControlEnable(&mut enable)
    ) {
        Ok(_) => Ok(enable),
        Err(err) => Err(err),
    }
}

pub fn set_pll_control_enable(enable: bool) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_SetPllControlEnable(enable)
    )
}

pub fn pll_control_locked() -> crate::Result<bool>
{
    let mut locked = false;

    match handle_unsafe!(
        crate::rp::rp_GetPllControlLocked(&mut locked)
    ) {
        Ok(_) => Ok(locked),
        Err(err) => Err(err),
    }
}
