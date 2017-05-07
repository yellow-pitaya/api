extern crate rp_sys;

use std::ptr;

/**
 * Type representing decimation used at acquiring signal.
 */
pub use rp_sys::rp_acq_decimation_t as Decimation;

/**
 * Type representing acquire signal sampling rate.
 */
pub use rp_sys::rp_acq_sampling_rate_t as SamplingRate;

/**
 * Type representing different trigger sources used at acquiring signal.
 */
pub use rp_sys::rp_acq_trig_src_t as TrigSrc;

/**
 * Type representing different trigger states.
 */
pub use rp_sys::rp_acq_trig_state_t as TrigState;

/**
 * Enables continous acquirement even after trigger has happened.
 */
pub fn set_arm_keep(enable: bool) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_AcqSetArmKeep(enable)
    )
}

/**
 * Sets the decimation used at acquiring signal.
 *
 * There is only a set of pre-defined decimation
 *
 * values which can be specified. See the `Decimation` enum values.
 */
pub fn set_decimation(decimat: Decimation) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_AcqSetDecimation(decimat)
    )
}

/**
 * Gets the decimation used at acquiring signal.
 *
 * There is only a set of pre-defined decimation values which can be specified.
 * See the `Decimation` enum values.
 */
pub fn get_decimation() -> Result<Decimation, String>
{
    let mut decimat = Decimation::RP_DEC_1;

    match handle_unsafe!(
        rp_sys::rp_AcqGetDecimation(&mut decimat)
    ) {
        Ok(_) => Ok(decimat),
        Err(err) => Err(err),
    }
}

/**
 * Gets the decimation factor used at acquiring signal in a numerical form.
 *
 * Although this method returns an integer value representing the current factor
 * of the decimation, there is only a set of pre-defined decimation factor
 * values which can be returned. See the `Decimation` enum values.
 */
pub fn get_decimation_factor() -> Result<u32, String>
{
    let mut decimation = 0;

    match handle_unsafe!(
        rp_sys::rp_AcqGetDecimationFactor(&mut decimation)
    ) {
        Ok(_) => Ok(decimation),
        Err(err) => Err(err),
    }
}

/**
 * Sets the sampling rate for acquiring signal.
 *
 * There is only a set of pre-defined sampling rate values which can be
 * specified. See the `SamplingRate` enum values.
 */
pub fn set_sampling_rate(sampling_rate: SamplingRate ) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_AcqSetSamplingRate(sampling_rate)
    )
}

/**
 * Gets the sampling rate for acquiring signal.
 *
 * There is only a set of pre-defined sampling rate values which can be
 * returned. See the `SamplingRate` enum values.
 */
pub fn get_sampling_rate() -> Result<SamplingRate, String>
{
    let mut sampling_rate = SamplingRate::RP_SMP_125M;

    match handle_unsafe!(
        rp_sys::rp_AcqGetSamplingRate(&mut sampling_rate)
    ) {
        Ok(_) => Ok(sampling_rate),
        Err(err) => Err(err),
    }
}

/**
 * Gets the sampling rate for acquiring signal in a numerical form in Hz.
 *
 * Although this method returns a float value representing the current value of
 * the sampling rate, there is only a set of pre-defined sampling rate values
 * which can be returned. See the `SamplingRate` enum values.
 */
pub fn get_sampling_rate_hz() -> Result<f32, String>
{
    let mut sampling_rate = 0.0;

    match handle_unsafe!(
        rp_sys::rp_AcqGetSamplingRateHz(&mut sampling_rate)
    ) {
        Ok(_) => Ok(sampling_rate),
        Err(err) => Err(err),
    }
}

/**
 * Enables or disables averaging of data between samples.
 *
 * Data between samples can be averaged by setting the averaging flag in the
 * Data decimation register.
 */
pub fn set_averaging(enabled: bool) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_AcqSetAveraging(enabled)
    )
}

/**
 * Returns information if averaging of data between samples is enabled or
 * disabled.
 *
 * Data between samples can be averaged by setting the averaging flag in the
 * Data decimation register.
 */
pub fn get_averaging() -> Result<bool, String>
{
    let mut enabled = false;

    match handle_unsafe!(
        rp_sys::rp_AcqGetAveraging(&mut enabled)
    ) {
        Ok(_) => Ok(enabled),
        Err(err) => Err(err),
    }
}

/**
 * Sets the trigger source used at acquiring signal.
 *
 * When acquiring is started, the FPGA waits for the trigger condition on the
 * specified source and when the condition is met, it starts writing the signal
 * to the buffer.
 */
pub fn set_trigger_src(source: TrigSrc) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_AcqSetTriggerSrc(source)
    )
}

/**
 * Gets the trigger source used at acquiring signal.
 *
 * When acquiring is started, the FPGA waits for the trigger condition on the
 * specified source and when the condition is met, it starts writing the signal
 * to the buffer.
 */
pub fn get_trigger_src() -> Result<TrigSrc, String>
{
    let mut source = TrigSrc::RP_TRIG_SRC_DISABLED;

    match handle_unsafe!(
        rp_sys::rp_AcqGetTriggerSrc(&mut source)
    ) {
        Ok(_) => Ok(source),
        Err(err) => Err(err),
    }
}

/**
 * Returns the trigger state.
 *
 * Either it is waiting for a trigger to happen, or it has already been
 * triggered.
 *
 * By default it is in the triggered state, which is treated the same as
 * disabled.
 */
pub fn get_trigger_state() -> Result<TrigState, String>
{
    let mut state = TrigState::RP_TRIG_STATE_TRIGGERED;

    match handle_unsafe!(
        rp_sys::rp_AcqGetTriggerState(&mut state)
    ) {
        Ok(_) => Ok(state),
        Err(err) => Err(err),
    }
}

/**
 * Sets the number of decimated data after trigger written into memory.
 */
pub fn set_trigger_delay(decimated_data_num: i32) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_AcqSetTriggerDelay(decimated_data_num)
    )
}

/**
 * Returns current number of decimated data after trigger written into memory.
 */
pub fn get_trigger_delay() -> Result<i32, String>
{
    let mut decimated_data_num = 0;

    match handle_unsafe!(
        rp_sys::rp_AcqGetTriggerDelay(&mut decimated_data_num)
    ) {
        Ok(_) => Ok(decimated_data_num),
        Err(err) => Err(err),
    }
}

/**
 * Sets the amount of decimated data in nanoseconds after trigger written into
 * memory.
 */
pub fn set_trigger_delay_ns(time_ns: i64) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_AcqSetTriggerDelayNs(time_ns)
    )
}

/**
 * Returns the current amount of decimated data in nanoseconds after trigger
 * written into memory.
 */
pub fn get_trigger_delay_ns() -> Result<i64, String>
{
    let mut time_ns = 0;

    match handle_unsafe!(
        rp_sys::rp_AcqGetTriggerDelayNs(&mut time_ns)
    ) {
        Ok(_) => Ok(time_ns),
        Err(err) => Err(err),
    }
}

/**
 * Returns the number of valid data ponts before trigger.
 */
pub fn get_pre_trigger_counter() -> Result<u32, String>
{
    let mut value = 0;

    match handle_unsafe!(
        rp_sys::rp_AcqGetPreTriggerCounter(&mut value)
    ) {
        Ok(_) => Ok(value),
        Err(err) => Err(err),
    }
}

/**
 * Sets the trigger threshold value in volts.
 *
 * Makes the trigger when ADC value crosses this value.
 */
pub fn set_trigger_level(channel: super::Channel, volatage: f32) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_AcqSetTriggerLevel(channel, volatage)
    )
}

/**
 * Gets currently set trigger threshold value in volts.
 */
pub fn get_trigger_level() -> Result<f32, String>
{
    let mut volatage = 0.0;

    match handle_unsafe!(
        rp_sys::rp_AcqGetTriggerLevel(&mut volatage)
    ) {
        Ok(_) => Ok(volatage),
        Err(err) => Err(err),
    }
}

/**
 * Sets the trigger threshold hysteresis value in volts.
 *
 * Value must be outside to enable the trigger again.
 */
pub fn set_trigger_hyst(volatage: f32) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_AcqSetTriggerHyst(volatage)
    )
}

/**
 * Gets currently set trigger threshold hysteresis value in volts.
 */
pub fn get_trigger_hyst() -> Result<f32, String>
{
    let mut volatage = 0.0;

    match handle_unsafe!(
        rp_sys::rp_AcqGetTriggerHyst(&mut volatage)
    ) {
        Ok(_) => Ok(volatage),
        Err(err) => Err(err),
    }
}

/**
 * Sets the acquire gain state.
 *
 * The gain should be set to the same value as it is set on the Red Pitaya
 * hardware by the LV/HV gain jumpers. LV = 1V; HV = 20V.
 */
pub fn set_gain(channel: super::Channel, state: super::pin::State) -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_AcqSetGain(channel, state)
    )
}

/**
 * Returns the currently set acquire gain state in the library.
 *
 * It may not be set to the same value as it is set on the Red Pitaya hardware
 * by the LV/HV gain jumpers. LV = 1V; HV = 20V.
 */
pub fn get_gain(channel: super::Channel) -> Result<super::pin::State, String>
{
    let mut state = super::pin::State::RP_LOW;

    match handle_unsafe!(
        rp_sys::rp_AcqGetGain(channel, &mut state)
    ) {
        Ok(_) => Ok(state),
        Err(err) => Err(err),
    }
}

/**
 * Returns the currently set acquire gain in the library.
 *
 * It may not be set to the same value as it is set on the Red Pitaya hardware
 * by the LV/HV gain jumpers. Returns value in Volts.
 */
pub fn get_gain_v(channel: super::Channel) -> Result<f32, String>
{
    let mut voltage = 0.0;

    match handle_unsafe!(
        rp_sys::rp_AcqGetGainV(channel, &mut voltage)
    ) {
        Ok(_) => Ok(voltage),
        Err(err) => Err(err),
    }
}

/**
 * Returns current position of ADC write pointer.
 */
pub fn get_write_pointer() -> Result<u32, String>
{
    let mut pos = 0;

    match handle_unsafe!(
        rp_sys::rp_AcqGetWritePointer(&mut pos)
    ) {
        Ok(_) => Ok(pos),
        Err(err) => Err(err),
    }
}

/**
 * Returns position of ADC write pointer at time when trigger arrived.
 */
pub fn get_write_pointer_at_trig() -> Result<u32, String>
{
    let mut pos = 0;

    match handle_unsafe!(
        rp_sys::rp_AcqGetWritePointerAtTrig(&mut pos)
    ) {
        Ok(_) => Ok(pos),
        Err(err) => Err(err),
    }
}

/**
 * Starts the acquire.
 *
 * Signals coming from the input channels are acquired and written into memory.
 */
pub fn start() -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_AcqStart()
    )
}

/**
 * Stops the acquire.
 */
pub fn stop() -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_AcqStop()
    )
}

/**
 * Resets the acquire writing state machine.
 */
pub fn reset() -> Result<(), String>
{
    handle_unsafe!(
        rp_sys::rp_AcqReset()
    )
}

/**
 * Normalizes the ADC buffer position.
 *
 * Returns the modulo operation of ADC buffer size...
 */
pub fn get_normalized_data_pos(pos: u32) -> u32
{
    unsafe {
        rp_sys::rp_AcqGetNormalizedDataPos(pos)
    }
}

/**
 * Returns the ADC buffer in raw units from start to end position.
 */
pub fn get_data_pos_raw(channel: super::Channel, start_pos: u32, end_pos: u32) -> Result<Vec<i16>, String>
{
    let mut length = 0;
    let buffer = ptr::null_mut();

    match handle_unsafe!(
        rp_sys::rp_AcqGetDataPosRaw(channel, start_pos, end_pos, buffer, &mut length)
    ) {
        Ok(_) => Ok(unsafe { Vec::from_raw_parts(buffer, length as usize, length as usize) }),
        Err(err) => Err(err),
    }
}

/**
 * Returns the ADC buffer in Volt units from start to end position.
 */
pub fn get_data_pos_v(channel: super::Channel, start_pos: u32, end_pos: u32) -> Result<Vec<f32>, String>
{
    let mut length = 0;
    let buffer = ptr::null_mut();

    match handle_unsafe!(
        rp_sys::rp_AcqGetDataPosV(channel, start_pos, end_pos, buffer, &mut length)
    ) {
        Ok(_) => Ok(unsafe { Vec::from_raw_parts(buffer, length as usize, length as usize) }),
        Err(err) => Err(err),
    }
}

/**
 * Returns the ADC buffer in raw units from specified position.
 */
pub fn get_data_raw(channel: super::Channel, pos: u32, size: u32) -> Result<Vec<i16>, String>
{
    let mut length = size;
    let buffer = ptr::null_mut();

    match handle_unsafe!(
        rp_sys::rp_AcqGetDataRaw(channel, pos, &mut length, buffer)
    ) {
        Ok(_) => Ok(unsafe { Vec::from_raw_parts(buffer, length as usize, length as usize) }),
        Err(err) => Err(err),
    }
}

/**
 * Returns the ADC buffer in raw units from specified position.
 */
pub fn get_data_raw_v2(pos: u32, size: u32) -> Result<[Vec<u16>;2], String>
{
    let mut length = size;
    let buffer1 = ptr::null_mut();
    let buffer2 = ptr::null_mut();

    match handle_unsafe!(
        rp_sys::rp_AcqGetDataRawV2(pos, &mut length, buffer1, buffer2)
    ) {
        Ok(_) => Ok(unsafe {[
            Vec::from_raw_parts(buffer1, length as usize, length as usize),
            Vec::from_raw_parts(buffer2, length as usize, length as usize)
        ]}),
        Err(err) => Err(err),
    }
}

/**
 * Returns the ADC buffer in raw units from the oldest sample to the newest one.
 *
 * CAUTION: Use this method only when write pointer has stopped (Trigger
 * happened and writing stopped).
 */
pub fn get_oldest_data_raw(channel: super::Channel, size: u32) -> Result<Vec<i16>, String>
{
    let mut length = size;
    let buffer = ptr::null_mut();

    match handle_unsafe!(
        rp_sys::rp_AcqGetOldestDataRaw(channel, &mut length, buffer)
    ) {
        Ok(_) => Ok(unsafe { Vec::from_raw_parts(buffer, length as usize, length as usize) }),
        Err(err) => Err(err),
    }
}

/**
 * Returns the latest ADC buffer samples in raw units.
 */
pub fn get_latest_data_raw(channel: super::Channel, size: u32) -> Result<Vec<i16>, String>
{
    let mut length = size;
    let buffer = ptr::null_mut();

    match handle_unsafe!(
        rp_sys::rp_AcqGetLatestDataRaw(channel, &mut length, buffer)
    ) {
        Ok(_) => Ok(unsafe { Vec::from_raw_parts(buffer, length as usize, length as usize) }),
        Err(err) => Err(err),
    }
}

/**
 * Returns the ADC buffer in Volt units from specified position.
 */
pub fn get_data_v(channel: super::Channel, pos: u32, size: u32) -> Result<Vec<f32>, String>
{
    let mut length = size;
    let buffer = ptr::null_mut();

    match handle_unsafe!(
        rp_sys::rp_AcqGetDataV(channel, pos, &mut length, buffer)
    ) {
        Ok(_) => Ok(unsafe { Vec::from_raw_parts(buffer, length as usize, length as usize) }),
        Err(err) => Err(err),
    }
}

/**
 * Returns the ADC buffer in Volt units from specified position.
 */
pub fn get_data_v2(pos: u32, size: u32) -> Result<[Vec<f32>;2], String>
{
    let mut length = size;
    let buffer1 = ptr::null_mut();
    let buffer2 = ptr::null_mut();

    match handle_unsafe!(
        rp_sys::rp_AcqGetDataV2(pos, &mut length, buffer1, buffer2)
    ) {
        Ok(_) => Ok(unsafe {[
            Vec::from_raw_parts(buffer1, length as usize, length as usize),
            Vec::from_raw_parts(buffer2, length as usize, length as usize)
        ]}),
        Err(err) => Err(err),
    }
}

/**
 * Returns the ADC buffer in Volt units from the oldest sample to the newest one.
 *
 * CAUTION: Use this method only when write pointer has stopped (Trigger happened and writing stopped).
 */
pub fn get_oldest_data_v(channel: super::Channel, size: u32) -> Result<Vec<f32>, String>
{
    let mut length = size;
    let buffer = ptr::null_mut();

    match handle_unsafe!(
        rp_sys::rp_AcqGetOldestDataV(channel, &mut length, buffer)
    ) {
        Ok(_) => Ok(unsafe { Vec::from_raw_parts(buffer, length as usize, length as usize) }),
        Err(err) => Err(err),
    }
}

/**
 * Returns the latest ADC buffer samples in Volt units.
 */
pub fn get_latest_data_v(channel: super::Channel, size: u32) -> Result<Vec<f32>, String>
{
    let mut length = size;
    let buffer = ptr::null_mut();

    match handle_unsafe!(
        rp_sys::rp_AcqGetLatestDataV(channel, &mut length, buffer)
    ) {
        Ok(_) => Ok(unsafe { Vec::from_raw_parts(buffer, length as usize, length as usize) }),
        Err(err) => Err(err),
    }
}

pub fn get_buffer_size() -> Result<u32, String>
{
    let mut size = 0;

    match handle_unsafe!(
        rp_sys::rp_AcqGetBufSize(&mut size)
    ) {
        Ok(_) => Ok(size),
        Err(err) => Err(err),
    }
}
